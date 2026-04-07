use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;

use crate::api::middleware::AuthUser;
use crate::api::router::AppState;
use crate::domain::{
    CreateBugRequest, CreateRequirementRequest, CreateStoryRequest, CreateTaskRequest,
    Status, UserId, WorkItemFilter,
};
use crate::error::AppError;
use crate::repository::{
    audit_repo::AuditRepo,
    project_repo::ProjectRepo,
    work_item_repo::WorkItemRepo,
};
use crate::service::work_item_service::WorkItemService;

fn make_work_item_service(state: &AppState) -> WorkItemService {
    WorkItemService::new(
        Arc::new(WorkItemRepo::new(state.pg_pool.clone())),
        Arc::new(AuditRepo::new(state.pg_pool.clone())),
        Arc::new(ProjectRepo::new(state.pg_pool.clone())),
    )
}

/// POST /projects/:pid/requirements
pub async fn create_requirement_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(pid): Path<i64>,
    Json(req): Json<CreateRequirementRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_work_item_service(&state);
    let item = svc.create_requirement(auth_user.user_id, pid, req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({ "data": item }))))
}

/// GET /projects/:pid/requirements
pub async fn list_requirements_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(pid): Path<i64>,
    axum::extract::Query(filter): axum::extract::Query<WorkItemFilter>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_work_item_service(&state);
    let items = svc.list_work_items(auth_user.user_id, pid, filter).await?;
    Ok(Json(json!({ "data": items })))
}

/// GET /projects/:pid/requirements/:id
pub async fn get_work_item_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path((_pid, id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, AppError> {
    let repo = WorkItemRepo::new(state.pg_pool.clone());
    let item = repo.find_by_id(id).await?.ok_or(AppError::NotFound)?;
    Ok(Json(json!({ "data": item })))
}

/// PUT /projects/:pid/requirements/:id
pub async fn update_work_item_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path((_pid, id)): Path<(i64, i64)>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let title = body["title"].as_str();
    let description = body["description"].as_str();
    let updated = sqlx::query_as::<_, crate::domain::WorkItem>(
        "UPDATE work_items SET \
         title = COALESCE($2, title), \
         description = COALESCE($3, description), \
         updated_at = NOW() \
         WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(title)
    .bind(description)
    .fetch_optional(&state.pg_pool)
    .await?
    .ok_or(AppError::NotFound)?;
    Ok(Json(json!({ "data": updated })))
}

/// GET /projects/:pid/requirements/:id/history
pub async fn get_history_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path((_pid, id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_work_item_service(&state);
    let history = svc.get_change_history(auth_user.user_id, id).await?;
    Ok(Json(json!({ "data": history })))
}

/// POST /requirements/:id/stories
pub async fn create_story_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<CreateStoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_work_item_service(&state);
    let item = svc.create_story(auth_user.user_id, id, req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({ "data": item }))))
}

/// POST /stories/:id/tasks
pub async fn create_task_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_work_item_service(&state);
    let item = svc.create_task(auth_user.user_id, id, req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({ "data": item }))))
}

/// POST /projects/:pid/bugs
pub async fn create_bug_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(pid): Path<i64>,
    Json(req): Json<CreateBugRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_work_item_service(&state);
    let item = svc.create_bug(auth_user.user_id, pid, req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({ "data": item }))))
}

/// GET /projects/:pid/bugs
pub async fn list_bugs_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(pid): Path<i64>,
    axum::extract::Query(filter): axum::extract::Query<WorkItemFilter>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_work_item_service(&state);
    let items = svc.list_work_items(auth_user.user_id, pid, filter).await?;
    Ok(Json(json!({ "data": items })))
}

/// PUT /work-items/:id/status
pub async fn update_status_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let new_status: Status = serde_json::from_value(body["status"].clone())
        .map_err(|_| AppError::Internal(anyhow::anyhow!("invalid status")))?;
    let svc = make_work_item_service(&state);
    let item = svc.update_status(auth_user.user_id, id, new_status).await?;
    Ok(Json(json!({ "data": item })))
}

/// PUT /work-items/:id/assign
pub async fn assign_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let assignee_id: UserId = body["assignee_id"]
        .as_i64()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("missing assignee_id")))?;
    let svc = make_work_item_service(&state);
    let item = svc.assign(auth_user.user_id, id, assignee_id).await?;
    Ok(Json(json!({ "data": item })))
}

/// POST /work-items/:id/comments
pub async fn add_comment_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let content = body["content"]
        .as_str()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("missing content")))?;
    let svc = make_work_item_service(&state);
    let comment = svc.add_comment(auth_user.user_id, id, content).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({ "data": comment }))))
}

/// POST /work-items/:id/attachments
pub async fn upload_attachment_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    // Accept JSON body with filename and size for skeleton implementation
    let filename = body["filename"]
        .as_str()
        .unwrap_or("file")
        .to_string();
    let size = body["size"].as_i64().unwrap_or(0);
    let file = crate::domain::FileUpload {
        filename,
        content_type: body["content_type"]
            .as_str()
            .unwrap_or("application/octet-stream")
            .to_string(),
        size,
        data: vec![],
    };
    let svc = make_work_item_service(&state);
    let attachment = svc.upload_attachment(auth_user.user_id, id, file).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({ "data": attachment }))))
}

/// PUT /tasks/:id/actual-hours
pub async fn log_actual_hours_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let hours = body["hours"]
        .as_f64()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("missing hours")))? as f32;
    let svc = make_work_item_service(&state);
    let item = svc.log_actual_hours(auth_user.user_id, id, hours).await?;
    Ok(Json(json!({ "data": item })))
}
