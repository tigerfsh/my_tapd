# 实现计划：IT需求管理系统

## 概述

基于需求文档和技术设计文档，将系统实现拆解为以下增量式编码任务。每个任务在前一任务基础上构建，最终完成完整的前后端系统。技术栈：Rust + Axum + PostgreSQL + Redis（后端），Vue 3 + Vite + Pinia + Element Plus（前端），Docker Compose（部署）。

## 任务列表

- [x] 1. 项目初始化与基础设施搭建
  - 完善 `Cargo.toml`，添加 axum、sqlx、redis、tokio、serde、jwt、bcrypt、thiserror、anyhow、proptest 等依赖
  - 创建 `src/config.rs`，实现从环境变量加载配置（DATABASE_URL、REDIS_URL、JWT_SECRET 等）
  - 创建 `src/error.rs`，定义 `AppError` 枚举及 `status_code()` 方法，实现 Axum `IntoResponse`
  - 创建 `src/db.rs`，初始化 SQLx PostgreSQL 连接池和 Redis 连接池
  - 更新 `src/main.rs`，完成配置加载、连接池初始化、路由挂载和服务启动
  - _需求：全局基础设施_

- [x] 2. 数据库迁移：创建所有表结构
  - 创建 `migrations/` 目录，编写初始迁移文件
  - 创建 `users`、`projects`、`project_members` 表
  - 创建 `work_items`、`work_item_labels` 表（含所有枚举类型）
  - 创建 `iterations`、`comments`、`attachments`、`audit_logs` 表
  - 创建 `notifications`、`notification_preferences`、`burndown_snapshots` 表
  - 创建设计文档中定义的所有索引（含全文搜索 GIN 索引）
  - _需求：1.1、2.1、3.1、4.1、5.1、6.1、8.1、9.4_

- [x] 3. 领域模型与核心算法实现
  - [x] 3.1 创建 `src/domain/` 模块，定义所有枚举类型（`WorkItemType`、`Status`、`Priority`、`Role`、`Severity`、`IterationStatus` 等）和结构体（`User`、`Project`、`WorkItem`、`Iteration`、`Notification` 等）
    - _需求：1.1、2.1、3.1、4.1、5.1、6.1_

  - [x] 3.2 在 `src/domain/` 中实现核心纯函数：`check_account_lock`、`should_lock_account`、`is_token_valid`、`iterations_overlap`、`find_conflicting_iteration`、`calc_completion_pct`、`is_valid_hours`、`is_valid_bug_transition`、`sanitize_query`、`validate_attachment_size`、`should_retry_notification`、`is_overdue`、`group_by_swimlane`
    - _需求：1.4、1.5、3.7、4.3、4.5、5.2、6.3、6.7、8.7、10.5_

  - [x] 3.3 为核心纯函数编写属性测试（proptest）
    - **属性 17：工时精度约束** — 验证 `is_valid_hours` 对 0.5 整数倍返回 true，其他返回 false
    - **属性 38：搜索关键词截断** — 验证 `sanitize_query` 对超过 200 字符的输入截断至恰好 200 字符
    - **属性 15：Story 完成百分比计算正确性** — 验证 `calc_completion_pct` 等于 round(K/N*100)
    - **属性 19：迭代时间冲突检测** — 验证 `iterations_overlap` 与手动区间重叠计算一致
    - **属性 13：逾期需求标记** — 验证 `is_overdue` 对截止日期已过且未完成的需求返回 true
    - **属性 22：Bug 状态机合法性** — 验证合法转换被允许，非法转换被拒绝
    - **属性 25：附件大小限制** — 验证超过 20MB 的请求被拒绝
    - **属性 27：泳道分组正确性** — 验证 `group_by_swimlane` 分组结果完整且正确
    - **验证需求：1.4、1.5、3.7、4.3、4.5、5.2、6.3、6.7、10.5**

  - [x] 3.4 为核心纯函数编写单元测试
    - 测试 Bug 状态机合法与非法转换（需求 6.3）
    - 测试逾期检测边界条件（需求 3.7）
    - 测试附件大小边界值（需求 6.7）
    - _需求：3.7、6.3、6.7_

