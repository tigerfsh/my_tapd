use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::{
    IterationId, Priority, Severity, Status, UserId, WorkItemId, WorkItemType,
    ProjectId,
};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkItem {
    pub id: WorkItemId,
    pub project_id: ProjectId,
    pub item_type: WorkItemType,
    pub number: String,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub priority: Priority,
    pub assignee_id: Option<UserId>,
    pub creator_id: UserId,
    pub parent_id: Option<WorkItemId>,
    pub iteration_id: Option<IterationId>,
    pub due_date: Option<NaiveDate>,
    pub story_points: Option<i32>,
    pub estimated_hours: Option<f32>,
    pub actual_hours: Option<f32>,
    pub severity: Option<Severity>,
    pub repro_steps: Option<String>,
    pub reopen_reason: Option<String>,
    pub completion_pct: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkItemLabel {
    pub work_item_id: WorkItemId,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: super::CommentId,
    pub work_item_id: WorkItemId,
    pub author_id: UserId,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attachment {
    pub id: super::AttachmentId,
    pub work_item_id: WorkItemId,
    pub uploader_id: UserId,
    pub filename: String,
    pub file_size: i64,
    pub storage_key: String,
    pub created_at: DateTime<Utc>,
}

// ---- Request/Response types ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequirementRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub assignee_id: Option<UserId>,
    pub due_date: Option<NaiveDate>,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStoryRequest {
    pub title: String,
    pub description: Option<String>,
    pub story_points: Option<i32>,
    pub assignee_id: Option<UserId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub assignee_id: Option<UserId>,
    pub estimated_hours: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBugRequest {
    pub title: String,
    pub description: Option<String>,
    pub repro_steps: Option<String>,
    pub severity: Severity,
    pub priority: Priority,
    pub assignee_id: Option<UserId>,
    pub related_requirement_id: Option<WorkItemId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItemFilter {
    pub status: Option<Status>,
    pub priority: Option<Priority>,
    pub assignee_id: Option<UserId>,
    pub label: Option<String>,
    pub iteration_id: Option<IterationId>,
    pub item_type: Option<WorkItemType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUpload {
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub keyword: String,
    pub item_type: Option<WorkItemType>,
    pub status: Option<Status>,
    pub priority: Option<Priority>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub items: Vec<WorkItem>,
    pub total: i64,
}
