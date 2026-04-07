use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{UserId, WorkItemId};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditLog {
    pub id: i64,
    pub work_item_id: WorkItemId,
    pub operator_id: UserId,
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_at: DateTime<Utc>,
}
