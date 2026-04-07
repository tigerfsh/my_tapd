pub mod audit;
pub mod functions;
pub mod iteration;
pub mod notification;
pub mod project;
pub mod user;
pub mod work_item;
pub mod tests_property;

pub use audit::AuditLog;
pub use functions::*;
pub use iteration::{
    BurndownChart, BurndownSnapshot, CreateIterationRequest, Iteration, IterationSummary,
    Pagination, UpdateIterationRequest,
};
pub use notification::{NewNotification, Notification, NotificationPreferences};
pub use project::{CreateProjectRequest, InviteMemberRequest, Project, UpdateProjectRequest};
pub use user::{AuthToken, LoginRequest, Member, RegisterRequest, UpdateProfileRequest, User};
pub use work_item::{
    Attachment, Comment, CreateBugRequest, CreateRequirementRequest, CreateStoryRequest,
    CreateTaskRequest, FileUpload, SearchQuery, SearchResult, WorkItem, WorkItemFilter,
    WorkItemLabel,
};

use serde::{Deserialize, Serialize};

// ---- Type aliases ----

pub type UserId = i64;
pub type ProjectId = i64;
pub type WorkItemId = i64;
pub type IterationId = i64;
pub type NotificationId = i64;
pub type CommentId = i64;
pub type AttachmentId = i64;

// ---- Enums ----

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "work_item_type", rename_all = "snake_case")]
pub enum WorkItemType {
    Requirement,
    Story,
    Task,
    Bug,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status", rename_all = "snake_case")]
pub enum Status {
    Pending,
    InProgress,
    Done,
    Closed,
    Rejected,
    PendingVerify,
    Fixing,
    Unassigned,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "priority", rename_all = "snake_case")]
pub enum Priority {
    Urgent,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "role", rename_all = "snake_case")]
pub enum Role {
    Admin,
    Developer,
    Tester,
    Observer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "project_type", rename_all = "snake_case")]
pub enum ProjectType {
    Agile,
    Waterfall,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "severity", rename_all = "snake_case")]
pub enum Severity {
    Fatal,
    Critical,
    Normal,
    Hint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "iteration_status", rename_all = "snake_case")]
pub enum IterationStatus {
    NotStarted,
    InProgress,
    Completed,
}