- [x] 4. 数据访问层（Repository）实现
  - [x] 4.1 创建 `src/repository/user_repo.rs`，实现用户的增删改查、登录失败计数更新、账户锁定查询
    - _需求：1.1、1.4、1.6_

  - [x] 4.2 创建 `src/repository/project_repo.rs`，实现项目 CRUD、成员管理（邀请/移除/角色更新）、归档操作
    - _需求：2.1、2.3、2.4、2.5、2.6_

  - [x] 4.3 创建 `src/repository/work_item_repo.rs`，实现工作项 CRUD、状态更新、标签管理、多条件筛选查询、按编号查找
    - _需求：3.1、3.6、4.1、4.2、6.1、6.6、10.4_

  - [x] 4.4 创建 `src/repository/iteration_repo.rs`，实现迭代 CRUD、故事分配、时间范围冲突查询、燃尽图快照写入
    - _需求：5.1、5.3、5.4、5.6_

  - [x] 4.5 创建 `src/repository/notification_repo.rs`，实现通知的创建、列表查询、标记已读、重试计数更新
    - _需求：8.1、8.6、8.7_

  - [x] 4.6 创建 `src/repository/audit_repo.rs`，实现变更历史记录写入和按工作项查询
    - _需求：3.8_

- [x] 5. 检查点 — 确保数据库迁移和 Repository 层编译通过
  - 确保所有测试通过，如有问题请向用户反馈。

- [x] 6. 认证服务（AuthService）实现
  - [x] 6.1 创建 `src/service/auth_service.rs`，实现 `register`（邮箱唯一性校验、bcrypt 哈希、发送验证邮件）
    - _需求：1.1、1.2、1.7_

  - [x] 6.2 实现 `verify_email`（验证令牌有效性、激活账户）和 `login`（凭证校验、锁定检查、JWT 生成、失败计数）
    - _需求：1.3、1.4_

  - [x] 6.3 实现 `request_password_reset`（生成 30 分钟有效期令牌、发送邮件）和 `reset_password`（令牌验证、密码更新）
    - _需求：1.5_

  - [x] 6.4 实现 `logout`（Redis 令牌黑名单）和 `update_profile`（昵称/头像/联系方式更新）
    - _需求：1.6_

  - [x] 6.5 为认证服务编写属性测试
    - **属性 1：邮箱唯一性约束** — 相同邮箱第二次注册必须被拒绝
    - **属性 2：密码哈希不可逆性** — 存储哈希不等于原始密码，verify 函数行为正确
    - **属性 3：登录令牌有效性** — 正确凭证必须返回非空有效令牌
    - **属性 4：账户锁定机制** — 连续 5 次失败后第 6 次必须被拒绝
    - **属性 5：重置令牌有效期** — 30 分钟内有效，之后失效
    - **验证需求：1.1、1.3、1.4、1.5、1.7**

- [x] 7. 项目服务（ProjectService）实现
  - [x] 7.1 创建 `src/service/project_service.rs`，实现 `create_project`（自动将创建者设为管理员）
    - _需求：2.1、2.2_

  - [x] 7.2 实现 `invite_member`、`remove_member`（移除后将未完成工作项状态改为 Unassigned）、`update_project`
    - _需求：2.3、2.4、2.5_

  - [x] 7.3 实现 `archive_project`（设为只读）和 `get_project`（私有项目权限检查）
    - _需求：2.6、2.7_

  - [x] 7.4 为项目服务编写属性测试
    - **属性 6：项目创建者自动成为管理员** — 创建者角色必须为 Admin
    - **属性 7：移除成员后工作项状态重置** — 所有未完成工作项变为 Unassigned
    - **属性 8：归档项目只读约束** — 归档后创建/修改操作必须被拒绝
    - **属性 9：私有项目访问控制** — 非成员访问必须被拒绝且不暴露内容
    - **验证需求：2.2、2.4、2.6、2.7**

