use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
};
use serde_json::json;

use crate::api::middleware::AuthUser;
use crate::api::router::AppState;
use crate::domain::{LoginRequest, RegisterRequest, UpdateProfileRequest};
use crate::error::AppError;
use crate::repository::{
    user_repo::UserRepo,
};
use crate::service::auth_service::AuthService;

fn make_auth_service(state: &AppState) -> AuthService {
    AuthService::new(
        Arc::new(UserRepo::new(state.pg_pool.clone())),
        state.redis.clone(),
        state.jwt_secret.clone(),
    )
}

/// POST /auth/register
pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_auth_service(&state);
    let user = svc.register(req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({ "data": user }))))
}

/// POST /auth/verify-email
pub async fn verify_email_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let token = body["token"]
        .as_str()
        .ok_or(AppError::InvalidToken)?;
    let svc = make_auth_service(&state);
    svc.verify_email(token).await?;
    Ok(Json(json!({ "message": "邮箱验证成功" })))
}

/// POST /auth/login
pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_auth_service(&state);
    let token = svc.login(req).await?;
    Ok(Json(json!({ "data": token })))
}

/// POST /auth/logout
pub async fn logout_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("")
        .to_string();
    let svc = make_auth_service(&state);
    svc.logout(auth_user.user_id, &token).await?;
    Ok(Json(json!({ "message": "已退出登录" })))
}

/// POST /auth/password-reset/request
pub async fn request_reset_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let email = body["email"]
        .as_str()
        .ok_or(AppError::InvalidToken)?;
    let svc = make_auth_service(&state);
    svc.request_password_reset(email).await?;
    Ok(Json(json!({ "message": "重置邮件已发送" })))
}

/// POST /auth/password-reset/confirm
pub async fn confirm_reset_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let token = body["token"].as_str().ok_or(AppError::InvalidToken)?;
    let new_password = body["new_password"].as_str().ok_or(AppError::InvalidToken)?;
    let svc = make_auth_service(&state);
    svc.reset_password(token, new_password).await?;
    Ok(Json(json!({ "message": "密码重置成功" })))
}

/// GET /users/me
pub async fn get_me_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let repo = UserRepo::new(state.pg_pool.clone());
    let user = repo
        .find_by_id(auth_user.user_id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(json!({ "data": user })))
}

/// PUT /users/me
pub async fn update_me_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = make_auth_service(&state);
    let user = svc.update_profile(auth_user.user_id, req).await?;
    Ok(Json(json!({ "data": user })))
}
