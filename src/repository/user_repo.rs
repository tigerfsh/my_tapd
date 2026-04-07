use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::domain::{User, UserId};
use crate::error::AppError;

pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        email: &str,
        password_hash: &str,
        nickname: &str,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash, nickname) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(email)
        .bind(password_hash)
        .bind(nickname)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_id(&self, id: UserId) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn activate(&self, id: UserId) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET is_active = true, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_profile(
        &self,
        id: UserId,
        nickname: Option<&str>,
        avatar_url: Option<&str>,
        phone: Option<&str>,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET \
             nickname = COALESCE($2, nickname), \
             avatar_url = COALESCE($3, avatar_url), \
             phone = COALESCE($4, phone), \
             updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(nickname)
        .bind(avatar_url)
        .bind(phone)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn update_password(&self, id: UserId, password_hash: &str) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE users SET password_hash = $2, updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .bind(password_hash)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn increment_login_fail(&self, id: UserId) -> Result<i32, AppError> {
        let row = sqlx::query_as::<_, (i32,)>(
            "UPDATE users SET login_fail_count = login_fail_count + 1, updated_at = NOW() \
             WHERE id = $1 RETURNING login_fail_count",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.0)
    }

    pub async fn reset_login_fail(&self, id: UserId) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE users SET login_fail_count = 0, locked_until = NULL, updated_at = NOW() \
             WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn lock_account(&self, id: UserId, until: DateTime<Utc>) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE users SET locked_until = $2, updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .bind(until)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
