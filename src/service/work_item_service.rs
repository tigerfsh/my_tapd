use std::sync::Arc;

use crate::domain::{
    AuditLog, Attachment, Comment, CreateBugRequest, CreateRequirementRequest, CreateStoryRequest,
    CreateTaskRequest, FileUpload, ProjectId, Role, Status, UserId, WorkItem, WorkItemFilter,
    WorkItemId, WorkItemType,
};
use crate::domain::functions::{
    calc_completion_pct, is_valid_bug_transition, is_valid_hours, validate_attachment_size,
};
use crate::error::AppError;
use crate::repository::audit_repo::AuditRepo;
use crate::repository::project_repo::ProjectRepo;
use crate::repository::work_item_repo::WorkItemRepo;

pub struct WorkItemService {
    work_item_repo: Arc<WorkItemRepo>,
    audit_repo: Arc<AuditRepo>,
    project_repo: Arc<ProjectRepo>,
}

impl WorkItemService {
    pub fn new(
        work_item_repo: Arc<WorkItemRepo>,
        audit_repo: Arc<AuditRepo>,
        project_repo: Arc<ProjectRepo>,
    ) -> Self {
        Self {
            work_item_repo,
            audit_repo,
            project_repo,
        }
    }

    // ---- Task 8.1: create_requirement ----

    /// 需求 3.1、3.2：创建需求，初始状态 Pending，记录创建人和创建时间
    pub async fn create_requirement(
        &self,
        creator_id: UserId,
        project_id: ProjectId,
        req: CreateRequirementRequest,
    ) -> Result<WorkItem, AppError> {
        let number = self
            .work_item_repo
            .get_next_number(project_id, "REQ")
            .await?;

        let item = self
            .work_item_repo
            .create(
                project_id,
                WorkItemType::Requirement,
                &number,
                &req.title,
                req.description.as_deref(),
                req.priority,
                req.assignee_id,
                creator_id,
                None,
                req.due_date,
                None,
                None,
                None,
            )
            .await?;

        for label in &req.labels {
            self.work_item_repo.add_label(item.id, label).await?;
        }

        self.audit_repo
            .create(item.id, creator_id, "created", None, Some(&item.title))
            .await?;

        Ok(item)
    }

    // ---- Task 8.2: create_story, create_task ----

