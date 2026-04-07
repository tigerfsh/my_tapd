use std::sync::Arc;

use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use redis::{AsyncCommands, aio::ConnectionManager};
use serde::{Deserialize, Serialize};

use crate::domain::{
    AuthToken, LoginRequest, RegisterRequest, UpdateProfileRequest, User, UserId,
};
use crate::domain::functions::{check_account_lock, should_lock_account};
use crate::error::AppError;
use crate::repository::user_repo::UserRepo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id as string
    pub exp: usize,
}

pub struct AuthService {
    user_repo: Arc<UserRepo>,
    redis: ConnectionManager,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repo: Arc<UserRepo>, redis: ConnectionManager, jwt_secret: String) -> Self {
        Self {
            user_repo,
            redis,
            jwt_secret,
        }
    }

    // ---- Task 6.1: register ----

    pub async fn register(&self, req: RegisterRequest) -> Result<User, AppError> {
        // Check email uniqueness
        if self.user_repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::EmailAlreadyExists);
        }

        // Hash password
        let hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        // Create user
        let user = self.user_repo.create(&req.email, &hash, &req.nickname).await?;

        // Log verification email (no real email service)
        tracing::info!("Verification email would be sent to {}", req.email);

        Ok(user)
    }

    // ---- Task 6.2: verify_email, login ----

    pub async fn verify_email(&self, token: &str) -> Result<(), AppError> {
        // Parse token as "verify:{user_id}"
        let user_id: UserId = token
            .strip_prefix("verify:")
            .and_then(|s| s.parse().ok())
            .ok_or(AppError::InvalidToken)?;

        // Find user and activate
        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::InvalidToken)?;

        self.user_repo.activate(user_id).await?;
        Ok(())
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthToken, AppError> {
        // Find user by email
        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        // Check account lock
        check_account_lock(&user, Utc::now())?;

        // Verify password
        let password_valid = bcrypt::verify(&req.password, &user.password_hash)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        if !password_valid {
            // Increment fail count
            let fail_count = self.user_repo.increment_login_fail(user.id).await?;

            // Check if should lock
            if let Some(lock_duration) = should_lock_account(fail_count) {
                let until = Utc::now() + lock_duration;
                self.user_repo.lock_account(user.id, until).await?;
            }

            return Err(AppError::InvalidCredentials);
        }

        // Reset fail count on success
        self.user_repo.reset_login_fail(user.id).await?;

        // Generate JWT
        let access_token = self.generate_token(user.id)?;

        Ok(AuthToken {
            access_token,
            token_type: "Bearer".into(),
            expires_in: 86400,
        })
    }

    // ---- Task 6.3: request_password_reset, reset_password ----

    pub async fn request_password_reset(&self, email: &str) -> Result<(), AppError> {
        // Find user by email (silently ignore if not found for security)
        let user = match self.user_repo.find_by_email(email).await? {
            Some(u) => u,
            None => return Ok(()),
        };

        // Generate token
        let token = uuid::Uuid::new_v4().to_string();

        // Store in Redis: SET reset:{token} {user_id} EX 1800 (30 min)
        let key = format!("reset:{}", token);
        let mut conn = self.redis.clone();
        conn.set_ex::<_, _, ()>(&key, user.id.to_string(), 1800)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        tracing::info!("Password reset email would be sent to {}", email);
        Ok(())
    }

    pub async fn reset_password(&self, token: &str, new_password: &str) -> Result<(), AppError> {
        let key = format!("reset:{}", token);
        let mut conn = self.redis.clone();

        // GET reset:{token} → parse user_id
        let user_id_str: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        let user_id: UserId = user_id_str
            .and_then(|s| s.parse().ok())
            .ok_or(AppError::InvalidToken)?;

        // Hash new password
        let hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        // Update password
        self.user_repo.update_password(user_id, &hash).await?;

        // Delete Redis key
        conn.del::<_, ()>(&key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        Ok(())
    }

    // ---- Task 6.4: logout, update_profile ----

    pub async fn logout(&self, _user_id: UserId, token: &str) -> Result<(), AppError> {
        // Add token to Redis blacklist: SET blacklist:{token} 1 EX 86400
        let key = format!("blacklist:{}", token);
        let mut conn = self.redis.clone();
        conn.set_ex::<_, _, ()>(&key, 1i32, 86400)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        Ok(())
    }

    pub async fn update_profile(
        &self,
        user_id: UserId,
        req: UpdateProfileRequest,
    ) -> Result<User, AppError> {
        self.user_repo
            .update_profile(
                user_id,
                req.nickname.as_deref(),
                req.avatar_url.as_deref(),
                req.phone.as_deref(),
            )
            .await
    }

    // ---- Helper methods ----

    pub fn generate_token(&self, user_id: UserId) -> Result<String, AppError> {
        jwt_generate(user_id, &self.jwt_secret)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        jwt_verify(token, &self.jwt_secret)
    }
}

// ---- Pure JWT free functions (pub(crate) for testing) ----

pub(crate) fn jwt_generate(user_id: UserId, secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = (now + chrono::Duration::seconds(86400)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
}

pub(crate) fn jwt_verify(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| AppError::InvalidToken)?;
    Ok(token_data.claims)
}
