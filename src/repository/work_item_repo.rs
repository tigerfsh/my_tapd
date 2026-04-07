use chrono::NaiveDate;
use sqlx::{PgPool, QueryBuilder};

use crate::domain::{
    Attachment, Comment, Priority, ProjectId, Severity, Status, UserId, WorkItem, WorkItemFilter,
    WorkItemId, WorkItemType,
};
use crate::error::AppError;

pub struct WorkItemRepo {
    pool: PgPool,
}

impl WorkItemRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        project_id: ProjectId,
        item_type: WorkItemType,
        number: &str,
        title: &str,
        description: Option<&str>,
        priority: Priority,
        assignee_id: Option<UserId>,
        creator_id: UserId,
        parent_id: Option<WorkItemId>,
        due_date: Option<NaiveDate>,
        story_points: Option<i32>,
        estimated_hours: Option<f32>,
        severity: Option<Severity>,
    ) -> Result<WorkItem, AppError> {
        let item = sqlx::query_as::<_, WorkItem>(
            "INSERT INTO work_items \
             (project_id, item_type, number, title, description, priority, assignee_id, \
              creator_id, parent_id, due_date, story_points, estimated_hours, severity) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) \
             RETURNING *",
        )
        .bind(project_id)
        .bind(item_type)
        .bind(number)
        .bind(title)
        .bind(description)
        .bind(priority)
        .bind(assignee_id)
        .bind(creator_id)
        .bind(parent_id)
        .bind(due_date)
        .bind(story_points)
        .bind(estimated_hours)
        .bind(severity)
        .fetch_one(&self.pool)
        .await?;
        Ok(item)
    }

    pub async fn find_by_id(&self, id: WorkItemId) -> Result<Option<WorkItem>, AppError> {
        let item = sqlx::query_as::<_, WorkItem>("SELECT * FROM work_items WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(item)
    }

    pub async fn find_by_number(
        &self,
        project_id: ProjectId,
        number: &str,
    ) -> Result<Option<WorkItem>, AppError> {
        let item = sqlx::query_as::<_, WorkItem>(
            "SELECT * FROM work_items WHERE project_id = $1 AND number = $2",
        )
        .bind(project_id)
        .bind(number)
        .fetch_optional(&self.pool)
        .await?;
        Ok(item)
    }

    pub async fn update_status(
        &self,
        id: WorkItemId,
        status: Status,
    ) -> Result<WorkItem, AppError> {
        let item = sqlx::query_as::<_, WorkItem>(
            "UPDATE work_items SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(status)
        .fetch_one(&self.pool)
        .await?;
        Ok(item)
    }

    pub async fn update_assignee(
        &self,
        id: WorkItemId,
        assignee_id: Option<UserId>,
    ) -> Result<WorkItem, AppError> {
        let item = sqlx::query_as::<_, WorkItem>(
            "UPDATE work_items SET assignee_id = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(assignee_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(item)
    }

    pub async fn update_completion_pct(&self, id: WorkItemId, pct: i32) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE work_items SET completion_pct = $2, updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .bind(pct)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_actual_hours(
        &self,
        id: WorkItemId,
        hours: f32,
    ) -> Result<WorkItem, AppError> {
        let item = sqlx::query_as::<_, WorkItem>(
            "UPDATE work_items SET actual_hours = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(hours)
        .fetch_one(&self.pool)
        .await?;
        Ok(item)
    }

    pub async fn update_reopen_reason(
        &self,
        id: WorkItemId,
        reason: &str,
    ) -> Result<WorkItem, AppError> {
        let item = sqlx::query_as::<_, WorkItem>(
            "UPDATE work_items SET reopen_reason = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(reason)
        .fetch_one(&self.pool)
        .await?;
        Ok(item)
    }

    pub async fn list_by_filter(
        &self,
        project_id: ProjectId,
        filter: &WorkItemFilter,
    ) -> Result<Vec<WorkItem>, AppError> {
        let mut qb = QueryBuilder::new("SELECT * FROM work_items WHERE project_id = ");
        qb.push_bind(project_id);

        if let Some(status) = &filter.status {
            qb.push(" AND status = ");
            qb.push_bind(status.clone());
        }
        if let Some(priority) = &filter.priority {
            qb.push(" AND priority = ");
            qb.push_bind(priority.clone());
        }
        if let Some(assignee_id) = &filter.assignee_id {
            qb.push(" AND assignee_id = ");
            qb.push_bind(*assignee_id);
        }
        if let Some(iteration_id) = &filter.iteration_id {
            qb.push(" AND iteration_id = ");
            qb.push_bind(*iteration_id);
        }
        if let Some(item_type) = &filter.item_type {
            qb.push(" AND item_type = ");
            qb.push_bind(item_type.clone());
        }

        let items = qb
            .build_query_as::<WorkItem>()
            .fetch_all(&self.pool)
            .await?;
        Ok(items)
    }

    pub async fn list_children(&self, parent_id: WorkItemId) -> Result<Vec<WorkItem>, AppError> {
        let items =
            sqlx::query_as::<_, WorkItem>("SELECT * FROM work_items WHERE parent_id = $1")
                .bind(parent_id)
                .fetch_all(&self.pool)
                .await?;
        Ok(items)
    }

    pub async fn list_by_assignee_incomplete(
        &self,
        project_id: ProjectId,
        assignee_id: UserId,
    ) -> Result<Vec<WorkItem>, AppError> {
        let items = sqlx::query_as::<_, WorkItem>(
            "SELECT * FROM work_items \
             WHERE project_id = $1 AND assignee_id = $2 AND status NOT IN ('done', 'closed')",
        )
        .bind(project_id)
        .bind(assignee_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(items)
    }

    pub async fn add_label(&self, work_item_id: WorkItemId, label: &str) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO work_item_labels (work_item_id, label) VALUES ($1, $2) \
             ON CONFLICT DO NOTHING",
        )
        .bind(work_item_id)
        .bind(label)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn remove_label(
        &self,
        work_item_id: WorkItemId,
        label: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            "DELETE FROM work_item_labels WHERE work_item_id = $1 AND label = $2",
        )
        .bind(work_item_id)
        .bind(label)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_labels(&self, work_item_id: WorkItemId) -> Result<Vec<String>, AppError> {
        let rows = sqlx::query_as::<_, (String,)>(
            "SELECT label FROM work_item_labels WHERE work_item_id = $1",
        )
        .bind(work_item_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.0).collect())
    }

    pub async fn add_comment(
        &self,
        work_item_id: WorkItemId,
        author_id: UserId,
        content: &str,
    ) -> Result<Comment, AppError> {
        let comment = sqlx::query_as::<_, Comment>(
            "INSERT INTO comments (work_item_id, author_id, content) \
             VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(work_item_id)
        .bind(author_id)
        .bind(content)
        .fetch_one(&self.pool)
        .await?;
        Ok(comment)
    }

    pub async fn list_comments(
        &self,
        work_item_id: WorkItemId,
    ) -> Result<Vec<Comment>, AppError> {
        let comments = sqlx::query_as::<_, Comment>(
            "SELECT * FROM comments WHERE work_item_id = $1 ORDER BY created_at ASC",
        )
        .bind(work_item_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(comments)
    }

    pub async fn add_attachment(
        &self,
        work_item_id: WorkItemId,
        uploader_id: UserId,
        filename: &str,
        file_size: i64,
        storage_key: &str,
    ) -> Result<Attachment, AppError> {
        let attachment = sqlx::query_as::<_, Attachment>(
            "INSERT INTO attachments (work_item_id, uploader_id, filename, file_size, storage_key) \
             VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(work_item_id)
        .bind(uploader_id)
        .bind(filename)
        .bind(file_size)
        .bind(storage_key)
        .fetch_one(&self.pool)
        .await?;
        Ok(attachment)
    }

    pub async fn get_next_number(
        &self,
        project_id: ProjectId,
        prefix: &str,
    ) -> Result<String, AppError> {
        let pattern = format!("{}%", prefix);
        let row = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM work_items WHERE project_id = $1 AND number LIKE $2",
        )
        .bind(project_id)
        .bind(&pattern)
        .fetch_one(&self.pool)
        .await?;
        let next = row.0 + 1;
        Ok(format!("{}-{:03}", prefix, next))
    }

    pub async fn get_comment_authors(
        &self,
        work_item_id: WorkItemId,
    ) -> Result<Vec<UserId>, AppError> {
        let rows = sqlx::query_as::<_, (UserId,)>(
            "SELECT DISTINCT author_id FROM comments WHERE work_item_id = $1",
        )
        .bind(work_item_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.0).collect())
    }
}