- [x] 8. 工作项服务（WorkItemService）实现
  - [x] 8.1 创建 `src/service/work_item_service.rs`，实现 `create_requirement`（初始状态 Pending、记录创建时间和创建人）
    - _需求：3.1、3.2_

  - [x] 8.2 实现 `create_story`（关联父需求）和 `create_task`（关联父故事）
    - _需求：3.3、4.1、4.2_

  - [x] 8.3 实现 `update_status`，包含：Bug 状态机校验、Task 完成触发 Story 完成百分比重算、Story 全部完成触发父需求完成、Task 状态修改权限校验
    - _需求：3.4、4.3、4.4、4.7、6.3_

  - [x] 8.4 实现 `create_bug`（初始状态 Pending）、`assign`（触发通知）、`log_actual_hours`（0.5h 精度校验）
    - _需求：4.5、6.1、6.2_

  - [x] 8.5 实现 `add_comment`（触发通知）、`upload_attachment`（20MB 大小校验）、`list_work_items`（多条件筛选）、`get_change_history`
    - _需求：3.5、3.6、3.8、6.7_

  - [x] 8.6 为工作项服务编写属性测试
    - **属性 10：需求创建初始状态** — 新需求初始状态必须为 Pending
    - **属性 11：子故事全部完成触发父需求完成** — 所有故事完成后父需求自动完成
    - **属性 12：工作项筛选结果一致性** — 筛选结果每项满足所有条件且无遗漏
    - **属性 14：变更历史完整性** — N 次变更后历史记录数量等于 N
    - **属性 16：子任务全部完成触发父故事完成** — 所有任务完成后父故事自动完成
    - **属性 18：任务状态修改权限控制** — 非负责人非管理员修改必须被拒绝
    - **属性 21：Bug 创建初始状态与通知** — 初始状态为 Pending 且通知负责人
    - **属性 24：Bug 重开状态重置** — 重开后状态为 Pending 且记录重开原因
    - **验证需求：3.2、3.4、3.6、3.8、4.4、4.7、6.2、6.5**

- [x] 9. 迭代服务（IterationService）实现
  - [x] 9.1 创建 `src/service/iteration_service.rs`，实现 `create_iteration`（时间冲突检测）
    - _需求：5.1、5.2_

  - [x] 9.2 实现 `assign_stories`（批量分配故事到迭代）和 `update_iteration`（仅限未开始迭代）
    - _需求：5.3、5.7_

  - [x] 9.3 实现 `close_iteration`（未完成故事移回 Backlog、生成迭代总结）和 `get_burndown_data`（读取每日快照）
    - _需求：5.4、5.5、5.6_

  - [x] 9.4 为迭代服务编写属性测试
    - **属性 19：迭代时间冲突检测** — 时间重叠时创建必须被拒绝并返回冲突名称
    - **属性 20：迭代结束未完成故事回归 Backlog** — 所有未完成故事 iteration_id 置为 None
    - **验证需求：5.2、5.5**

- [x] 10. 通知服务（NotificationService）与 WebSocket 实现
  - [x] 10.1 创建 `src/service/notification_service.rs`，实现 `send`（写入数据库、推送 Redis 队列、重试逻辑）、`list_notifications`、`mark_read`、`mark_all_read`、`update_preferences`
    - _需求：8.1、8.2、8.3、8.4、8.6、8.7_

  - [x] 10.2 创建 `src/ws/handler.rs`，实现 WebSocket 连接管理（JWT 验证、订阅 Redis 通知队列、5 秒内推送）
    - _需求：8.5_

  - [x] 10.3 为通知服务编写属性测试
    - **属性 28：工作项分配通知** — 分配操作必须向被分配成员发送通知
    - **属性 29：工作项变更通知覆盖** — 状态/优先级/负责人变更必须通知创建人和当前负责人
    - **属性 30：评论通知覆盖** — 评论发布必须通知创建人、负责人及其他评论者（排除发布者）
    - **属性 31：通知已读状态管理** — 标记已读后 is_read 为 true，未标记的不变
    - **属性 32：通知重试次数上限** — 重试次数不超过 3 次
    - **验证需求：8.1、8.2、8.3、8.6、8.7**

- [x] 11. 搜索服务与报表服务实现
  - [x] 11.1 创建 `src/service/search_service.rs`，实现 `search`（PostgreSQL 全文搜索、关键词截断、结果按相关度排序）和 `find_by_number`（按唯一编号精确查找）
    - _需求：10.1、10.2、10.4、10.5_

  - [x] 11.2 创建 `src/service/report_service.rs`，实现需求完成率报表、Bug 统计报表、成员工作量报表、仪表盘数据聚合、Excel/CSV 导出
    - _需求：9.1、9.2、9.3、9.4、9.5_

  - [x] 11.3 为搜索和报表服务编写属性测试
    - **属性 33：需求完成率计算正确性** — 完成率等于已完成数/总数×100%，各状态之和等于总数
    - **属性 34：Bug 统计数据一致性** — 新增+修复+遗留等于总数，各严重程度之和等于总数
    - **属性 35：成员工时汇总正确性** — 汇总值等于各任务实际工时之和
    - **属性 36：搜索结果关键词匹配** — 结果中每项标题必须包含关键词
    - **属性 37：按编号精确查找** — 返回工作项的 number 字段与查询编号完全一致
    - **验证需求：9.2、9.3、9.5、10.1、10.4**

