use chrono::{DateTime, Duration, NaiveDate, Utc};
use std::collections::HashMap;

use crate::domain::{Iteration, IterationStatus, Status, WorkItem, WorkItemId};
use crate::domain::user::User;
use crate::error::AppError;

// ---- 账户锁定 ----

/// 需求 1.4：连续 5 次失败锁定 15 分钟
pub fn check_account_lock(user: &User, now: DateTime<Utc>) -> Result<(), AppError> {
    if let Some(locked_until) = user.locked_until {
        if now < locked_until {
            return Err(AppError::AccountLocked { until: locked_until });
        }
    }
    Ok(())
}

pub fn should_lock_account(fail_count: i32) -> Option<Duration> {
    if fail_count >= 5 {
        Some(Duration::minutes(15))
    } else {
        None
    }
}

// ---- 令牌有效期 ----

/// 需求 1.5：重置令牌有效期 30 分钟
pub fn is_token_valid(issued_at: DateTime<Utc>, now: DateTime<Utc>, ttl: Duration) -> bool {
    now < issued_at + ttl
}

// ---- 迭代时间冲突 ----

/// 需求 5.2：检测迭代时间范围重叠
pub fn iterations_overlap(a: &Iteration, b: &Iteration) -> bool {
    a.start_date <= b.end_date && b.start_date <= a.end_date
}

pub fn find_conflicting_iteration<'a>(
    new_start: NaiveDate,
    new_end: NaiveDate,
    existing: &'a [Iteration],
) -> Option<&'a Iteration> {
    existing.iter().find(|it| {
        it.status != IterationStatus::Completed
            && new_start <= it.end_date
            && it.start_date <= new_end
    })
}

// ---- Story 完成百分比 ----

/// 需求 4.3：完成百分比 = 已完成 Task 数 / 总 Task 数 × 100
pub fn calc_completion_pct(tasks: &[WorkItem]) -> i32 {
    if tasks.is_empty() {
        return 0;
    }
    let done = tasks.iter().filter(|t| t.status == Status::Done).count();
    ((done as f64 / tasks.len() as f64) * 100.0).round() as i32
}

// ---- 工时精度 ----

/// 需求 4.5：实际工时精度为 0.5 小时
pub fn is_valid_hours(hours: f32) -> bool {
    hours > 0.0 && (hours * 2.0).fract() == 0.0
}

// ---- Bug 状态机 ----

/// 需求 6.3：Bug 状态流转路径
pub fn is_valid_bug_transition(from: &Status, to: &Status) -> bool {
    matches!(
        (from, to),
        (Status::Pending, Status::Fixing)
            | (Status::Fixing, Status::PendingVerify)
            | (Status::PendingVerify, Status::Closed)
            | (Status::PendingVerify, Status::Rejected)
            | (Status::Closed, Status::Pending)
    )
}

// ---- 搜索关键词截断 ----

/// 需求 10.5：关键词超过 200 字符时截断
pub fn sanitize_query(query: &str) -> (String, bool) {
    let max_chars = 200;
    let chars: Vec<char> = query.chars().collect();
    if chars.len() > max_chars {
        (chars[..max_chars].iter().collect(), true)
    } else {
        (query.to_string(), false)
    }
}

// ---- 附件大小校验 ----

/// 需求 6.7：单个附件不超过 20MB
pub const MAX_ATTACHMENT_BYTES: i64 = 20 * 1024 * 1024;

pub fn validate_attachment_size(size_bytes: i64) -> Result<(), AppError> {
    if size_bytes > MAX_ATTACHMENT_BYTES {
        Err(AppError::AttachmentTooLarge {
            max: MAX_ATTACHMENT_BYTES,
            actual: size_bytes,
        })
    } else {
        Ok(())
    }
}

// ---- 通知重试 ----

/// 需求 8.7：推送失败最多重试 3 次
pub const MAX_NOTIFY_RETRIES: i32 = 3;

pub fn should_retry_notification(retry_count: i32) -> bool {
    retry_count < MAX_NOTIFY_RETRIES
}

// ---- 逾期检测 ----

/// 需求 3.7：截止日期已过且未完成的需求标记为逾期
pub fn is_overdue(item: &WorkItem, today: NaiveDate) -> bool {
    matches!(item.due_date, Some(d) if d < today)
        && item.status != Status::Done
        && item.status != Status::Closed
}

// ---- 泳道分组 ----

/// 需求 7.6：按负责人或 Label 对工作项进行泳道分组
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwimlaneDimension {
    Assignee,
    Label,
}

