-- ============================================================
-- IT需求管理系统 初始数据库迁移
-- ============================================================

-- ---- 枚举类型 ----

CREATE TYPE work_item_type AS ENUM (
    'requirement',
    'story',
    'task',
    'bug'
);

CREATE TYPE status AS ENUM (
    'pending',
    'in_progress',
    'done',
    'closed',
    'rejected',
    'pending_verify',
    'fixing',
    'unassigned'
);

CREATE TYPE priority AS ENUM (
    'urgent',
    'high',
    'medium',
    'low'
);

CREATE TYPE role AS ENUM (
    'admin',
    'developer',
    'tester',
    'observer'
);

CREATE TYPE project_type AS ENUM (
    'agile',
    'waterfall'
);

CREATE TYPE severity AS ENUM (
    'fatal',
    'critical',
    'normal',
    'hint'
);

CREATE TYPE iteration_status AS ENUM (
    'not_started',
    'in_progress',
    'completed'
);

-- ---- 表结构 ----

CREATE TABLE IF NOT EXISTS users (
    id                BIGSERIAL       PRIMARY KEY,
    email             VARCHAR(255)    NOT NULL UNIQUE,
    password_hash     VARCHAR(255)    NOT NULL,
    nickname          VARCHAR(100)    NOT NULL,
    avatar_url        TEXT,
    phone             VARCHAR(20),
    is_active         BOOLEAN         NOT NULL DEFAULT FALSE,
    login_fail_count  INT             NOT NULL DEFAULT 0,
    locked_until      TIMESTAMPTZ,
    created_at        TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS projects (
    id           BIGSERIAL       PRIMARY KEY,
    name         VARCHAR(255)    NOT NULL,
    description  TEXT,
    project_type project_type    NOT NULL,
    is_public    BOOLEAN         NOT NULL DEFAULT FALSE,
    is_archived  BOOLEAN         NOT NULL DEFAULT FALSE,
    created_by   BIGINT          NOT NULL REFERENCES users(id),
    created_at   TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS project_members (
    project_id  BIGINT      NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id     BIGINT      NOT NULL REFERENCES users(id)    ON DELETE CASCADE,
    role        role        NOT NULL,
    joined_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (project_id, user_id)
);

CREATE TABLE IF NOT EXISTS iterations (
    id          BIGSERIAL        PRIMARY KEY,
    project_id  BIGINT           NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name        VARCHAR(255)     NOT NULL,
    goal        TEXT,
    start_date  DATE             NOT NULL,
    end_date    DATE             NOT NULL,
    status      iteration_status NOT NULL DEFAULT 'not_started',
    created_by  BIGINT           NOT NULL REFERENCES users(id),
    created_at  TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ      NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS work_items (
    id               BIGSERIAL       PRIMARY KEY,
    project_id       BIGINT          NOT NULL REFERENCES projects(id)   ON DELETE CASCADE,
    item_type        work_item_type  NOT NULL,
    number           VARCHAR(50)     NOT NULL,
    title            VARCHAR(500)    NOT NULL,
    description      TEXT,
    status           status          NOT NULL DEFAULT 'pending',
    priority         priority        NOT NULL DEFAULT 'medium',
    assignee_id      BIGINT          REFERENCES users(id)       ON DELETE SET NULL,
    creator_id       BIGINT          NOT NULL REFERENCES users(id),
    parent_id        BIGINT          REFERENCES work_items(id)  ON DELETE SET NULL,
    iteration_id     BIGINT          REFERENCES iterations(id)  ON DELETE SET NULL,
    due_date         DATE,
    story_points     INT,
    estimated_hours  FLOAT,
    actual_hours     FLOAT,
    severity         severity,
    repro_steps      TEXT,
    reopen_reason    TEXT,
    completion_pct   INT,
    created_at       TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    UNIQUE (project_id, number)
);

CREATE TABLE IF NOT EXISTS work_item_labels (
    work_item_id  BIGINT       NOT NULL REFERENCES work_items(id) ON DELETE CASCADE,
    label         VARCHAR(100) NOT NULL,
    PRIMARY KEY (work_item_id, label)
);

CREATE TABLE IF NOT EXISTS comments (
    id            BIGSERIAL   PRIMARY KEY,
    work_item_id  BIGINT      NOT NULL REFERENCES work_items(id) ON DELETE CASCADE,
    author_id     BIGINT      NOT NULL REFERENCES users(id),
    content       TEXT        NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS attachments (
    id            BIGSERIAL    PRIMARY KEY,
    work_item_id  BIGINT       NOT NULL REFERENCES work_items(id) ON DELETE CASCADE,
    uploader_id   BIGINT       NOT NULL REFERENCES users(id),
    filename      VARCHAR(255) NOT NULL,
    file_size     BIGINT       NOT NULL,
    storage_key   TEXT         NOT NULL,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS audit_logs (
    id            BIGSERIAL    PRIMARY KEY,
    work_item_id  BIGINT       NOT NULL REFERENCES work_items(id) ON DELETE CASCADE,
    operator_id   BIGINT       NOT NULL REFERENCES users(id),
    field_name    VARCHAR(100) NOT NULL,
    old_value     TEXT,
    new_value     TEXT,
    changed_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS notifications (
    id            BIGSERIAL    PRIMARY KEY,
    user_id       BIGINT       NOT NULL REFERENCES users(id)       ON DELETE CASCADE,
    event_type    VARCHAR(100) NOT NULL,
    work_item_id  BIGINT       REFERENCES work_items(id)           ON DELETE SET NULL,
    content       TEXT         NOT NULL,
    is_read       BOOLEAN      NOT NULL DEFAULT FALSE,
    retry_count   INT          NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS notification_preferences (
    user_id           BIGINT  PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    on_assigned       BOOLEAN NOT NULL DEFAULT TRUE,
    on_status_change  BOOLEAN NOT NULL DEFAULT TRUE,
    on_comment        BOOLEAN NOT NULL DEFAULT TRUE,
    on_due_date       BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS burndown_snapshots (
    id                BIGSERIAL PRIMARY KEY,
    iteration_id      BIGINT    NOT NULL REFERENCES iterations(id) ON DELETE CASCADE,
    snapshot_date     DATE      NOT NULL,
    remaining_points  INT       NOT NULL,
    total_points      INT       NOT NULL
);

-- ---- 索引 ----

CREATE INDEX idx_work_items_project_type   ON work_items(project_id, item_type);
CREATE INDEX idx_work_items_assignee       ON work_items(assignee_id);
CREATE INDEX idx_work_items_iteration      ON work_items(iteration_id);
CREATE INDEX idx_work_items_parent         ON work_items(parent_id);
CREATE INDEX idx_work_items_number         ON work_items(project_id, number);
CREATE INDEX idx_work_items_due_date       ON work_items(due_date) WHERE status != 'done';

CREATE INDEX idx_work_items_fts ON work_items
    USING GIN(to_tsvector('simple', title || ' ' || COALESCE(description, '')));

CREATE INDEX idx_notifications_user_unread ON notifications(user_id, is_read)
    WHERE is_read = false;

CREATE INDEX idx_iterations_project_dates ON iterations(project_id, start_date, end_date);
