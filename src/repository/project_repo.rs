use sqlx::PgPool;

use crate::domain::{Member, Project, ProjectId, ProjectType, Role, UserId};
use crate::error::AppError;

pub struct ProjectRepo {
    pool: PgPool,
}

impl ProjectRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        name: &str,
        description: Option<&str>,
        project_type: ProjectType,
        is_public: bool,
        created_by: UserId,
    ) -> Result<Project, AppError> {
        let project = sqlx::query_as::<_, Project>(
            "INSERT INTO projects (name, description, project_type, is_public, created_by) \
             VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(name)
        .bind(description)
        .bind(project_type)
        .bind(is_public)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await?;
        Ok(project)
    }

    /// List all projects the user is a member of (or public projects).
    pub async fn list_by_user(&self, user_id: UserId) -> Result<Vec<Project>, AppError> {
        let projects = sqlx::query_as::<_, Project>(
            "SELECT p.* FROM projects p \
             JOIN project_members pm ON pm.project_id = p.id \
             WHERE pm.user_id = $1 AND p.is_archived = false \
             ORDER BY p.created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(projects)
    }

    pub async fn find_by_id(&self, id: ProjectId) -> Result<Option<Project>, AppError> {
        let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(project)
    }

    pub async fn update(
        &self,
        id: ProjectId,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<Project, AppError> {
        let project = sqlx::query_as::<_, Project>(
            "UPDATE projects SET \
             name = COALESCE($2, name), \
             description = COALESCE($3, description), \
             updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;
        Ok(project)
    }

    pub async fn archive(&self, id: ProjectId) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE projects SET is_archived = true, updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn add_member(
        &self,
        project_id: ProjectId,
        user_id: UserId,
        role: Role,
    ) -> Result<Member, AppError> {
        let member = sqlx::query_as::<_, Member>(
            "INSERT INTO project_members (project_id, user_id, role) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(project_id)
        .bind(user_id)
        .bind(role)
        .fetch_one(&self.pool)
        .await?;
        Ok(member)
    }

    pub async fn get_member(
        &self,
        project_id: ProjectId,
        user_id: UserId,
    ) -> Result<Option<Member>, AppError> {
        let member = sqlx::query_as::<_, Member>(
            "SELECT * FROM project_members WHERE project_id = $1 AND user_id = $2",
        )
        .bind(project_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(member)
    }

    pub async fn list_members(&self, project_id: ProjectId) -> Result<Vec<Member>, AppError> {
        let members =
            sqlx::query_as::<_, Member>("SELECT * FROM project_members WHERE project_id = $1")
                .bind(project_id)
                .fetch_all(&self.pool)
                .await?;
        Ok(members)
    }

    pub async fn update_member_role(
        &self,
        project_id: ProjectId,
        user_id: UserId,
        role: Role,
    ) -> Result<Member, AppError> {
        let member = sqlx::query_as::<_, Member>(
            "UPDATE project_members SET role = $3 WHERE project_id = $1 AND user_id = $2 RETURNING *",
        )
        .bind(project_id)
        .bind(user_id)
        .bind(role)
        .fetch_one(&self.pool)
        .await?;
        Ok(member)
    }

    pub async fn remove_member(
        &self,
        project_id: ProjectId,
        user_id: UserId,
    ) -> Result<(), AppError> {
        sqlx::query(
            "DELETE FROM project_members WHERE project_id = $1 AND user_id = $2",
        )
        .bind(project_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
