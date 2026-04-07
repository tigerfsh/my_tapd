use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::{IterationId, IterationStatus, ProjectId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Iteration {
    pub id: IterationId,
    pub project_id: ProjectId,
    pub name: String,
    pub goal: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: IterationStatus,
    pub created_by: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BurndownSnapshot {
    pub id: i64,
    pub iteration_id: IterationId,
    pub snapshot_date: NaiveDate,
    pub remaining_points: i32,
    pub total_points: i32,
}

// ---- Request/Response types ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIterationRequest {
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub goal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIterationRequest {
    pub name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub goal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationSummary {
    pub iteration_id: IterationId,
    pub total_stories: i32,
    pub completed_stories: i32,
    pub moved_to_backlog: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurndownChart {
    pub iteration_id: IterationId,
    pub snapshots: Vec<BurndownSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
}
