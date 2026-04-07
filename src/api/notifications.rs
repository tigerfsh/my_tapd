use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    response::IntoResponse,
};
use serde_json::json;

use crate::api::middleware::AuthUser;
use crate::api::router::AppState;
use crate::domain::{NotificationId, NotificationPreferences, Pagination};
use crate::error::AppError;
use crate::repository::notification_repo::NotificationRepo;
use crate::service::notification_service::NotificationService;

fn make_notification_service(state: &AppState) -> NotificationService {
    NotificationService::new(
        Arc::new(NotificationRepo::new(state.pg_pool.clone())),
        state.redis.clone(),
    )
}

/// GET /notifications
pub async fn list_notifications_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    axum::extract::Query(page): axum::extract::Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_notification_service(&state);
    let notifications = svc.list_notifications(auth_user.user_id, page).await?;
    Ok(Json(json!({ "data": notifications })))
}

/// POST /notifications/read
pub async fn mark_read_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let ids: Vec<NotificationId> = serde_json::from_value(body["ids"].clone())
        .map_err(|_| AppError::Internal(anyhow::anyhow!("invalid ids")))?;
    let svc = make_notification_service(&state);
    svc.mark_read(auth_user.user_id, ids).await?;
    Ok(Json(json!({ "message": "已标记为已读" })))
}

/// POST /notifications/read-all
pub async fn mark_all_read_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_notification_service(&state);
    svc.mark_all_read(auth_user.user_id).await?;
    Ok(Json(json!({ "message": "全部已读" })))
}

/// GET /users/me/notification-preferences
pub async fn get_preferences_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let repo = NotificationRepo::new(state.pg_pool.clone());
    let prefs = repo
        .get_preferences(auth_user.user_id)
        .await?
        .unwrap_or(NotificationPreferences {
            user_id: auth_user.user_id,
            on_assigned: true,
            on_status_change: true,
            on_comment: true,
            on_due_date: true,
        });
    Ok(Json(json!({ "data": prefs })))
}

/// PUT /users/me/notification-preferences
pub async fn update_preferences_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut prefs): Json<NotificationPreferences>,
) -> Result<impl IntoResponse, AppError> {
    prefs.user_id = auth_user.user_id;
    let svc = make_notification_service(&state);
    svc.update_preferences(auth_user.user_id, prefs).await?;
    Ok(Json(json!({ "message": "通知偏好已更新" })))
}
