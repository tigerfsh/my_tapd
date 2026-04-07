use std::sync::Arc;

use redis::{AsyncCommands, aio::ConnectionManager};

use crate::domain::{
    NewNotification, Notification, NotificationId, NotificationPreferences, Pagination, UserId,
};
use crate::error::AppError;
use crate::repository::notification_repo::NotificationRepo;

pub struct NotificationService {
    notification_repo: Arc<NotificationRepo>,
    redis: ConnectionManager,
}

impl NotificationService {
    pub fn new(notification_repo: Arc<NotificationRepo>, redis: ConnectionManager) -> Self {
        Self {
            notification_repo,
            redis,
        }
    }

    // ---- Task 10.1: send ----

    /// 需求 8.1、8.2、8.3：创建通知并推送到 Redis 队列
    pub async fn send(&self, notification: NewNotification) -> Result<(), AppError> {
        // Write to DB first (ensures record exists for retry if Redis push fails)
        let saved = self
            .notification_repo
            .create(
                notification.user_id,
                &notification.event_type,
                notification.work_item_id,
                &notification.content,
            )
            .await?;

        // Push to Redis list: LPUSH notifications:{user_id} {json}
        let queue_key = format!("notifications:{}", notification.user_id);
        let payload = serde_json::to_string(&saved)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        let mut conn = self.redis.clone();
        // LPUSH then set 24h TTL
        let _: i64 = conn
            .lpush(&queue_key, &payload)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        let _: bool = conn
            .expire(&queue_key, 86400)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        Ok(())
    }

    // ---- Task 10.1: list_notifications ----

    /// 需求 8.4：分页查询用户通知列表
    pub async fn list_notifications(
        &self,
        user_id: UserId,
        page: Pagination,
    ) -> Result<Vec<Notification>, AppError> {
        self.notification_repo
            .list_by_user(user_id, page.page, page.per_page)
            .await
    }

    // ---- Task 10.1: mark_read ----

    /// 需求 8.6：标记指定通知为已读
    pub async fn mark_read(
        &self,
        user_id: UserId,
        notification_ids: Vec<NotificationId>,
    ) -> Result<(), AppError> {
        self.notification_repo
            .mark_read(user_id, &notification_ids)
            .await
    }

    // ---- Task 10.1: mark_all_read ----

    /// 需求 8.6：标记用户所有通知为已读
    pub async fn mark_all_read(&self, user_id: UserId) -> Result<(), AppError> {
        self.notification_repo.mark_all_read(user_id).await
    }

    // ---- Task 10.1: update_preferences ----

    /// 需求 8.4：更新用户通知偏好设置
    pub async fn update_preferences(
        &self,
        _user_id: UserId,
        prefs: NotificationPreferences,
    ) -> Result<(), AppError> {
        self.notification_repo.upsert_preferences(&prefs).await
    }
}
