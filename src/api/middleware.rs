use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

use crate::api::router::AppState;
use crate::error::AppError;
use crate::service::auth_service::jwt_verify;

/// Extractor for authenticated user — reads `Authorization: Bearer <token>` header.
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: i64,
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| unauthorized_response())?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| unauthorized_response())?;

        // Check blacklist in Redis
        let blacklist_key = format!("blacklist:{}", token);
        let mut conn = state.redis.clone();
        let blacklisted: Option<i32> = redis::AsyncCommands::get(&mut conn, &blacklist_key)
            .await
            .unwrap_or(None);
        if blacklisted.is_some() {
            return Err(unauthorized_response());
        }

        let claims = jwt_verify(token, &state.jwt_secret)
            .map_err(|_| unauthorized_response())?;

        let user_id: i64 = claims
            .sub
            .parse()
            .map_err(|_| unauthorized_response())?;

        Ok(AuthUser { user_id })
    }
}

fn unauthorized_response() -> (StatusCode, axum::Json<serde_json::Value>) {
    let err = AppError::Unauthorized;
    let body = serde_json::json!({
        "error": {
            "code": "UNAUTHORIZED",
            "message": err.to_string(),
            "details": null
        }
    });
    (StatusCode::UNAUTHORIZED, axum::Json(body))
}
