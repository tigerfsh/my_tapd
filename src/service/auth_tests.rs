// Feature: it-requirements-management-system, Property 2: 密码哈希不可逆性
// Feature: it-requirements-management-system, Property 3: 登录令牌有效性
// Feature: it-requirements-management-system, Property 4: 账户锁定机制
// Feature: it-requirements-management-system, Property 5: 重置令牌有效期

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use proptest::prelude::*;

    use crate::domain::functions::{check_account_lock, is_token_valid, should_lock_account};
    use crate::domain::user::User;
    use crate::domain::UserId;
    use crate::service::auth_service::{jwt_generate, jwt_verify};

    // ---- Helper: build a minimal User for lock tests ----

    fn make_user(locked_until: Option<chrono::DateTime<Utc>>) -> User {
        User {
            id: 1,
            email: "test@example.com".into(),
            password_hash: "$2b$12$placeholder".into(),
            nickname: "tester".into(),
            avatar_url: None,
            phone: None,
            is_active: true,
            login_fail_count: 0,
            locked_until,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // ========================================================================
    // Property 2: 密码哈希不可逆性
    // Validates: Requirements 1.3
    // ========================================================================

    proptest! {
        /// For any password, hash != password
        #[test]
        fn prop2_hash_differs_from_password(password in "[a-zA-Z0-9!@#$%]{8,32}") {
            let hash = bcrypt::hash(&password, 4).expect("bcrypt::hash failed");
            prop_assert_ne!(hash.as_str(), password.as_str());
        }

        /// bcrypt::verify(password, hash) returns true
        #[test]
        fn prop2_verify_correct_password(password in "[a-zA-Z0-9!@#$%]{8,32}") {
            let hash = bcrypt::hash(&password, 4).expect("bcrypt::hash failed");
            let ok = bcrypt::verify(&password, &hash).expect("bcrypt::verify failed");
            prop_assert!(ok);
        }

        /// bcrypt::verify(different_password, hash) returns false
        #[test]
        fn prop2_verify_wrong_password_fails(
            password in "[a-zA-Z0-9!@#$%]{8,32}",
            suffix in "[a-zA-Z0-9]{1,4}",
        ) {
            let hash = bcrypt::hash(&password, 4).expect("bcrypt::hash failed");
            let wrong = format!("{}_wrong_{}", password, suffix);
            let ok = bcrypt::verify(&wrong, &hash).expect("bcrypt::verify failed");
            prop_assert!(!ok);
        }
    }

    // ========================================================================
    // Property 3: 登录令牌有效性
    // Validates: Requirements 1.2
    // ========================================================================

    proptest! {
        /// For any user_id, jwt_generate produces a non-empty token
        #[test]
        fn prop3_generate_token_non_empty(user_id in 1i64..=1_000_000i64) {
            let secret = "test-secret-key";
            let token = jwt_generate(user_id as UserId, secret).expect("jwt_generate failed");
            prop_assert!(!token.is_empty());
        }

        /// jwt_verify(token) returns claims where sub == user_id.to_string()
        #[test]
        fn prop3_verify_token_sub_matches(user_id in 1i64..=1_000_000i64) {
            let secret = "test-secret-key";
            let token = jwt_generate(user_id as UserId, secret).expect("jwt_generate failed");
            let claims = jwt_verify(&token, secret).expect("jwt_verify failed");
            prop_assert_eq!(claims.sub, user_id.to_string());
        }

        /// Token signed with wrong secret fails verification
        #[test]
        fn prop3_wrong_secret_fails(user_id in 1i64..=1_000_000i64) {
            let token = jwt_generate(user_id as UserId, "correct-secret").expect("jwt_generate failed");
            let result = jwt_verify(&token, "wrong-secret");
            prop_assert!(result.is_err());
        }
    }

    // ========================================================================
    // Property 4: 账户锁定机制
    // Validates: Requirements 1.4
    // ========================================================================

    proptest! {
        /// fail_count < 5 => should_lock_account returns None
        #[test]
        fn prop4_no_lock_below_threshold(fail_count in 0i32..5i32) {
            prop_assert!(should_lock_account(fail_count).is_none());
        }

        /// fail_count >= 5 => should_lock_account returns Some(15 minutes)
        #[test]
        fn prop4_lock_at_or_above_threshold(fail_count in 5i32..=100i32) {
            let duration = should_lock_account(fail_count);
            prop_assert!(duration.is_some());
            prop_assert_eq!(duration.unwrap(), Duration::minutes(15));
        }

        /// check_account_lock with locked_until in the future returns Err
        #[test]
        fn prop4_locked_until_future_returns_err(seconds_ahead in 1i64..=3600i64) {
            let now = Utc::now();
            let locked_until = now + Duration::seconds(seconds_ahead);
            let user = make_user(Some(locked_until));
            prop_assert!(check_account_lock(&user, now).is_err());
        }

        /// check_account_lock with locked_until in the past returns Ok
        #[test]
        fn prop4_locked_until_past_returns_ok(seconds_ago in 1i64..=3600i64) {
            let now = Utc::now();
            let locked_until = now - Duration::seconds(seconds_ago);
            let user = make_user(Some(locked_until));
            prop_assert!(check_account_lock(&user, now).is_ok());
        }

        /// check_account_lock with no locked_until returns Ok
        #[test]
        fn prop4_no_lock_returns_ok(_dummy in 0u8..1u8) {
            let now = Utc::now();
            let user = make_user(None);
            prop_assert!(check_account_lock(&user, now).is_ok());
        }
    }

    // ========================================================================
    // Property 5: 重置令牌有效期
    // Validates: Requirements 1.5
    // ========================================================================

    proptest! {
        /// ttl=30min, issued_at=now, checked at now+offset where offset < 30min => valid
        #[test]
        fn prop5_token_valid_within_ttl(offset_minutes in 0i64..30i64) {
            let ttl = Duration::minutes(30);
            let issued_at = Utc::now();
            let check_time = issued_at + Duration::minutes(offset_minutes);
            prop_assert!(is_token_valid(issued_at, check_time, ttl));
        }

        /// ttl=30min, issued_at=now, checked at now+offset where offset >= 30min => invalid
        #[test]
        fn prop5_token_invalid_after_ttl(offset_minutes in 30i64..=120i64) {
            let ttl = Duration::minutes(30);
            let issued_at = Utc::now();
            let check_time = issued_at + Duration::minutes(offset_minutes);
            prop_assert!(!is_token_valid(issued_at, check_time, ttl));
        }
    }
}
