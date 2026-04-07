use std::sync::Arc;

use crate::domain::{
    BurndownChart, CreateIterationRequest, Iteration, IterationId, IterationStatus, IterationSummary,
    ProjectId, Status, UpdateIterationRequest, UserId, WorkItemId,
};
use crate::error::AppError;
use crate::repository::iteration_repo::IterationRepo;

pub struct IterationService {
    iteration_repo: Arc<IterationRepo>,
}

impl IterationService {
    pub fn new(iteration_repo: Arc<IterationRepo>) -> Self {
        Self { iteration_repo }
    }

    // ---- Task 9.1: create_iteration ----

    /// 需求 5.1、5.2：创建迭代，检测时间冲突
    pub async fn create_iteration(
        &self,
        operator_id: UserId,
        project_id: ProjectId,
        req: CreateIterationRequest,
    ) -> Result<Iteration, AppError> {
        let conflicts = self
            .iteration_repo
            .find_overlapping(project_id, req.start_date, req.end_date, None)
            .await?;

        if let Some(conflicting) = conflicts.first() {
            return Err(AppError::IterationConflict {
                conflict_name: conflicting.name.clone(),
            });
        }

        let iteration = self
            .iteration_repo
            .create(
                project_id,
                &req.name,
                req.start_date,
                req.end_date,
                req.goal.as_deref(),
                operator_id,
            )
            .await?;

        Ok(iteration)
    }

    // ---- Task 9.2: assign_stories, update_iteration ----

    /// 需求 5.3：将故事分配到迭代
    pub async fn assign_stories(
        &self,
        _operator_id: UserId,
        iteration_id: IterationId,
        story_ids: Vec<WorkItemId>,
    ) -> Result<(), AppError> {
        for story_id in story_ids {
            self.iteration_repo
                .assign_story(iteration_id, story_id)
                .await?;
        }
        Ok(())
    }

    /// 需求 5.4：更新迭代信息，仅允许在未开始状态下修改，检测时间冲突
    pub async fn update_iteration(
        &self,
        _operator_id: UserId,
        iteration_id: IterationId,
        req: UpdateIterationRequest,
    ) -> Result<Iteration, AppError> {
        let iteration = self
            .iteration_repo
            .find_by_id(iteration_id)
            .await?
            .ok_or(AppError::NotFound)?;

        if iteration.status != IterationStatus::NotStarted {
            return Err(AppError::Forbidden);
        }

        // Check for date conflicts if dates are being updated
        let new_start = req.start_date.unwrap_or(iteration.start_date);
        let new_end = req.end_date.unwrap_or(iteration.end_date);

        if req.start_date.is_some() || req.end_date.is_some() {
            let conflicts = self
                .iteration_repo
                .find_overlapping(
                    iteration.project_id,
                    new_start,
                    new_end,
                    Some(iteration_id),
                )
                .await?;

            if let Some(conflicting) = conflicts.first() {
                return Err(AppError::IterationConflict {
                    conflict_name: conflicting.name.clone(),
                });
            }
        }

        let updated = self
            .iteration_repo
            .update(
                iteration_id,
                req.name.as_deref(),
                req.start_date,
                req.end_date,
                req.goal.as_deref(),
            )
            .await?;

        Ok(updated)
    }

    // ---- Task 9.3: close_iteration, get_burndown_data ----

    /// 需求 5.5：关闭迭代，未完成故事回归 Backlog，返回迭代摘要
    pub async fn close_iteration(
        &self,
        iteration_id: IterationId,
    ) -> Result<IterationSummary, AppError> {
        let _ = self
            .iteration_repo
            .find_by_id(iteration_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let stories = self.iteration_repo.list_stories(iteration_id).await?;
        let total_stories = stories.len() as i32;
        let completed_stories = stories
            .iter()
            .filter(|s| s.status == Status::Done || s.status == Status::Closed)
            .count() as i32;

        let unassigned_ids = self
            .iteration_repo
            .unassign_incomplete_stories(iteration_id)
            .await?;

        self.iteration_repo
            .update_status(iteration_id, IterationStatus::Completed)
            .await?;

        Ok(IterationSummary {
            iteration_id,
            total_stories,
            completed_stories,
            moved_to_backlog: unassigned_ids.len() as i32,
        })
    }

    /// 需求 5.6：获取燃尽图数据
    pub async fn get_burndown_data(
        &self,
        iteration_id: IterationId,
    ) -> Result<BurndownChart, AppError> {
        let snapshots = self
            .iteration_repo
            .get_burndown_snapshots(iteration_id)
            .await?;

        Ok(BurndownChart {
            iteration_id,
            snapshots,
        })
    }
}
