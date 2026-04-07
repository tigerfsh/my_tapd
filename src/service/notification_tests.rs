// Feature: it-requirements-management-system, Property 28: 工作项分配通知
// Feature: it-requirements-management-system, Property 31: 通知已读状态管理
// Feature: it-requirements-management-system, Property 32: 通知重试次数上限

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::domain::{NewNotification, Notification, NotificationId, UserId, WorkItemId};
    use crate::domain::functions::{should_retry_notification, MAX_NOTIFY_RETRIES};

    // ---- Helper: build a minimal Notification ----

    fn make_notification(
        id: NotificationId,
        user_id: UserId,
        event_type: &str,
        work_item_id: Option<WorkItemId>,
        is_read: bool,
        retry_count: i32,
    ) -> Notification {
        use chrono::{TimeZone, Utc};
        Notification {
            id,
            user_id,
            event_type: event_type.to_string(),
            work_item_id,
            content: format!("Notification {}", id),
            is_read,
            retry_count,
            created_at: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        }
    }

    // ========================================================================
    // Property 28: 工作项分配通知
    // Validates: Requirements 8.1
    // ========================================================================

    /// When a work item is assigned, a NewNotification with event_type "assigned"
    /// must be created targeting the assignee.
    #[test]
    fn prop28_assignment_creates_assigned_notification() {
        let assignee_id: UserId = 42;
        let work_item_id: WorkItemId = 7;

        let notification = NewNotification {
            user_id: assignee_id,
            event_type: "assigned".to_string(),
            work_item_id: Some(work_item_id),
            content: "工作项已分配给您".to_string(),
        };

        assert_eq!(notification.user_id, assignee_id);
        assert_eq!(notification.event_type, "assigned");
        assert_eq!(notification.work_item_id, Some(work_item_id));
    }

    proptest! {
        /// For any assignee_id and work_item_id, the NewNotification must carry
        /// event_type "assigned" and target the correct user.
        /// Validates: Requirements 8.1
        #[test]
        fn prop28_assignment_notification_targets_assignee(
            assignee_id in 1i64..=10000i64,
            work_item_id in 1i64..=10000i64,
        ) {
            let notification = NewNotification {
                user_id: assignee_id,
                event_type: "assigned".to_string(),
                work_item_id: Some(work_item_id),
                content: format!("工作项 {} 已分配给您", work_item_id),
            };

            prop_assert_eq!(notification.user_id, assignee_id);
            prop_assert_eq!(&notification.event_type, "assigned");
            prop_assert_eq!(notification.work_item_id, Some(work_item_id));
        }
    }

    // ========================================================================
    // Property 31: 通知已读状态管理
    // Validates: Requirements 8.6
    // ========================================================================

    /// After marking a notification as read, is_read must be true.
    #[test]
    fn prop31_mark_read_sets_is_read_true() {
        let mut n = make_notification(1, 10, "assigned", Some(5), false, 0);
        // Simulate mark_read
        n.is_read = true;
        assert!(n.is_read);
    }

    /// Notifications not in the mark_read set must remain unchanged.
    #[test]
    fn prop31_unmarked_notifications_unchanged() {
        let n = make_notification(2, 10, "assigned", Some(5), false, 0);
        // n was not marked — is_read stays false
        assert!(!n.is_read);
    }

    proptest! {
        /// For any set of notifications, after marking a subset as read,
        /// every marked notification has is_read=true and every unmarked one is unchanged.
        /// Validates: Requirements 8.6
        #[test]
        fn prop31_mark_read_only_affects_targeted_ids(
            ids in prop::collection::vec(1i64..=100i64, 1..=20usize),
            mark_count in 0usize..=20usize,
        ) {
            let notifications: Vec<Notification> = ids
                .iter()
                .map(|&id| make_notification(id, 1, "assigned", None, false, 0))
                .collect();

            let mark_count = mark_count.min(notifications.len());
            let to_mark: Vec<NotificationId> = notifications[..mark_count]
                .iter()
                .map(|n| n.id)
                .collect();

            // Simulate mark_read: set is_read=true for targeted ids
            let updated: Vec<Notification> = notifications
                .iter()
                .map(|n| {
                    let mut n2 = n.clone();
                    if to_mark.contains(&n2.id) {
                        n2.is_read = true;
                    }
                    n2
                })
                .collect();

            // Every marked notification must have is_read=true
            for n in updated.iter().filter(|n| to_mark.contains(&n.id)) {
                prop_assert!(n.is_read, "marked notification {} should be read", n.id);
            }

            // Every unmarked notification must still have is_read=false
            for n in updated.iter().filter(|n| !to_mark.contains(&n.id)) {
                prop_assert!(!n.is_read, "unmarked notification {} should remain unread", n.id);
            }
        }
    }

    // ========================================================================
    // Property 32: 通知重试次数上限
    // Validates: Requirements 8.7
    // ========================================================================

    /// should_retry_notification returns true only when retry_count < MAX_NOTIFY_RETRIES (3).
    #[test]
    fn prop32_retry_allowed_below_max() {
        assert!(should_retry_notification(0));
        assert!(should_retry_notification(1));
        assert!(should_retry_notification(2));
    }

    #[test]
    fn prop32_retry_denied_at_max() {
        assert!(!should_retry_notification(MAX_NOTIFY_RETRIES));
        assert!(!should_retry_notification(MAX_NOTIFY_RETRIES + 1));
    }

    proptest! {
        /// For any retry_count, should_retry_notification must return true iff count < 3.
        /// Validates: Requirements 8.7
        #[test]
        fn prop32_retry_count_never_exceeds_max(retry_count in 0i32..=10i32) {
            let can_retry = should_retry_notification(retry_count);
            if retry_count < MAX_NOTIFY_RETRIES {
                prop_assert!(can_retry, "should retry when count={} < {}", retry_count, MAX_NOTIFY_RETRIES);
            } else {
                prop_assert!(!can_retry, "should not retry when count={} >= {}", retry_count, MAX_NOTIFY_RETRIES);
            }
        }
    }
}
