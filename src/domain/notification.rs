use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{NotificationId, UserId, WorkItemId};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: NotificationId,
    pub user_id: UserId,
    pub event_type: String,
    pub work_item_id: Option<WorkItemId>,
    pub content: String,
    pub is_read: bool,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationPreferences {
    pub user_id: UserId,
    pub on_assigned: bool,
    pub on_status_change: bool,
    pub on_comment: bool,
    pub on_due_date: bool,
}

// ---- Request/Response types ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewNotification {
    pub user_id: UserId,
    pub event_type: String,
    pub work_item_id: Option<WorkItemId>,
    pub content: String,
}
