use sqlx::{PgPool, QueryBuilder};

use crate::domain::{
    functions::sanitize_query, ProjectId, SearchQuery, SearchResult, UserId, WorkItem,
};
use crate::error::AppError;

pub struct SearchService {
    pool: PgPool,
}

impl SearchService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// 需求 10.1–10.5：全文搜索工作项，支持关键词截断和可选过滤器
    pub async fn search(
        &self,
        _requester_id: UserId,
        project_id: ProjectId,
        query: SearchQuery,
    ) -> Result<SearchResult, AppError> {
        let (sanitized, _) = sanitize_query(&query.keyword);

        // Base full-text search with ts_rank
        let mut qb = QueryBuilder::new(
            "SELECT *, ts_rank(to_tsvector('simple', title || ' ' || COALESCE(description, '')), \
             plainto_tsquery('simple', ",
        );
        qb.push_bind(sanitized);
        qb.push(")) AS rank FROM work_items WHERE project_id = ");
        qb.push_bind(project_id);
        qb.push(
            " AND to_tsvector('simple', title || ' ' || COALESCE(description, '')) \
             @@ plainto_tsquery('simple', ",
        );
        // Re-bind sanitized — need to re-sanitize since we can't reuse the bind
        let (sanitized2, _) = sanitize_query(&query.keyword);
        qb.push_bind(sanitized2);
        qb.push(")");

        if let Some(item_type) = query.item_type {
            qb.push(" AND item_type = ");
            qb.push_bind(item_type);
        }
        if let Some(status) = query.status {
            qb.push(" AND status = ");
            qb.push_bind(status);
        }
        if let Some(priority) = query.priority {
            qb.push(" AND priority = ");
            qb.push_bind(priority);
        }

        qb.push(" ORDER BY rank DESC");

        let items = qb
            .build_query_as::<WorkItem>()
            .fetch_all(&self.pool)
            .await?;

        let total = items.len() as i64;
        Ok(SearchResult { items, total })
    }

    /// 需求 10.2：按编号精确查找工作项
    pub async fn find_by_number(
        &self,
        _requester_id: UserId,
        project_id: ProjectId,
        number: &str,
    ) -> Result<WorkItem, AppError> {
        let item = sqlx::query_as::<_, WorkItem>(
            "SELECT * FROM work_items WHERE project_id = $1 AND number = $2",
        )
        .bind(project_id)
        .bind(number)
        .fetch_optional(&self.pool)
        .await?;

        item.ok_or(AppError::NotFound)
    }
}