- [x] 12. HTTP API 路由与中间件实现
  - [x] 12.1 创建 `src/api/middleware.rs`，实现 JWT 认证中间件（从 Authorization 头提取并验证令牌）和请求日志中间件
    - _需求：1.3_

  - [x] 12.2 创建 `src/api/auth.rs`，实现认证相关 Handler（注册、邮箱验证、登录、登出、密码重置、个人信息）
    - _需求：1.1、1.2、1.3、1.5、1.6_

  - [x] 12.3 创建 `src/api/projects.rs`，实现项目相关 Handler（创建、详情、更新、归档、成员管理）
    - _需求：2.1、2.3、2.5、2.6_

  - [x] 12.4 创建 `src/api/work_items.rs`，实现工作项相关 Handler（需求/故事/任务/Bug 的 CRUD、状态更新、分配、评论、附件、工时记录、变更历史）
    - _需求：3.1、3.5、3.6、3.8、4.1、4.2、4.5、6.1、6.6_

  - [x] 12.5 创建 `src/api/iterations.rs`，实现迭代相关 Handler（创建、更新、故事分配、关闭、燃尽图、统计）
    - _需求：5.1、5.3、5.4、5.6_

  - [x] 12.6 创建 `src/api/notifications.rs`、`src/api/reports.rs`、`src/api/search.rs`，实现通知、报表、搜索 Handler
    - _需求：8.4、8.6、9.1、9.4、10.1、10.4_

  - [x] 12.7 创建 `src/api/router.rs`，将所有 Handler 挂载到对应路由，配置 CORS 和全局错误处理
    - _需求：全局 API 路由_

- [x] 13. 检查点 — 确保后端所有测试通过
  - 运行 `cargo test` 确保所有单元测试和属性测试通过，如有问题请向用户反馈。

- [x] 14. 前端项目初始化
  - 在 `frontend/` 目录下初始化 Vue 3 + Vite + TypeScript 项目
  - 创建 `deno.json`（配置 tasks: dev/build/preview/lint/fmt，以及 npm 包 imports 映射）
  - 创建 `vite.config.ts`（使用 `npm:vite@^8` 导入，配置路径别名 `@`、开发代理 `/api` 和 `/ws`）
  - 创建 `src/types/domain.ts`、`src/types/api.ts`、`src/types/enums.ts`，定义所有 TypeScript 类型
  - 验证 `deno task dev` 可正常启动开发服务器
  - _需求：前端基础设施_

- [x] 15. 前端 API 层与状态管理
  - [x] 15.1 创建 `src/api/client.ts`（Axios 实例、JWT 请求拦截器、401/403 响应拦截器）
    - _需求：1.3_

  - [x] 15.2 创建 `src/api/auth.ts`、`src/api/projects.ts`、`src/api/workItems.ts`、`src/api/iterations.ts`、`src/api/notifications.ts`、`src/api/reports.ts`，封装所有 API 调用
    - _需求：全部 API 接口_

  - [x] 15.3 创建 Pinia Store：`src/stores/auth.ts`（用户认证状态）、`src/stores/project.ts`（当前项目）、`src/stores/notification.ts`（通知列表和未读数）、`src/stores/workitem.ts`（工作项缓存）
    - _需求：1.3、8.5、8.6_

  - [x] 15.4 创建 `src/composables/useWebSocket.ts`（WebSocket 连接管理、断线重连）、`src/composables/useNotification.ts`、`src/composables/usePagination.ts`
    - _需求：8.5_

- [x] 16. 前端路由与布局组件
  - 创建 `src/router/index.ts`，配置所有路由（含权限守卫，未登录跳转 `/login`）
  - 创建 `src/components/layout/AppLayout.vue`（主布局：侧边栏 + 顶栏）、`AppSidebar.vue`、`AppHeader.vue`
  - 创建 `src/components/common/NotificationBell.vue`（通知铃铛，显示未读数）
  - _需求：1.3、8.6_

