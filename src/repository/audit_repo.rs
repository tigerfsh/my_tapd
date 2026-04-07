use sqlx::PgPool;

use crate::domain::{AuditLog, UserId, WorkItemId};
use crate::error::AppError;

pub struct AuditRepo {
    pool: PgPool,
}

impl AuditRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        work_item_id: WorkItemId,
        operator_id: UserId,
        field_name: &str,
        old_value: Option<&str>,
        new_value: Option<&str>,
    ) -> Result<AuditLog, AppError> {
        let log = sqlx::query_as::<_, AuditLog>(
            "INSERT INTO audit_logs (work_item_id, operator_id, field_name, old_value, new_value) \
             VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(work_item_id)
        .bind(operator_id)
        .bind(field_name)
        .bind(old_value)
        .bind(new_value)
        .fetch_one(&self.pool)
        .await?;
        Ok(log)
    }

    pub async fn list_by_work_item(
        &self,
        work_item_id: WorkItemId,
    ) -> Result<Vec<AuditLog>, AppError> {
        let logs = sqlx::query_as::<_, AuditLog>(
            "SELECT * FROM audit_logs WHERE work_item_id = $1 ORDER BY changed_at ASC",
        )
        .bind(work_item_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(logs)
    }
}
