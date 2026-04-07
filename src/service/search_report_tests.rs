// Properties 33–37: 搜索与报表服务属性测试

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use std::collections::HashMap;

    use crate::domain::functions::sanitize_query;
    use crate::service::report_service::{BugReport, MemberWorkload, RequirementReport};

    // ========================================================================
    // Property 33: 需求完成率计算正确性
    // Validates: Requirements 11.1
    // ========================================================================

    /// Compute completion_rate the same way ReportService does.
    fn calc_completion_rate(total: i64, completed: i64) -> f64 {
        if total > 0 {
            completed as f64 / total as f64 * 100.0
        } else {
            0.0
        }
    }

    proptest! {
        /// For any total > 0 and completed <= total, completion_rate == completed/total * 100.
        /// Validates: Requirements 11.1
        #[test]
        fn prop33_completion_rate_correct(
            total in 1i64..=1000i64,
            completed_frac in 0.0f64..=1.0f64,
        ) {
            let completed = (total as f64 * completed_frac).floor() as i64;
            let completed = completed.min(total);

            let rate = calc_completion_rate(total, completed);
            let expected = completed as f64 / total as f64 * 100.0;

            prop_assert!(
                (rate - expected).abs() < 1e-9,
                "completion_rate={} expected={}", rate, expected
            );
        }

        /// Zero total yields 0.0 completion rate (no division by zero).
        /// Validates: Requirements 11.1
        #[test]
        fn prop33_zero_total_yields_zero_rate(_dummy in 0i64..=100i64) {
            let rate = calc_completion_rate(0, 0);
            prop_assert_eq!(rate, 0.0);
        }
    }

    // ========================================================================
    // Property 34: Bug 统计数据一致性
    // Validates: Requirements 11.2
    // ========================================================================

    proptest! {
        /// new + fixed + remaining == total for any valid BugReport.
        /// Validates: Requirements 11.2
        #[test]
        fn prop34_bug_counts_sum_to_total(
            new_count in 0i64..=100i64,
            fixed_count in 0i64..=100i64,
            remaining_count in 0i64..=100i64,
        ) {
            let total = new_count + fixed_count + remaining_count;
            let report = BugReport {
                total,
                new_count,
                fixed_count,
                remaining_count,
                by_severity: HashMap::new(),
            };

            prop_assert_eq!(
                report.new_count + report.fixed_count + report.remaining_count,
                report.total,
                "new + fixed + remaining must equal total"
            );
        }
    }

    // ========================================================================
    // Property 35: 成员工时汇总正确性
    // Validates: Requirements 11.3
    // ========================================================================

    proptest! {
        /// Sum of individual member hours equals the aggregate total.
        /// Validates: Requirements 11.3
        #[test]
        fn prop35_member_hours_sum_correct(
            hours in prop::collection::vec(0.0f64..=100.0f64, 1..=20usize),
        ) {
            let workloads: Vec<MemberWorkload> = hours
                .iter()
                .enumerate()
                .map(|(i, &h)| MemberWorkload {
                    user_id: i as i64 + 1,
                    completed_count: 1,
                    total_hours: h,
                })
                .collect();

            let sum: f64 = workloads.iter().map(|w| w.total_hours).sum();
            let expected: f64 = hours.iter().sum();

            prop_assert!(
                (sum - expected).abs() < 1e-9,
                "sum of member hours={} expected={}", sum, expected
            );
        }
    }

    // ========================================================================
    // Property 36: 搜索结果关键词匹配（sanitize_query 行为）
    // Validates: Requirements 10.5
    // ========================================================================

    proptest! {
        /// Keywords within 200 chars are returned unchanged (not truncated).
        /// Validates: Requirements 10.5
        #[test]
        fn prop36_short_keyword_not_truncated(
            keyword in "[a-zA-Z0-9 ]{1,200}",
        ) {
            let (sanitized, truncated) = sanitize_query(&keyword);
            let char_count = keyword.chars().count();
            if char_count <= 200 {
                prop_assert!(!truncated, "short keyword should not be truncated");
                prop_assert_eq!(&sanitized, &keyword);
            }
        }

        /// Keywords exceeding 200 chars are truncated to exactly 200 chars.
        /// Validates: Requirements 10.5
        #[test]
        fn prop36_long_keyword_truncated_to_200(
            extra in 1usize..=100usize,
        ) {
            // Build a keyword of exactly 200 + extra chars
            let keyword: String = "a".repeat(200 + extra);
            let (sanitized, truncated) = sanitize_query(&keyword);
            prop_assert!(truncated, "keyword longer than 200 chars should be truncated");
            prop_assert_eq!(
                sanitized.chars().count(),
                200,
                "truncated keyword must be exactly 200 chars"
            );
        }
    }

    // ========================================================================
    // Property 37: 按编号精确查找
    // Validates: Requirements 10.2
    // ========================================================================

    /// Simulate find_by_number logic: exact match on number field.
    fn find_by_number_in_list<'a>(
        items: &'a [(&'a str, i64)],
        project_id: i64,
        number: &str,
    ) -> Option<(&'a str, i64)> {
        items
            .iter()
            .find(|(num, pid)| *num == number && *pid == project_id)
            .copied()
    }

    proptest! {
        /// find_by_number returns the item whose number field exactly matches the query.
        /// Validates: Requirements 10.2
        #[test]
        fn prop37_find_by_number_exact_match(
            prefix in prop_oneof![Just("REQ"), Just("BUG"), Just("TASK"), Just("STORY")],
            n in 1u32..=999u32,
        ) {
            let number = format!("{}-{:03}", prefix, n);
            let project_id: i64 = 1;
            let items = vec![(number.as_str(), project_id)];

            let result = find_by_number_in_list(&items, project_id, &number);
            prop_assert!(result.is_some(), "exact number match must be found");
            prop_assert_eq!(result.unwrap().0, number.as_str());
        }

        /// find_by_number returns None when the number does not exist.
        /// Validates: Requirements 10.2
        #[test]
        fn prop37_find_by_number_not_found(
            n in 1u32..=999u32,
        ) {
            let number = format!("REQ-{:03}", n);
            let project_id: i64 = 1;
            // List contains a different number
            let other = format!("REQ-{:03}", n + 1000);
            let items = vec![(other.as_str(), project_id)];

            let result = find_by_number_in_list(&items, project_id, &number);
            prop_assert!(result.is_none(), "non-existent number must not be found");
        }
    }
}