pub fn group_by_swimlane<'a>(
    items: &'a [WorkItem],
    labels: &HashMap<WorkItemId, Vec<String>>,
    dimension: SwimlaneDimension,
) -> HashMap<String, Vec<&'a WorkItem>> {
    let mut groups: HashMap<String, Vec<&'a WorkItem>> = HashMap::new();
    for item in items {
        let keys: Vec<String> = match dimension {
            SwimlaneDimension::Assignee => {
                vec![item
                    .assignee_id
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "unassigned".into())]
            }
            SwimlaneDimension::Label => labels.get(&item.id).cloned().unwrap_or_default(),
        };
        for key in keys {
            groups.entry(key).or_default().push(item);
        }
    }
    groups
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use chrono::{TimeZone, Utc};

    use crate::domain::{Priority, Status, WorkItem, WorkItemId, WorkItemType};
    use super::{is_valid_bug_transition, is_overdue, validate_attachment_size, MAX_ATTACHMENT_BYTES};

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

    // ---- Bug 状态机合法与非法转换（需求 6.3）----

    #[test]
    fn bug_transition_pending_to_fixing_valid() {
        assert!(is_valid_bug_transition(&Status::Pending, &Status::Fixing));
    }

    #[test]
    fn bug_transition_fixing_to_pending_verify_valid() {
        assert!(is_valid_bug_transition(&Status::Fixing, &Status::PendingVerify));
    }

    #[test]
    fn bug_transition_pending_verify_to_closed_valid() {
        assert!(is_valid_bug_transition(&Status::PendingVerify, &Status::Closed));
    }

    #[test]
    fn bug_transition_pending_verify_to_rejected_valid() {
        assert!(is_valid_bug_transition(&Status::PendingVerify, &Status::Rejected));
    }

    #[test]
    fn bug_transition_closed_to_pending_valid() {
        assert!(is_valid_bug_transition(&Status::Closed, &Status::Pending));
    }

    #[test]
    fn bug_transition_pending_to_done_invalid() {
        assert!(!is_valid_bug_transition(&Status::Pending, &Status::Done));
    }

    #[test]
    fn bug_transition_done_to_fixing_invalid() {
        assert!(!is_valid_bug_transition(&Status::Done, &Status::Fixing));
    }

    #[test]
    fn bug_transition_in_progress_to_closed_invalid() {
        assert!(!is_valid_bug_transition(&Status::InProgress, &Status::Closed));
    }

    #[test]
    fn bug_transition_fixing_to_pending_invalid() {
        assert!(!is_valid_bug_transition(&Status::Fixing, &Status::Pending));
    }

    #[test]
    fn bug_transition_closed_to_done_invalid() {
        assert!(!is_valid_bug_transition(&Status::Closed, &Status::Done));
    }

    // ---- 逾期检测边界条件（需求 3.7）----

    #[test]
    fn overdue_due_date_equals_today_not_overdue() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let item = make_work_item(1, Status::InProgress, None, Some(today));
        assert!(!is_overdue(&item, today));
    }

    #[test]
    fn overdue_due_date_yesterday_active_status_is_overdue() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let yesterday = today - chrono::Duration::days(1);
        let item = make_work_item(1, Status::InProgress, None, Some(yesterday));
        assert!(is_overdue(&item, today));
    }

    #[test]
    fn overdue_done_status_with_past_due_not_overdue() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let past = today - chrono::Duration::days(10);
        let item = make_work_item(1, Status::Done, None, Some(past));
        assert!(!is_overdue(&item, today));
    }

    #[test]
    fn overdue_closed_status_with_past_due_not_overdue() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let past = today - chrono::Duration::days(10);
        let item = make_work_item(1, Status::Closed, None, Some(past));
        assert!(!is_overdue(&item, today));
    }

    #[test]
    fn overdue_no_due_date_not_overdue() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let item = make_work_item(1, Status::InProgress, None, None);
        assert!(!is_overdue(&item, today));
    }

    // ---- 附件大小边界值（需求 6.7）----

    #[test]
    fn attachment_size_exactly_max_is_ok() {
        assert!(validate_attachment_size(MAX_ATTACHMENT_BYTES).is_ok());
    }

    #[test]
    fn attachment_size_max_plus_one_is_err() {
        assert!(validate_attachment_size(MAX_ATTACHMENT_BYTES + 1).is_err());
    }

    #[test]
    fn attachment_size_zero_is_ok() {
        assert!(validate_attachment_size(0).is_ok());
    }

    #[test]
    fn attachment_size_one_is_ok() {
        assert!(validate_attachment_size(1).is_ok());
    }
}
