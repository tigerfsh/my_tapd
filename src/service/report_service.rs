use std::collections::HashMap;

use serde::Serialize;
use sqlx::PgPool;

use crate::domain::{IterationId, ProjectId};
use crate::error::AppError;

// ---- Report structs ----

#[derive(Debug, Serialize)]
pub struct RequirementReport {
    pub total: i64,
    pub completed: i64,
    pub completion_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct BugReport {
    pub total: i64,
    pub new_count: i64,
    pub fixed_count: i64,
    pub remaining_count: i64,
    pub by_severity: HashMap<String, i64>,
}

#[derive(Debug, Serialize)]
pub struct MemberWorkload {
    pub user_id: i64,
    pub completed_count: i64,
    pub total_hours: f64,
}

#[derive(Debug, Serialize)]
pub struct DashboardData {
    pub requirement_report: RequirementReport,
    pub bug_report: BugReport,
}

// ---- Service ----

pub struct ReportService {
    pool: PgPool,
}

impl ReportService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// 需求 11.1：需求完成率报告
    pub async fn requirement_completion_report(
        &self,
        project_id: ProjectId,
        _iteration_id: Option<IterationId>,
    ) -> Result<RequirementReport, AppError> {
        let rows = sqlx::query_as::<_, (String, i64)>(
            "SELECT status::text, COUNT(*) FROM work_items \
             WHERE project_id = $1 AND item_type = 'requirement' \
             GROUP BY status",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        let mut total: i64 = 0;
        let mut completed: i64 = 0;
        for (status, count) in &rows {
            total += count;
            if status == "done" || status == "closed" {
                completed += count;
            }
        }

        let completion_rate = if total > 0 {
            completed as f64 / total as f64 * 100.0
        } else {
            0.0
        };

        Ok(RequirementReport {
            total,
            completed,
            completion_rate,
        })
    }

    /// 需求 11.2：Bug 统计报告
    pub async fn bug_stats_report(&self, project_id: ProjectId) -> Result<BugReport, AppError> {
        // Count by status
        let status_rows = sqlx::query_as::<_, (String, i64)>(
            "SELECT status::text, COUNT(*) FROM work_items \
             WHERE project_id = $1 AND item_type = 'bug' \
             GROUP BY status",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        let mut total: i64 = 0;
        let mut new_count: i64 = 0;
        let mut fixed_count: i64 = 0;
        let mut remaining_count: i64 = 0;

        for (status, count) in &status_rows {
            total += count;
            match status.as_str() {
                "pending" => new_count += count,
                "closed" => fixed_count += count,
                _ => remaining_count += count,
            }
        }

        // Count by severity
        let severity_rows = sqlx::query_as::<_, (String, i64)>(
            "SELECT COALESCE(severity::text, 'unknown'), COUNT(*) FROM work_items \
             WHERE project_id = $1 AND item_type = 'bug' \
             GROUP BY severity",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        let by_severity: HashMap<String, i64> = severity_rows.into_iter().collect();

        Ok(BugReport {
            total,
            new_count,
            fixed_count,
            remaining_count,
            by_severity,
        })
    }

    /// 需求 11.3：成员工作量报告
    pub async fn member_workload_report(
        &self,
        project_id: ProjectId,
    ) -> Result<Vec<MemberWorkload>, AppError> {
        let rows = sqlx::query_as::<_, (i64, i64, f64)>(
            "SELECT assignee_id, COUNT(*) as completed_count, \
             COALESCE(SUM(actual_hours), 0) as total_hours \
             FROM work_items \
             WHERE project_id = $1 AND status = 'done' AND assignee_id IS NOT NULL \
             GROUP BY assignee_id",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        let workloads = rows
            .into_iter()
            .map(|(user_id, completed_count, total_hours)| MemberWorkload {
                user_id,
                completed_count,
                total_hours,
            })
            .collect();

        Ok(workloads)
    }

    /// 需求 11.4：仪表盘数据聚合
    pub async fn dashboard_data(&self, project_id: ProjectId) -> Result<DashboardData, AppError> {
        let requirement_report = self
            .requirement_completion_report(project_id, None)
            .await?;
        let bug_report = self.bug_stats_report(project_id).await?;

        Ok(DashboardData {
            requirement_report,
            bug_report,
        })
    }

    /// 需求 11.5：导出报告为 CSV
    pub async fn export_report(
        &self,
        project_id: ProjectId,
        report_type: &str,
    ) -> Result<Vec<u8>, AppError> {
        let csv = match report_type {
            "requirements" => {
                let report = self
                    .requirement_completion_report(project_id, None)
                    .await?;
                format!(
                    "total,completed,completion_rate\n{},{},{:.2}\n",
                    report.total, report.completed, report.completion_rate
                )
            }
            "bugs" => {
                let report = self.bug_stats_report(project_id).await?;
                format!(
                    "total,new,fixed,remaining\n{},{},{},{}\n",
                    report.total, report.new_count, report.fixed_count, report.remaining_count
                )
            }
            "workload" => {
                let workloads = self.member_workload_report(project_id).await?;
                let mut csv = "user_id,completed_count,total_hours\n".to_string();
                for w in workloads {
                    csv.push_str(&format!(
                        "{},{},{:.1}\n",
                        w.user_id, w.completed_count, w.total_hours
                    ));
                }
                csv
            }
            _ => {
                return Err(AppError::NotFound);
            }
        };

        Ok(csv.into_bytes())
    }
}
