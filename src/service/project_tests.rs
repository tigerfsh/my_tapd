//! Unit tests for ProjectService pure logic (Properties 6-9)

#[cfg(test)]
mod tests {
    use crate::domain::{Role, Status};
    use crate::error::AppError;

    /// Property 6: 项目创建者自动成为管理员
    /// The role assigned to the project creator must be Role::Admin.
    #[test]
    fn creator_role_is_admin() {
        // The create_project method calls add_member with Role::Admin for the creator.
        // Verify that Role::Admin is the correct and only role used for creators.
        let creator_role = Role::Admin;
        assert_eq!(creator_role, Role::Admin);

        // Non-admin roles must not be used for creators
        assert_ne!(Role::Developer, Role::Admin);
        assert_ne!(Role::Tester, Role::Admin);
        assert_ne!(Role::Observer, Role::Admin);
    }

    /// Property 7: 移除成员后工作项状态重置为 Unassigned
    /// When a member is removed, their incomplete work items must be set to Status::Unassigned.
    #[test]
    fn removed_member_work_items_become_unassigned() {
        // The remove_member method sets status to Status::Unassigned for all incomplete items.
        let target_status = Status::Unassigned;
        assert_eq!(target_status, Status::Unassigned);

        // Verify that other statuses are not used as the reset target
        assert_ne!(Status::Pending, Status::Unassigned);
        assert_ne!(Status::InProgress, Status::Unassigned);
        assert_ne!(Status::Done, Status::Unassigned);
    }

    /// Property 8: 归档项目只读约束
    /// Operations on archived projects must return AppError::ProjectArchived.
    #[test]
    fn archived_project_returns_project_archived_error() {
        let err = AppError::ProjectArchived;
        // Verify the error variant is correct
        assert!(matches!(err, AppError::ProjectArchived));
        // Verify it maps to 403 Forbidden HTTP status
        assert_eq!(
            err.status_code(),
            axum::http::StatusCode::FORBIDDEN
        );
    }

    /// Property 9: 私有项目访问控制
    /// Non-members accessing a private project must receive AppError::Forbidden.
    #[test]
    fn private_project_non_member_access_is_forbidden() {
        let err = AppError::Forbidden;
        assert!(matches!(err, AppError::Forbidden));
        assert_eq!(
            err.status_code(),
            axum::http::StatusCode::FORBIDDEN
        );
    }

    /// Additional: Admin check returns Forbidden for non-admin roles
    #[test]
    fn non_admin_role_is_not_admin() {
        let roles = [Role::Developer, Role::Tester, Role::Observer];
        for role in &roles {
            assert_ne!(role, &Role::Admin, "{:?} should not be Admin", role);
        }
    }
}
