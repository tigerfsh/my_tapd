use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;

use crate::api::middleware::AuthUser;
use crate::api::router::AppState;
use crate::domain::{CreateProjectRequest, InviteMemberRequest, Role, UpdateProjectRequest};
use crate::error::AppError;
use crate::repository::{project_repo::ProjectRepo, work_item_repo::WorkItemRepo};
use crate::service::project_service::ProjectService;

fn make_project_service(state: &AppState) -> ProjectService {
    ProjectService::new(
        Arc::new(ProjectRepo::new(state.pg_pool.clone())),
        Arc::new(WorkItemRepo::new(state.pg_pool.clone())),
    )
}

/// GET /projects
pub async fn list_projects_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_project_service(&state);
    let projects = svc.list_projects(auth_user.user_id).await?;
    Ok(Json(json!({ "data": projects })))
}

/// POST /projects
pub async fn create_project_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(req): Json<CreateProjectRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_project_service(&state);
    let project = svc.create_project(auth_user.user_id, req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({ "data": project }))))
}

/// GET /projects/:id
pub async fn get_project_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_project_service(&state);
    let project = svc.get_project(auth_user.user_id, id).await?;
    Ok(Json(json!({ "data": project })))
}

/// PUT /projects/:id
pub async fn update_project_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<UpdateProjectRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_project_service(&state);
    let project = svc.update_project(auth_user.user_id, id, req).await?;
    Ok(Json(json!({ "data": project })))
}

/// POST /projects/:id/archive
pub async fn archive_project_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_project_service(&state);
    svc.archive_project(auth_user.user_id, id).await?;
    Ok(Json(json!({ "message": "项目已归档" })))
}

/// GET /projects/:id/members
pub async fn list_members_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let repo = ProjectRepo::new(state.pg_pool.clone());
    let members = repo.list_members(id).await?;
    Ok(Json(json!({ "data": members })))
}

/// POST /projects/:id/members
pub async fn invite_member_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<InviteMemberRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_project_service(&state);
    let member = svc.invite_member(auth_user.user_id, id, req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({ "data": member }))))
}

/// PUT /projects/:id/members/:uid
pub async fn update_member_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path((id, uid)): Path<(i64, i64)>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    // Require admin
    let repo = ProjectRepo::new(state.pg_pool.clone());
    let member = repo.get_member(id, auth_user.user_id).await?;
    match member {
        Some(m) if m.role == crate::domain::Role::Admin => {}
        _ => return Err(AppError::Forbidden),
    }

    let role: Role = serde_json::from_value(body["role"].clone())
        .map_err(|_| AppError::Internal(anyhow::anyhow!("invalid role")))?;
    let updated = repo.update_member_role(id, uid, role).await?;
    Ok(Json(json!({ "data": updated })))
}

/// DELETE /projects/:id/members/:uid
pub async fn remove_member_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path((id, uid)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_project_service(&state);
    svc.remove_member(auth_user.user_id, id, uid).await?;
    Ok(Json(json!({ "message": "成员已移除" })))
}
