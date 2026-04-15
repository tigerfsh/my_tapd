use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post, put},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::api::{auth, iterations, notifications, projects, reports, search, work_items};
use crate::ws::handler::{WsState, ws_handler};

/// Shared application state passed to all handlers.
#[derive(Clone)]
pub struct AppState {
    pub pg_pool: sqlx::PgPool,
    pub redis: redis::aio::ConnectionManager,
    pub jwt_secret: String,
}

pub fn create_router(state: AppState) -> Router {
    let shared = Arc::new(state.clone());
    let ws_state = Arc::new(WsState {
        redis: state.redis.clone(),
        jwt_secret: state.jwt_secret.clone(),
    });

    let api = Router::new()
        // ---- Auth ----
        .route("/auth/register", post(auth::register_handler))
        .route("/auth/verify-email", post(auth::verify_email_handler))
        .route("/auth/login", post(auth::login_handler))
        .route("/auth/logout", post(auth::logout_handler))
        .route("/auth/password-reset/request", post(auth::request_reset_handler))
        .route("/auth/password-reset/confirm", post(auth::confirm_reset_handler))
        .route("/users/me", get(auth::get_me_handler).put(auth::update_me_handler))
        .route("/users/me/notification-preferences",
            get(notifications::get_preferences_handler)
                .put(notifications::update_preferences_handler),
        )
        // ---- Projects ----
        .route("/projects",
            get(projects::list_projects_handler).post(projects::create_project_handler),
        )
        .route("/projects/:id", get(projects::get_project_handler).put(projects::update_project_handler))
        .route("/projects/:id/archive", post(projects::archive_project_handler))
        .route("/projects/:id/members",
            get(projects::list_members_handler).post(projects::invite_member_handler),
        )
        .route("/projects/:id/members/:uid",
            put(projects::update_member_handler).delete(projects::remove_member_handler),
        )
        // ---- Work Items ----
        .route("/projects/:pid/requirements",
            post(work_items::create_requirement_handler)
                .get(work_items::list_requirements_handler),
        )
        .route("/projects/:pid/requirements/:id",
            get(work_items::get_work_item_handler)
                .put(work_items::update_work_item_handler),
        )
        .route("/projects/:pid/requirements/:id/history", get(work_items::get_history_handler))
        .route("/requirements/:id/stories", post(work_items::create_story_handler))
        .route("/stories/:id/tasks", post(work_items::create_task_handler))
        .route("/projects/:pid/bugs",
            post(work_items::create_bug_handler).get(work_items::list_bugs_handler),
        )
        .route("/work-items/:id/status", put(work_items::update_status_handler))
        .route("/work-items/:id/assign", put(work_items::assign_handler))
        .route("/work-items/:id/comments", post(work_items::add_comment_handler))
        .route("/work-items/:id/attachments", post(work_items::upload_attachment_handler))
        .route("/tasks/:id/actual-hours", put(work_items::log_actual_hours_handler))
        // ---- Iterations ----
        .route("/projects/:pid/iterations",
            post(iterations::create_iteration_handler)
                .get(iterations::list_iterations_handler),
        )
        .route("/iterations/:id",
            put(iterations::update_iteration_handler),
        )
        .route("/iterations/:id/stories", post(iterations::assign_stories_handler))
        .route("/iterations/:id/close", post(iterations::close_iteration_handler))
        .route("/iterations/:id/burndown", get(iterations::get_burndown_handler))
        .route("/iterations/:id/stats", get(iterations::get_stats_handler))
        // ---- Notifications ----
        .route("/notifications", get(notifications::list_notifications_handler))
        .route("/notifications/read", post(notifications::mark_read_handler))
        .route("/notifications/read-all", post(notifications::mark_all_read_handler))
        // ---- Reports ----
        .route("/projects/:pid/dashboard", get(reports::dashboard_handler))
        .route("/projects/:pid/reports/requirements", get(reports::requirements_report_handler))
        .route("/projects/:pid/reports/bugs", get(reports::bugs_report_handler))
        .route("/projects/:pid/reports/members", get(reports::members_report_handler))
        .route("/projects/:pid/reports/export", post(reports::export_report_handler))
        // ---- Search ----
        .route("/projects/:pid/search", get(search::search_handler))
        .route("/projects/:pid/work-items/:number", get(search::find_by_number_handler))
        .with_state(shared);

    Router::new()
        .nest("/api/v1", api)
        .route("/ws", get(ws_handler).with_state(ws_state))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
