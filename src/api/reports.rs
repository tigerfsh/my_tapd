use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;

use crate::api::middleware::AuthUser;
use crate::api::router::AppState;
use crate::error::AppError;
use crate::service::report_service::ReportService;

fn make_report_service(state: &AppState) -> ReportService {
    ReportService::new(state.pg_pool.clone())
}

/// GET /projects/:pid/dashboard
pub async fn dashboard_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(pid): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_report_service(&state);
    let data = svc.dashboard_data(pid).await?;
    Ok(Json(json!({ "data": data })))
}

/// GET /projects/:pid/reports/requirements
pub async fn requirements_report_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(pid): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_report_service(&state);
    let report = svc.requirement_completion_report(pid, None).await?;
    Ok(Json(json!({ "data": report })))
}

/// GET /projects/:pid/reports/bugs
pub async fn bugs_report_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(pid): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_report_service(&state);
    let report = svc.bug_stats_report(pid).await?;
    Ok(Json(json!({ "data": report })))
}

/// GET /projects/:pid/reports/members
pub async fn members_report_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(pid): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_report_service(&state);
    let report = svc.member_workload_report(pid).await?;
    Ok(Json(json!({ "data": report })))
}

/// POST /projects/:pid/reports/export
pub async fn export_report_handler(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(pid): Path<i64>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let report_type = body["type"].as_str().unwrap_or("requirements");
    let svc = make_report_service(&state);
    let csv_bytes = svc.export_report(pid, report_type).await?;
    Ok((
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/csv")],
        csv_bytes,
    ))
}
