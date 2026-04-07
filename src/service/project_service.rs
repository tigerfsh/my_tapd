use std::sync::Arc;

use crate::domain::{
    CreateProjectRequest, InviteMemberRequest, Member, Project, ProjectId, Role, Status,
    UpdateProjectRequest, UserId,
};
use crate::error::AppError;
use crate::repository::project_repo::ProjectRepo;
use crate::repository::work_item_repo::WorkItemRepo;

pub struct ProjectService {
    project_repo: Arc<ProjectRepo>,
    work_item_repo: Arc<WorkItemRepo>,
}

impl ProjectService {
    pub fn new(project_repo: Arc<ProjectRepo>, work_item_repo: Arc<WorkItemRepo>) -> Self {
        Self {
            project_repo,
            work_item_repo,
        }
    }

    // ---- Task 7.1: create_project ----

    pub async fn create_project(
        &self,
        creator_id: UserId,
        req: CreateProjectRequest,
    ) -> Result<Project, AppError> {
        let project = self
            .project_repo
            .create(
                &req.name,
                req.description.as_deref(),
                req.project_type,
                req.is_public,
                creator_id,
            )
            .await?;

        // Automatically add creator as Admin
        self.project_repo
            .add_member(project.id, creator_id, Role::Admin)
            .await?;

        Ok(project)
    }

    // ---- Task 7.2: invite_member, remove_member, update_project ----

    pub async fn invite_member(
        &self,
        operator_id: UserId,
        project_id: ProjectId,
        req: InviteMemberRequest,
    ) -> Result<Member, AppError> {
        self.require_admin(project_id, operator_id).await?;

        let member = self
            .project_repo
            .add_member(project_id, req.user_id, req.role)
            .await?;

        Ok(member)
    }

    pub async fn remove_member(
        &self,
        operator_id: UserId,
        project_id: ProjectId,
        member_id: UserId,
    ) -> Result<(), AppError> {
        self.require_admin(project_id, operator_id).await?;

        // Reassign incomplete work items to None and set status to Unassigned
        let items = self
            .work_item_repo
            .list_by_assignee_incomplete(project_id, member_id)
            .await?;

        for item in items {
            self.work_item_repo.update_assignee(item.id, None).await?;
            self.work_item_repo
                .update_status(item.id, Status::Unassigned)
                .await?;
        }

        self.project_repo
            .remove_member(project_id, member_id)
            .await?;

        Ok(())
    }

    pub async fn update_project(
        &self,
        operator_id: UserId,
        project_id: ProjectId,
        req: UpdateProjectRequest,
    ) -> Result<Project, AppError> {
        self.require_admin(project_id, operator_id).await?;

        let project = self
            .project_repo
            .update(project_id, req.name.as_deref(), req.description.as_deref())
            .await?;

        Ok(project)
    }

    // ---- Task 7.3: archive_project, get_project ----

    pub async fn archive_project(
        &self,
        operator_id: UserId,
        project_id: ProjectId,
    ) -> Result<(), AppError> {
        self.require_admin(project_id, operator_id).await?;
        self.project_repo.archive(project_id).await?;
        Ok(())
    }

    pub async fn get_project(
        &self,
        requester_id: UserId,
        project_id: ProjectId,
    ) -> Result<Project, AppError> {
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await?
            .ok_or(AppError::NotFound)?;

        // Private project: only members can access
        if !project.is_public {
            let member = self
                .project_repo
                .get_member(project_id, requester_id)
                .await?;
            if member.is_none() {
                return Err(AppError::Forbidden);
            }
        }

        Ok(project)
    }

    // ---- Helper ----

    async fn require_admin(
        &self,
        project_id: ProjectId,
        user_id: UserId,
    ) -> Result<(), AppError> {
        let member = self
            .project_repo
            .get_member(project_id, user_id)
            .await?;

        match member {
            Some(m) if m.role == Role::Admin => Ok(()),
            _ => Err(AppError::Forbidden),
        }
    }
}