    /// 需求 3.3：创建故事，关联父需求
    pub async fn create_story(
        &self,
        creator_id: UserId,
        parent_req_id: WorkItemId,
        req: CreateStoryRequest,
    ) -> Result<WorkItem, AppError> {
        let parent = self
            .work_item_repo
            .find_by_id(parent_req_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let project_id = parent.project_id;
        let number = self
            .work_item_repo
            .get_next_number(project_id, "STORY")
            .await?;

        let item = self
            .work_item_repo
            .create(
                project_id,
                WorkItemType::Story,
                &number,
                &req.title,
                req.description.as_deref(),
                crate::domain::Priority::Medium,
                req.assignee_id,
                creator_id,
                Some(parent_req_id),
                None,
                req.story_points,
                None,
                None,
            )
            .await?;

        self.audit_repo
            .create(item.id, creator_id, "created", None, Some(&item.title))
            .await?;

        Ok(item)
    }

    /// 需求 4.1、4.2：创建任务，关联父故事
    pub async fn create_task(
        &self,
        creator_id: UserId,
        parent_story_id: WorkItemId,
        req: CreateTaskRequest,
    ) -> Result<WorkItem, AppError> {
        let parent = self
            .work_item_repo
            .find_by_id(parent_story_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let project_id = parent.project_id;
        let number = self
            .work_item_repo
            .get_next_number(project_id, "TASK")
            .await?;

        let item = self
            .work_item_repo
            .create(
                project_id,
                WorkItemType::Task,
                &number,
                &req.title,
                None,
                crate::domain::Priority::Medium,
                req.assignee_id,
                creator_id,
                Some(parent_story_id),
                None,
                None,
                req.estimated_hours,
                None,
            )
            .await?;

        self.audit_repo
            .create(item.id, creator_id, "created", None, Some(&item.title))
            .await?;

        Ok(item)
    }

    // ---- Task 8.3: update_status ----

    /// 需求 3.4、4.3、4.4、4.7、6.3：更新状态，含 Bug 状态机校验、Task 完成级联、权限校验
    pub async fn update_status(
        &self,
        operator_id: UserId,
        item_id: WorkItemId,
        new_status: Status,
    ) -> Result<WorkItem, AppError> {
        let item = self
            .work_item_repo
            .find_by_id(item_id)
            .await?
            .ok_or(AppError::NotFound)?;

        // Bug: validate state machine transition
        if item.item_type == WorkItemType::Bug {
            if !is_valid_bug_transition(&item.status, &new_status) {
                return Err(AppError::InvalidStatusTransition {
                    from: item.status.clone(),
                    to: new_status,
                });
            }
        }

        // Task: permission check — only assignee or Admin may update
        if item.item_type == WorkItemType::Task {
            if item.assignee_id != Some(operator_id) {
                let member = self
                    .project_repo
                    .get_member(item.project_id, operator_id)
                    .await?;
                let is_admin = matches!(member, Some(m) if m.role == Role::Admin);
                if !is_admin {
                    return Err(AppError::Forbidden);
                }
            }
        }

        let old_status_str = format!("{:?}", item.status);
        let new_status_str = format!("{:?}", new_status);

        let updated = self
            .work_item_repo
            .update_status(item_id, new_status.clone())
            .await?;

        self.audit_repo
            .create(
                item_id,
                operator_id,
                "status",
                Some(&old_status_str),
                Some(&new_status_str),
            )
            .await?;

        // Task done: cascade completion_pct and status up the hierarchy
        if item.item_type == WorkItemType::Task && new_status == Status::Done {
            if let Some(parent_story_id) = item.parent_id {
                self.recalc_story_completion(parent_story_id, operator_id)
                    .await?;
            }
        }

        Ok(updated)
    }

    /// Recalculate story completion_pct and cascade to parent requirement if needed.
    async fn recalc_story_completion(
        &self,
        story_id: WorkItemId,
        operator_id: UserId,
    ) -> Result<(), AppError> {
        let tasks = self.work_item_repo.list_children(story_id).await?;
        let pct = calc_completion_pct(&tasks);
        self.work_item_repo
            .update_completion_pct(story_id, pct)
            .await?;

        let all_done = !tasks.is_empty() && tasks.iter().all(|t| t.status == Status::Done);
        if all_done {
            self.work_item_repo
                .update_status(story_id, Status::Done)
                .await?;
            self.audit_repo
                .create(story_id, operator_id, "status", Some("InProgress"), Some("Done"))
                .await?;

            // Cascade to parent requirement
            let story = self
                .work_item_repo
                .find_by_id(story_id)
                .await?;
            if let Some(story) = story {
                if let Some(req_id) = story.parent_id {
                    let stories = self.work_item_repo.list_children(req_id).await?;
                    let all_stories_done = !stories.is_empty()
                        && stories.iter().all(|s| s.status == Status::Done);
                    if all_stories_done {
                        self.work_item_repo
                            .update_status(req_id, Status::Done)
                            .await?;
                        self.audit_repo
                            .create(req_id, operator_id, "status", Some("InProgress"), Some("Done"))
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }

    // ---- Task 8.4: create_bug, assign, log_actual_hours ----

    /// 需求 6.1、6.2：创建 Bug，初始状态 Pending
    pub async fn create_bug(
        &self,
        creator_id: UserId,
        project_id: ProjectId,
        req: CreateBugRequest,
    ) -> Result<WorkItem, AppError> {
        let number = self
            .work_item_repo
            .get_next_number(project_id, "BUG")
            .await?;

        let item = self
            .work_item_repo
            .create(
                project_id,
                WorkItemType::Bug,
                &number,
                &req.title,
                req.description.as_deref(),
                req.priority,
                req.assignee_id,
                creator_id,
                None,
                None,
                None,
                None,
                Some(req.severity),
            )
            .await?;

        self.audit_repo
            .create(item.id, creator_id, "created", None, Some(&item.title))
            .await?;

        Ok(item)
    }

    /// 需求 4.5（分配）：分配工作项给成员，触发通知（此处记录审计日志）
    pub async fn assign(
        &self,
        operator_id: UserId,
        item_id: WorkItemId,
        assignee_id: UserId,
    ) -> Result<WorkItem, AppError> {
        let item = self
            .work_item_repo
            .find_by_id(item_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let old_assignee = item
            .assignee_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "none".into());

        let updated = self
            .work_item_repo
            .update_assignee(item_id, Some(assignee_id))
            .await?;

        self.audit_repo
            .create(
                item_id,
                operator_id,
                "assignee",
                Some(&old_assignee),
                Some(&assignee_id.to_string()),
            )
            .await?;

        Ok(updated)
    }

    /// 需求 4.5：记录实际工时，精度为 0.5 小时
    pub async fn log_actual_hours(
        &self,
        _operator_id: UserId,
        task_id: WorkItemId,
        hours: f32,
    ) -> Result<WorkItem, AppError> {
        if !is_valid_hours(hours) {
            return Err(AppError::InvalidHoursPrecision);
        }

        let _ = self
            .work_item_repo
            .find_by_id(task_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let updated = self
            .work_item_repo
            .update_actual_hours(task_id, hours)
            .await?;

        Ok(updated)
    }

    // ---- Task 8.5: add_comment, upload_attachment, list_work_items, get_change_history ----

    /// 需求 3.5：添加评论
    pub async fn add_comment(
        &self,
        author_id: UserId,
        item_id: WorkItemId,
        content: &str,
    ) -> Result<Comment, AppError> {
        let comment = self
            .work_item_repo
            .add_comment(item_id, author_id, content)
            .await?;
        Ok(comment)
    }

    /// 需求 6.7：上传附件，校验大小不超过 20MB
    pub async fn upload_attachment(
        &self,
        uploader_id: UserId,
        item_id: WorkItemId,
        file: FileUpload,
    ) -> Result<Attachment, AppError> {
        validate_attachment_size(file.size)?;

        let storage_key = format!("uploads/{}/{}", item_id, file.filename);

        let attachment = self
            .work_item_repo
            .add_attachment(item_id, uploader_id, &file.filename, file.size, &storage_key)
            .await?;

        Ok(attachment)
    }

    /// 需求 3.6：多条件筛选工作项
    pub async fn list_work_items(
        &self,
        _requester_id: UserId,
        project_id: ProjectId,
        filter: WorkItemFilter,
    ) -> Result<Vec<WorkItem>, AppError> {
        self.work_item_repo
            .list_by_filter(project_id, &filter)
            .await
    }

    /// 需求 3.8：获取变更历史
    pub async fn get_change_history(
        &self,
        _requester_id: UserId,
        item_id: WorkItemId,
    ) -> Result<Vec<AuditLog>, AppError> {
        self.audit_repo.list_by_work_item(item_id).await
    }
}
