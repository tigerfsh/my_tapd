// Feature: it-requirements-management-system, Property 17: 工时精度约束
// Feature: it-requirements-management-system, Property 38: 搜索关键词截断
// Feature: it-requirements-management-system, Property 15: Story 完成百分比计算正确性
// Feature: it-requirements-management-system, Property 19: 迭代时间冲突检测
// Feature: it-requirements-management-system, Property 13: 逾期需求标记
// Feature: it-requirements-management-system, Property 22: Bug 状态机合法性
// Feature: it-requirements-management-system, Property 25: 附件大小限制
// Feature: it-requirements-management-system, Property 27: 泳道分组正确性

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chrono::{NaiveDate, TimeZone, Utc};
    use proptest::prelude::*;

    use crate::domain::{
        functions::{
            calc_completion_pct, group_by_swimlane, is_overdue, is_valid_bug_transition,
            is_valid_hours, iterations_overlap, sanitize_query, validate_attachment_size,
            SwimlaneDimension, MAX_ATTACHMENT_BYTES,
        },
        Iteration, IterationStatus, Priority, Status, WorkItem, WorkItemId, WorkItemType,
    };

    // ---- Helper: build a minimal WorkItem ----

    fn make_work_item(
        id: WorkItemId,
        status: Status,
        assignee_id: Option<i64>,
        due_date: Option<NaiveDate>,
    ) -> WorkItem {
        WorkItem {
            id,
            project_id: 1,
            item_type: WorkItemType::Task,
            number: format!("TASK-{}", id),
            title: "test".into(),
            description: None,
            status,
            priority: Priority::Medium,
            assignee_id,
            creator_id: 1,
            parent_id: None,
            iteration_id: None,
            due_date,
            story_points: None,
            estimated_hours: None,
            actual_hours: None,
            severity: None,
            repro_steps: None,
            reopen_reason: None,
            completion_pct: None,
            created_at: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        }
    }

    fn make_iteration(
        id: i64,
        start: NaiveDate,
        end: NaiveDate,
        status: IterationStatus,
    ) -> Iteration {
        Iteration {
            id,
            project_id: 1,
            name: format!("Sprint-{}", id),
            goal: None,
            start_date: start,
            end_date: end,
            status,
            created_by: 1,
            created_at: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        }
    }

    fn naive_date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap_or(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
    }

    // ---- Strategy helpers ----

    fn date_strategy() -> impl Strategy<Value = NaiveDate> {
        (2020i32..2030, 1u32..13u32, 1u32..29u32)
            .prop_map(|(y, m, d)| NaiveDate::from_ymd_opt(y, m, d).unwrap_or(naive_date(2024, 1, 1)))
    }

    fn status_strategy() -> impl Strategy<Value = Status> {
        prop_oneof![
            Just(Status::Pending),
            Just(Status::InProgress),
            Just(Status::Done),
            Just(Status::Closed),
            Just(Status::Rejected),
            Just(Status::PendingVerify),
            Just(Status::Fixing),
            Just(Status::Unassigned),
        ]
    }

    // ========================================================================
    // Property 17: 工时精度约束
    // Validates: Requirements 4.5
    // ========================================================================

    proptest! {
        /// Valid hours: multiples of 0.5 in (0, 100] must return true
        #[test]
        fn prop17_valid_hours_multiples_of_half(n in 1u32..=200u32) {
            let hours = n as f32 * 0.5;
            prop_assert!(is_valid_hours(hours), "expected true for hours={}", hours);
        }

        /// Invalid hours: integer + one of the non-half decimals must return false
        #[test]
        fn prop17_invalid_hours_non_multiples(
            integer_part in 0i32..100i32,
            decimal in prop::sample::select(vec![0.1f32, 0.2, 0.3, 0.4, 0.6, 0.7, 0.8, 0.9])
        ) {
            let hours = integer_part as f32 + decimal;
            prop_assert!(!is_valid_hours(hours), "expected false for hours={}", hours);
        }

        /// Zero and negative values must return false
        #[test]
        fn prop17_zero_or_negative_invalid(hours in -100.0f32..=0.0f32) {
            prop_assert!(!is_valid_hours(hours));
        }
    }

    // ========================================================================
    // Property 38: 搜索关键词截断
    // Validates: Requirements 10.5
    // ========================================================================

    proptest! {
        /// Strings longer than 200 chars: result is exactly 200 chars, truncated=true
        #[test]
        fn prop38_long_query_truncated(s in "\\PC{201,400}") {
            let (result, truncated) = sanitize_query(&s);
            prop_assert_eq!(result.chars().count(), 200);
            prop_assert!(truncated);
        }

        /// Strings <= 200 chars: result equals original, truncated=false
        #[test]
        fn prop38_short_query_not_truncated(s in "\\PC{0,200}") {
            let (result, truncated) = sanitize_query(&s);
            prop_assert_eq!(result, s);
            prop_assert!(!truncated);
        }
    }

    // ========================================================================
    // Property 15: Story 完成百分比计算正确性
    // Validates: Requirements 4.3
    // ========================================================================

    proptest! {
        /// N tasks, K done: result == round(K/N * 100)
        #[test]
        fn prop15_completion_pct_correct(done_flags in prop::collection::vec(any::<bool>(), 1..=50)) {
            let tasks: Vec<WorkItem> = done_flags
                .iter()
                .enumerate()
                .map(|(i, &done)| {
                    make_work_item(
                        i as i64 + 1,
                        if done { Status::Done } else { Status::InProgress },
                        None,
                        None,
                    )
                })
                .collect();

            let n = tasks.len();
            let k = done_flags.iter().filter(|&&d| d).count();
            let expected = ((k as f64 / n as f64) * 100.0).round() as i32;
            prop_assert_eq!(calc_completion_pct(&tasks), expected);
        }

        /// Empty task list returns 0
        #[test]
        fn prop15_empty_tasks_returns_zero(_dummy in 0u8..1u8) {
            prop_assert_eq!(calc_completion_pct(&[]), 0);
        }
    }

    // ========================================================================
    // Property 19: 迭代时间冲突检测
    // Validates: Requirements 5.2
    // ========================================================================

    proptest! {
        /// iterations_overlap is consistent with manual interval overlap check
        #[test]
        fn prop19_overlap_consistent_with_manual(
            a_start in date_strategy(),
            a_len in 1i64..30i64,
            b_start in date_strategy(),
            b_len in 1i64..30i64,
        ) {
            let a_end = a_start + chrono::Duration::days(a_len);
            let b_end = b_start + chrono::Duration::days(b_len);

            let iter_a = make_iteration(1, a_start, a_end, IterationStatus::InProgress);
            let iter_b = make_iteration(2, b_start, b_end, IterationStatus::InProgress);

            let manual_overlap = a_start <= b_end && b_start <= a_end;
            prop_assert_eq!(iterations_overlap(&iter_a, &iter_b), manual_overlap);
        }
    }

    // ========================================================================
    // Property 13: 逾期需求标记
    // Validates: Requirements 3.7
    // ========================================================================

    proptest! {
        /// due_date < today AND status != Done AND status != Closed => is_overdue == true
        #[test]
        fn prop13_overdue_when_past_due_and_not_done(
            days_ago in 1i64..365i64,
            status in prop_oneof![
                Just(Status::Pending),
                Just(Status::InProgress),
                Just(Status::Fixing),
                Just(Status::PendingVerify),
                Just(Status::Rejected),
                Just(Status::Unassigned),
            ]
        ) {
            let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
            let due = today - chrono::Duration::days(days_ago);
            let item = make_work_item(1, status, None, Some(due));
            prop_assert!(is_overdue(&item, today));
        }

        /// status == Done => is_overdue == false regardless of due_date
        #[test]
        fn prop13_not_overdue_when_done(days_ago in 1i64..365i64) {
            let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
            let due = today - chrono::Duration::days(days_ago);
            let item = make_work_item(1, Status::Done, None, Some(due));
            prop_assert!(!is_overdue(&item, today));
        }

        /// status == Closed => is_overdue == false regardless of due_date
        #[test]
        fn prop13_not_overdue_when_closed(days_ago in 1i64..365i64) {
            let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
            let due = today - chrono::Duration::days(days_ago);
            let item = make_work_item(1, Status::Closed, None, Some(due));
            prop_assert!(!is_overdue(&item, today));
        }

        /// due_date >= today => is_overdue == false regardless of status
        #[test]
        fn prop13_not_overdue_when_future_due(
            days_ahead in 0i64..365i64,
            status in status_strategy()
        ) {
            let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
            let due = today + chrono::Duration::days(days_ahead);
            let item = make_work_item(1, status, None, Some(due));
            prop_assert!(!is_overdue(&item, today));
        }

        /// No due_date => is_overdue == false
        #[test]
        fn prop13_not_overdue_when_no_due_date(status in status_strategy()) {
            let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
            let item = make_work_item(1, status, None, None);
            prop_assert!(!is_overdue(&item, today));
        }
    }

    // ========================================================================
    // Property 22: Bug 状态机合法性
    // Validates: Requirements 6.3
    // ========================================================================

    proptest! {
        /// Valid transitions must return true
        #[test]
        fn prop22_valid_transitions_allowed(
            transition in prop_oneof![
                Just((Status::Pending,       Status::Fixing)),
                Just((Status::Fixing,        Status::PendingVerify)),
                Just((Status::PendingVerify, Status::Closed)),
                Just((Status::PendingVerify, Status::Rejected)),
                Just((Status::Closed,        Status::Pending)),
            ]
        ) {
            let (from, to) = transition;
            prop_assert!(is_valid_bug_transition(&from, &to));
        }

        /// Invalid transitions must return false
        #[test]
        fn prop22_invalid_transitions_rejected(
            from in status_strategy(),
            to in status_strategy(),
        ) {
            let valid = matches!(
                (&from, &to),
                (Status::Pending,       Status::Fixing)       |
                (Status::Fixing,        Status::PendingVerify)|
                (Status::PendingVerify, Status::Closed)       |
                (Status::PendingVerify, Status::Rejected)     |
                (Status::Closed,        Status::Pending)
            );
            prop_assert_eq!(is_valid_bug_transition(&from, &to), valid);
        }
    }

    // ========================================================================
    // Property 25: 附件大小限制
    // Validates: Requirements 6.7
    // ========================================================================

    proptest! {
        /// size <= 20MB => Ok
        #[test]
        fn prop25_size_within_limit_ok(size in 0i64..=MAX_ATTACHMENT_BYTES) {
            prop_assert!(validate_attachment_size(size).is_ok());
        }

        /// size > 20MB => Err
        #[test]
        fn prop25_size_over_limit_err(excess in 1i64..=10_000_000i64) {
            let size = MAX_ATTACHMENT_BYTES + excess;
            prop_assert!(validate_attachment_size(size).is_err());
        }
    }

    // ========================================================================
    // Property 27: 泳道分组正确性 (Assignee dimension)
    // Validates: Requirements 7.6
    // ========================================================================

    proptest! {
        /// Every item appears in exactly one group; group key matches assignee_id or "unassigned"
        #[test]
        fn prop27_assignee_grouping_correct(
            assignee_ids in prop::collection::vec(prop::option::of(0i64..10i64), 1..=20usize)
        ) {
            let items: Vec<WorkItem> = assignee_ids
                .iter()
                .enumerate()
                .map(|(i, &aid)| make_work_item(i as i64 + 1, Status::InProgress, aid, None))
                .collect();

            let labels: HashMap<WorkItemId, Vec<String>> = HashMap::new();
            let groups = group_by_swimlane(&items, &labels, SwimlaneDimension::Assignee);

            // Every item appears in exactly one group
            for item in &items {
                let expected_key = item
                    .assignee_id
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "unassigned".into());

                let count = groups
                    .iter()
                    .map(|(k, v)| if k == &expected_key { v.iter().filter(|wi| wi.id == item.id).count() } else { 0 })
                    .sum::<usize>();

                prop_assert_eq!(count, 1, "item {} should appear exactly once", item.id);

                // Group key must match assignee_id or "unassigned"
                prop_assert!(
                    groups.contains_key(&expected_key),
                    "group key '{}' not found", expected_key
                );
            }

            // All items appear in at least one group
            let total_in_groups: usize = groups.values().map(|v| v.len()).sum();
            prop_assert_eq!(total_in_groups, items.len());
        }
    }
}
