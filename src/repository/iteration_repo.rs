use chrono::NaiveDate;
use sqlx::{PgPool, QueryBuilder};

use crate::domain::{
    BurndownSnapshot, Iteration, IterationId, IterationStatus, ProjectId, UserId, WorkItem,
    WorkItemId,
};
use crate::error::AppError;

pub struct IterationRepo {
    pool: PgPool,
}

impl IterationRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        project_id: ProjectId,
        name: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
        goal: Option<&str>,
        created_by: UserId,
    ) -> Result<Iteration, AppError> {
        let iteration = sqlx::query_as::<_, Iteration>(
            "INSERT INTO iterations (project_id, name, start_date, end_date, goal, created_by) \
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(project_id)
        .bind(name)
        .bind(start_date)
        .bind(end_date)
        .bind(goal)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await?;
        Ok(iteration)
    }

    pub async fn find_by_id(&self, id: IterationId) -> Result<Option<Iteration>, AppError> {
        let iteration =
            sqlx::query_as::<_, Iteration>("SELECT * FROM iterations WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(iteration)
    }

    pub async fn list_by_project(
        &self,
        project_id: ProjectId,
    ) -> Result<Vec<Iteration>, AppError> {
        let iterations = sqlx::query_as::<_, Iteration>(
            "SELECT * FROM iterations WHERE project_id = $1 ORDER BY start_date ASC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(iterations)
    }

    pub async fn update(
        &self,
        id: IterationId,
        name: Option<&str>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        goal: Option<&str>,
    ) -> Result<Iteration, AppError> {
        let iteration = sqlx::query_as::<_, Iteration>(
            "UPDATE iterations \
             SET name = COALESCE($2, name), \
                 start_date = COALESCE($3, start_date), \
                 end_date = COALESCE($4, end_date), \
                 goal = COALESCE($5, goal), \
                 updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(name)
        .bind(start_date)
        .bind(end_date)
        .bind(goal)
        .fetch_one(&self.pool)
        .await?;
        Ok(iteration)
    }

    pub async fn update_status(
        &self,
        id: IterationId,
        status: IterationStatus,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE iterations SET status = $2, updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .bind(status)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_overlapping(
        &self,
        project_id: ProjectId,
        start_date: NaiveDate,
        end_date: NaiveDate,
        exclude_id: Option<IterationId>,
    ) -> Result<Vec<Iteration>, AppError> {
        let mut qb = QueryBuilder::new("SELECT * FROM iterations WHERE project_id = ");
        qb.push_bind(project_id);
        qb.push(" AND status != 'completed' AND start_date <= ");
        qb.push_bind(end_date);
        qb.push(" AND ");
        qb.push_bind(start_date);
        qb.push(" <= end_date");
        if let Some(eid) = exclude_id {
            qb.push(" AND id != ");
            qb.push_bind(eid);
        }
        let iterations = qb
            .build_query_as::<Iteration>()
            .fetch_all(&self.pool)
            .await?;
        Ok(iterations)
    }

    pub async fn assign_story(
        &self,
        iteration_id: IterationId,
        story_id: WorkItemId,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE work_items SET iteration_id = $1, updated_at = NOW() WHERE id = $2",
        )
        .bind(iteration_id)
        .bind(story_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn unassign_incomplete_stories(
        &self,
        iteration_id: IterationId,
    ) -> Result<Vec<WorkItemId>, AppError> {
        let rows = sqlx::query_as::<_, (WorkItemId,)>(
            "UPDATE work_items \
             SET iteration_id = NULL, updated_at = NOW() \
             WHERE iteration_id = $1 AND status NOT IN ('done', 'closed') \
             RETURNING id",
        )
        .bind(iteration_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.0).collect())
    }

    pub async fn list_stories(
        &self,
        iteration_id: IterationId,
    ) -> Result<Vec<WorkItem>, AppError> {
        let items = sqlx::query_as::<_, WorkItem>(
            "SELECT * FROM work_items WHERE iteration_id = $1 AND item_type = 'story'",
        )
        .bind(iteration_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(items)
    }

    /// Save or update a burndown snapshot for the given date.
    ///
    /// Requires a unique constraint on (iteration_id, snapshot_date):
    ///   ALTER TABLE burndown_snapshots
    ///     ADD CONSTRAINT uq_burndown_snapshot UNIQUE (iteration_id, snapshot_date);
    pub async fn save_burndown_snapshot(
        &self,
        iteration_id: IterationId,
        snapshot_date: NaiveDate,
        remaining_points: i32,
        total_points: i32,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO burndown_snapshots \
             (iteration_id, snapshot_date, remaining_points, total_points) \
             VALUES ($1, $2, $3, $4) \
             ON CONFLICT (iteration_id, snapshot_date) \
             DO UPDATE SET remaining_points = $3, total_points = $4",
        )
        .bind(iteration_id)
        .bind(snapshot_date)
        .bind(remaining_points)
        .bind(total_points)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_burndown_snapshots(
        &self,
        iteration_id: IterationId,
    ) -> Result<Vec<BurndownSnapshot>, AppError> {
        let snapshots = sqlx::query_as::<_, BurndownSnapshot>(
            "SELECT * FROM burndown_snapshots WHERE iteration_id = $1 ORDER BY snapshot_date ASC",
        )
        .bind(iteration_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(snapshots)
    }
}
