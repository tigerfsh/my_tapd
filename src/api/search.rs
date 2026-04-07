use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;

use crate::api::middleware::AuthUser;
use crate::api::router::AppState;
use crate::domain::SearchQuery;
use crate::error::AppError;
use crate::service::search_service::SearchService;

fn make_search_service(state: &AppState) -> SearchService {
    SearchService::new(state.pg_pool.clone())
}

/// GET /projects/:pid/search
pub async fn search_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(pid): Path<i64>,
    axum::extract::Query(query): axum::extract::Query<SearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_search_service(&state);
    let result = svc.search(auth_user.user_id, pid, query).await?;
    Ok(Json(json!({ "data": result })))
}

/// GET /projects/:pid/work-items/:number
pub async fn find_by_number_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path((pid, number)): Path<(i64, String)>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_search_service(&state);
    let item = svc.find_by_number(auth_user.user_id, pid, &number).await?;
    Ok(Json(json!({ "data": item })))
}
