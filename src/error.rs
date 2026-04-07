use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde_json::json;

use crate::domain::Status;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // 认证与授权
    #[error("邮箱已被注册")]
    EmailAlreadyExists,
    #[error("邮箱或密码错误")]
    InvalidCredentials,
    #[error("账户已锁定，解锁时间：{until}")]
    AccountLocked { until: DateTime<Utc> },
    #[error("令牌已过期或无效")]
    InvalidToken,
    #[error("权限不足")]
    Forbidden,
    #[error("未认证")]
    Unauthorized,

    // 资源
    #[error("资源不存在")]
    NotFound,
    #[error("项目已归档，禁止修改")]
    ProjectArchived,
    #[error("迭代时间与 {conflict_name} 冲突")]
    IterationConflict { conflict_name: String },
    #[error("非法的状态转换：{from:?} -> {to:?}")]
    InvalidStatusTransition { from: Status, to: Status },
    #[error("工时精度错误，必须为 0.5 小时的整数倍")]
    InvalidHoursPrecision,
    #[error("附件超过大小限制（最大 {max} 字节，实际 {actual} 字节）")]
    AttachmentTooLarge { max: i64, actual: i64 },

    // 基础设施
    #[error("数据库错误：{0}")]
    Database(#[from] sqlx::Error),
    #[error("内部服务错误")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden | AppError::ProjectArchived => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::EmailAlreadyExists
            | AppError::IterationConflict { .. }
            | AppError::InvalidStatusTransition { .. }
            | AppError::InvalidHoursPrecision
            | AppError::AttachmentTooLarge { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::InvalidCredentials
            | AppError::AccountLocked { .. }
            | AppError::InvalidToken => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            AppError::EmailAlreadyExists => "EMAIL_ALREADY_EXISTS",
            AppError::InvalidCredentials => "INVALID_CREDENTIALS",
            AppError::AccountLocked { .. } => "ACCOUNT_LOCKED",
            AppError::InvalidToken => "INVALID_TOKEN",
            AppError::Forbidden => "FORBIDDEN",
            AppError::Unauthorized => "UNAUTHORIZED",
            AppError::NotFound => "NOT_FOUND",
            AppError::ProjectArchived => "PROJECT_ARCHIVED",
            AppError::IterationConflict { .. } => "ITERATION_CONFLICT",
            AppError::InvalidStatusTransition { .. } => "INVALID_STATUS_TRANSITION",
            AppError::InvalidHoursPrecision => "INVALID_HOURS_PRECISION",
            AppError::AttachmentTooLarge { .. } => "ATTACHMENT_TOO_LARGE",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Internal(_) => "INTERNAL_ERROR",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = json!({
            "error": {
                "code": self.error_code(),
                "message": self.to_string(),
                "details": null
            }
        });
        (status, Json(body)).into_response()
    }
}
