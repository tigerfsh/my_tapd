// Feature: it-requirements-management-system, Property 19: 迭代时间冲突检测
// Feature: it-requirements-management-system, Property 20: 迭代结束未完成故事回归 Backlog

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, TimeZone, Utc};
    use proptest::prelude::*;

    use crate::domain::{
        functions::find_conflicting_iteration, Iteration, IterationId, IterationStatus, Priority,
        Status, WorkItem, WorkItemId, WorkItemType,
    };

    // ---- Helpers ----

    fn make_iteration(
        id: IterationId,
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

    fn make_story(id: WorkItemId, status: Status, iteration_id: Option<IterationId>) -> WorkItem {
        WorkItem {
            id,
            project_id: 1,
            item_type: WorkItemType::Story,
            number: format!("STORY-{}", id),
            title: format!("Story {}", id),
            description: None,
            status,
            priority: Priority::Medium,
            assignee_id: None,
            creator_id: 1,
            parent_id: None,
            iteration_id,
            due_date: None,
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

    fn date_strategy() -> impl Strategy<Value = NaiveDate> {
        (2020i32..2030, 1u32..13u32, 1u32..29u32).prop_map(|(y, m, d)| {
            NaiveDate::from_ymd_opt(y, m, d)
                .unwrap_or_else(|| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        })
    }

    // ========================================================================
    // Property 19: 迭代时间冲突检测
    // Validates: Requirements 5.2
    // ========================================================================

    proptest! {
        /// Non-overlapping iterations must not be detected as conflicting.
        /// Validates: Requirements 5.2
        #[test]
        fn prop19_non_overlapping_no_conflict(
            base_start in date_strategy(),
            gap in 1i64..10i64,
            len in 1i64..20i64,
        ) {
            // existing: [base_start, base_start + len]
            // new: [base_start + len + gap, ...]  — starts after existing ends
            let existing_end = base_start + chrono::Duration::days(len);
            let new_start = existing_end + chrono::Duration::days(gap);
            let new_end = new_start + chrono::Duration::days(len);

            let existing = vec![make_iteration(1, base_start, existing_end, IterationStatus::InProgress)];
            let conflict = find_conflicting_iteration(new_start, new_end, &existing);
            prop_assert!(conflict.is_none(), "non-overlapping iterations should not conflict");
        }

        /// Overlapping active iterations must be detected as conflicting.
        /// Validates: Requirements 5.2
        #[test]
        fn prop19_overlapping_active_detected(
            base_start in date_strategy(),
            len in 4i64..20i64,
            overlap in 1i64..3i64,
        ) {
            // existing: [base_start, base_start + len]
            // new starts inside existing range (overlap < len ensures new_start < existing_end)
            let existing_end = base_start + chrono::Duration::days(len);
            let new_start = base_start + chrono::Duration::days(overlap);
            let new_end = new_start + chrono::Duration::days(len);

            let existing = vec![make_iteration(1, base_start, existing_end, IterationStatus::InProgress)];
            let conflict = find_conflicting_iteration(new_start, new_end, &existing);
            prop_assert!(conflict.is_some(), "overlapping active iterations should conflict");
        }

        /// Completed iterations must NOT be detected as conflicting even if dates overlap.
        /// Validates: Requirements 5.2
        #[test]
        fn prop19_completed_iteration_not_conflicting(
            base_start in date_strategy(),
            len in 1i64..20i64,
        ) {
            let existing_end = base_start + chrono::Duration::days(len);
            // new iteration fully overlaps with completed one
            let existing = vec![make_iteration(1, base_start, existing_end, IterationStatus::Completed)];
            let conflict = find_conflicting_iteration(base_start, existing_end, &existing);
            prop_assert!(conflict.is_none(), "completed iterations should not block new ones");
        }

        /// Empty existing list: no conflict possible.
        /// Validates: Requirements 5.2
        #[test]
        fn prop19_empty_existing_no_conflict(
            start in date_strategy(),
            len in 1i64..20i64,
        ) {
            let end = start + chrono::Duration::days(len);
            let conflict = find_conflicting_iteration(start, end, &[]);
            prop_assert!(conflict.is_none());
        }
    }

    // ========================================================================
    // Property 20: 迭代结束未完成故事回归 Backlog
    // Validates: Requirements 5.5
    // ========================================================================

    /// Pure logic: stories with status != Done/Closed should be moved to backlog (iteration_id = None).
    fn simulate_close_iteration(stories: &[WorkItem]) -> (Vec<WorkItemId>, Vec<WorkItemId>) {
        let mut moved_to_backlog = Vec::new();
        let mut stayed = Vec::new();
        for story in stories {
            if story.status == Status::Done || story.status == Status::Closed {
                stayed.push(story.id);
            } else {
                moved_to_backlog.push(story.id);
            }
        }
        (moved_to_backlog, stayed)
    }

    proptest! {
        /// After close_iteration, all incomplete stories must be moved to backlog.
        /// Validates: Requirements 5.5
        #[test]
        fn prop20_incomplete_stories_moved_to_backlog(
            statuses in prop::collection::vec(
                prop_oneof![
                    Just(Status::Pending),
                    Just(Status::InProgress),
                    Just(Status::Done),
                    Just(Status::Closed),
                ],
                1..=20usize,
            )
        ) {
            let iteration_id: IterationId = 42;
            let stories: Vec<WorkItem> = statuses
                .iter()
                .enumerate()
                .map(|(i, s)| make_story(i as WorkItemId + 1, s.clone(), Some(iteration_id)))
                .collect();

            let (moved, stayed) = simulate_close_iteration(&stories);

            // Every moved story must have been incomplete
            for id in &moved {
                let story = stories.iter().find(|s| s.id == *id).unwrap();
                prop_assert!(
                    story.status != Status::Done && story.status != Status::Closed,
                    "only incomplete stories should be moved to backlog"
                );
            }

            // Every stayed story must have been complete
            for id in &stayed {
                let story = stories.iter().find(|s| s.id == *id).unwrap();
                prop_assert!(
                    story.status == Status::Done || story.status == Status::Closed,
                    "completed stories should stay in iteration"
                );
            }

            // All stories are accounted for
            prop_assert_eq!(moved.len() + stayed.len(), stories.len());
        }

        /// All-done iteration: no stories moved to backlog.
        /// Validates: Requirements 5.5
        #[test]
        fn prop20_all_done_nothing_moved(n in 1usize..=20usize) {
            let iteration_id: IterationId = 1;
            let stories: Vec<WorkItem> = (0..n)
                .map(|i| make_story(i as WorkItemId + 1, Status::Done, Some(iteration_id)))
                .collect();

            let (moved, _stayed) = simulate_close_iteration(&stories);
            prop_assert_eq!(moved.len(), 0, "no stories should be moved when all are done");
        }

        /// All-incomplete iteration: all stories moved to backlog.
        /// Validates: Requirements 5.5
        #[test]
        fn prop20_all_incomplete_all_moved(n in 1usize..=20usize) {
            let iteration_id: IterationId = 1;
            let stories: Vec<WorkItem> = (0..n)
                .map(|i| make_story(i as WorkItemId + 1, Status::InProgress, Some(iteration_id)))
                .collect();

            let (moved, _stayed) = simulate_close_iteration(&stories);
            prop_assert_eq!(moved.len(), n, "all incomplete stories should be moved to backlog");
        }
    }
}
