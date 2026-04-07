use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;

use crate::api::middleware::AuthUser;
use crate::api::router::AppState;
use crate::domain::{CreateIterationRequest, UpdateIterationRequest, WorkItemId};
use crate::error::AppError;
use crate::repository::iteration_repo::IterationRepo;
use crate::service::iteration_service::IterationService;

fn make_iteration_service(state: &AppState) -> IterationService {
    IterationService::new(Arc::new(IterationRepo::new(state.pg_pool.clone())))
}

/// POST /projects/:pid/iterations
pub async fn create_iteration_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(pid): Path<i64>,
    Json(req): Json<CreateIterationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_iteration_service(&state);
    let iteration = svc.create_iteration(auth_user.user_id, pid, req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({ "data": iteration }))))
}

/// GET /projects/:pid/iterations
pub async fn list_iterations_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(pid): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let repo = IterationRepo::new(state.pg_pool.clone());
    let iterations = repo.list_by_project(pid).await?;
    Ok(Json(json!({ "data": iterations })))
}

/// PUT /iterations/:id
pub async fn update_iteration_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<UpdateIterationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_iteration_service(&state);
    let iteration = svc.update_iteration(auth_user.user_id, id, req).await?;
    Ok(Json(json!({ "data": iteration })))
}

/// POST /iterations/:id/stories
pub async fn assign_stories_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let story_ids: Vec<WorkItemId> = serde_json::from_value(body["story_ids"].clone())
        .map_err(|_| AppError::Internal(anyhow::anyhow!("invalid story_ids")))?;
    let svc = make_iteration_service(&state);
    svc.assign_stories(auth_user.user_id, id, story_ids).await?;
    Ok(Json(json!({ "message": "故事已分配到迭代" })))
}

/// POST /iterations/:id/close
pub async fn close_iteration_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_iteration_service(&state);
    let summary = svc.close_iteration(id).await?;
    Ok(Json(json!({ "data": summary })))
}

/// GET /iterations/:id/burndown
pub async fn get_burndown_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_iteration_service(&state);
    let chart = svc.get_burndown_data(id).await?;
    Ok(Json(json!({ "data": chart })))
}

/// GET /iterations/:id/stats
pub async fn get_stats_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let repo = IterationRepo::new(state.pg_pool.clone());
    let stories = repo.list_stories(id).await?;
    let total = stories.len() as i64;
    let done = stories
        .iter()
        .filter(|s| s.status == crate::domain::Status::Done)
        .count() as i64;
    Ok(Json(json!({
        "data": {
            "total": total,
            "done": done,
            "in_progress": stories.iter().filter(|s| s.status == crate::domain::Status::InProgress).count() as i64,
            "pending": stories.iter().filter(|s| s.status == crate::domain::Status::Pending).count() as i64,
        }
    })))
}
