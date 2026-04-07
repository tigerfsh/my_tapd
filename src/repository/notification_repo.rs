use sqlx::PgPool;

use crate::domain::{Notification, NotificationId, NotificationPreferences, UserId, WorkItemId};
use crate::error::AppError;

pub struct NotificationRepo {
    pool: PgPool,
}

impl NotificationRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: UserId,
        event_type: &str,
        work_item_id: Option<WorkItemId>,
        content: &str,
    ) -> Result<Notification, AppError> {
        let notification = sqlx::query_as::<_, Notification>(
            "INSERT INTO notifications (user_id, event_type, work_item_id, content) \
             VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(user_id)
        .bind(event_type)
        .bind(work_item_id)
        .bind(content)
        .fetch_one(&self.pool)
        .await?;
        Ok(notification)
    }

    pub async fn list_by_user(
        &self,
        user_id: UserId,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<Notification>, AppError> {
        let offset = (page - 1) * per_page;
        let notifications = sqlx::query_as::<_, Notification>(
            "SELECT * FROM notifications WHERE user_id = $1 \
             ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok(notifications)
    }

    pub async fn mark_read(
        &self,
        user_id: UserId,
        notification_ids: &[NotificationId],
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE notifications SET is_read = true WHERE user_id = $1 AND id = ANY($2)",
        )
        .bind(user_id)
        .bind(notification_ids)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn mark_all_read(&self, user_id: UserId) -> Result<(), AppError> {
        sqlx::query("UPDATE notifications SET is_read = true WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn increment_retry(&self, id: NotificationId) -> Result<i32, AppError> {
        let row = sqlx::query_as::<_, (i32,)>(
            "UPDATE notifications SET retry_count = retry_count + 1 WHERE id = $1 \
             RETURNING retry_count",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.0)
    }

    pub async fn get_preferences(
        &self,
        user_id: UserId,
    ) -> Result<Option<NotificationPreferences>, AppError> {
        let prefs = sqlx::query_as::<_, NotificationPreferences>(
            "SELECT * FROM notification_preferences WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(prefs)
    }

    pub async fn upsert_preferences(
        &self,
        prefs: &NotificationPreferences,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO notification_preferences \
             (user_id, on_assigned, on_status_change, on_comment, on_due_date) \
             VALUES ($1, $2, $3, $4, $5) \
             ON CONFLICT (user_id) DO UPDATE \
             SET on_assigned = $2, on_status_change = $3, on_comment = $4, on_due_date = $5",
        )
        .bind(prefs.user_id)
        .bind(prefs.on_assigned)
        .bind(prefs.on_status_change)
        .bind(prefs.on_comment)
        .bind(prefs.on_due_date)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