- [x] 17. 认证页面实现
  - 创建 `src/views/auth/LoginView.vue`（邮箱密码登录表单，JWT 存储至 localStorage）
  - 创建 `src/views/auth/RegisterView.vue`（注册表单，含邮箱唯一性提示）
  - _需求：1.1、1.2、1.3_

- [x] 18. 项目管理页面实现
  - 创建 `src/views/project/ProjectListView.vue`（项目列表，支持创建项目）
  - 创建 `src/views/project/ProjectDetailView.vue`（项目详情，含成员列表）
  - 创建 `src/views/project/ProjectSettingsView.vue`（项目设置：名称/描述/成员角色/归档）
  - _需求：2.1、2.3、2.5、2.6_

- [x] 19. 工作项页面实现
  - [x] 19.1 创建 `src/components/workitem/WorkItemCard.vue`、`WorkItemForm.vue`、`StatusBadge.vue`、`CommentList.vue` 通用组件
    - _需求：3.1、3.5_

  - [x] 19.2 创建 `src/views/workitem/RequirementListView.vue`（需求列表，支持按状态/优先级/负责人/标签/迭代筛选和排序，逾期需求高亮）
    - _需求：3.6、3.7_

  - [x] 19.3 创建 `src/views/workitem/WorkItemDetailView.vue`（工作项详情：含故事/任务子列表、评论、附件上传、变更历史）
    - _需求：3.3、3.5、3.8、4.1、4.2_

  - [x] 19.4 创建 `src/views/workitem/BugListView.vue`（Bug 列表，支持按严重程度/优先级/状态/负责人筛选）
    - _需求：6.6_

- [x] 20. 看板视图实现
  - 创建 `src/components/kanban/KanbanColumn.vue` 和 `KanbanCard.vue`
  - 创建 `src/views/kanban/KanbanView.vue`，实现拖拽更新状态（调用 `PUT /work-items/:id/status`）、按负责人/标签/优先级筛选、泳道分组展示
  - _需求：7.1、7.2、7.3、7.4、7.5、7.6_

- [x] 21. 迭代管理页面实现
  - 创建 `src/components/chart/BurndownChart.vue`（ECharts 燃尽图）和 `StatusPieChart.vue`（状态分布饼图）
  - 创建 `src/views/iteration/IterationListView.vue`（迭代列表，支持创建迭代）
  - 创建 `src/views/iteration/IterationDetailView.vue`（迭代详情：燃尽图、故事列表、状态分布统计）
  - _需求：5.1、5.3、5.4、5.6_

- [x] 22. 报表与通知页面实现
  - 创建 `src/views/report/ReportView.vue`（报表中心：需求完成率、Bug 统计、成员工作量，支持导出 Excel/CSV，Dashboard 每 5 分钟自动刷新）
  - 创建 `src/views/notification/NotificationView.vue`（通知列表，支持标记已读/全部已读）
  - _需求：8.6、9.1、9.2、9.3、9.4、9.5、9.6_

- [x] 23. 全局搜索组件实现
  - 创建 `src/components/common/SearchBar.vue`（全局搜索框，支持按类型/状态/优先级二次筛选，1 秒内返回结果）
  - 支持通过唯一编号（REQ-001、BUG-042）直接跳转到对应工作项
  - _需求：10.1、10.2、10.3、10.4_

- [x] 24. Docker 部署配置
  - 创建 `Dockerfile.backend`（多阶段构建：rust:1.82-slim 编译，debian:bookworm-slim 运行）
  - 创建 `frontend/Dockerfile.frontend`（denoland/deno:2-alpine 构建，nginx:1.27-alpine 服务）
  - 创建 `frontend/nginx.conf`（Vue Router history 模式、/api/ 和 /ws 反向代理、静态资源缓存）
  - 创建 `docker-compose.yaml`（postgres、redis、backend、frontend 四个服务，含健康检查和依赖关系）
  - 创建 `.env.example`（所有环境变量说明）
  - _需求：部署_

- [x] 25. 最终检查点 — 确保所有测试通过
  - 确保所有测试通过，如有问题请向用户反馈。

## 备注

- 标有 `*` 的子任务为可选项，可跳过以加快 MVP 交付
- 每个任务均引用了具体需求条款，确保可追溯性
- 属性测试使用 proptest 库，每个属性最少 100 次迭代
- 每个属性测试文件顶部须包含注释：`// Feature: it-requirements-management-system, Property N: <属性描述>`
- 检查点任务确保增量验证，避免积累错误
