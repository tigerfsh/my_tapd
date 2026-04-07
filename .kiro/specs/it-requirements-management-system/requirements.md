# 需求文档

## 简介

IT需求管理系统是一个面向研发团队的敏捷项目管理平台，类似腾讯TAPD。系统支持需求全生命周期管理，涵盖需求收集、分析、拆解、迭代规划、开发跟踪、测试验收及发布上线等环节。系统面向产品经理、开发工程师、测试工程师及项目管理者，提供多角色协作能力，帮助团队高效交付软件产品。

---

## 术语表

- **System**：IT需求管理系统整体
- **User**：已登录系统的用户（产品经理、开发、测试、项目经理等）
- **Project**：项目，团队协作的基本单元，包含需求、迭代、缺陷等资源
- **Requirement**：需求条目，描述产品功能或业务诉求的工作项
- **Story**：用户故事，需求的细化拆解单元
- **Task**：任务，可分配给个人的最小执行单元
- **Bug**：缺陷，测试或用户反馈的问题记录
- **Iteration**：迭代（Sprint），固定时间周期的开发计划单元
- **Backlog**：待办列表，未纳入迭代的需求池
- **WorkItem**：工作项，需求/故事/任务/缺陷的统称
- **Status**：工作项状态，如"待处理"、"进行中"、"已完成"、"已关闭"
- **Priority**：优先级，如"紧急"、"高"、"中"、"低"
- **Member**：项目成员，被加入项目的用户
- **Role**：角色，成员在项目中的权限身份（管理员、开发、测试、观察者）
- **Comment**：评论，工作项下的讨论记录
- **Attachment**：附件，工作项关联的文件资源
- **Label**：标签，用于分类和筛选工作项的自定义标记
- **Dashboard**：仪表盘，展示项目统计数据和进度的可视化页面
- **Notification**：通知，系统向用户推送的变更提醒

---

## 需求列表

### 需求 1：用户账户管理

**用户故事：** 作为一名用户，我希望能够注册、登录和管理个人账户，以便安全地访问系统并维护个人信息。

#### 验收标准

1. THE System SHALL 支持用户通过邮箱和密码完成注册，注册时邮箱须唯一。
2. WHEN 用户提交注册表单时，THE System SHALL 向注册邮箱发送验证邮件，并在用户点击验证链接后激活账户。
3. WHEN 用户提供正确的邮箱和密码时，THE System SHALL 在 2 秒内完成身份验证并返回访问令牌。
4. IF 用户连续 5 次登录失败，THEN THE System SHALL 锁定该账户 15 分钟并通知用户。
5. WHEN 用户请求重置密码时，THE System SHALL 向注册邮箱发送有效期为 30 分钟的重置链接。
6. THE System SHALL 支持用户修改昵称、头像和联系方式等个人信息。
7. THE System SHALL 对所有用户密码使用不可逆加密算法（bcrypt 或同等强度）进行存储。

---

### 需求 2：项目管理

**用户故事：** 作为一名项目管理者，我希望能够创建和管理项目，以便组织团队协作和资源分配。

#### 验收标准

1. THE System SHALL 支持用户创建项目，项目包含名称、描述、类型（敏捷/瀑布）和可见性（公开/私有）属性。
2. WHEN 用户创建项目时，THE System SHALL 将创建者自动设置为项目管理员。
3. THE System SHALL 支持项目管理员邀请成员加入项目，并为成员分配角色（管理员/开发/测试/观察者）。
4. WHEN 项目管理员移除成员时，THE System SHALL 将该成员名下未完成的 WorkItem 状态更新为"待分配"。
5. THE System SHALL 支持项目管理员修改项目名称、描述和成员角色。
6. WHEN 项目被归档时，THE System SHALL 将项目设为只读状态，禁止新增或修改 WorkItem。
7. IF 用户尝试访问无权限的私有项目，THEN THE System SHALL 返回权限不足提示，不暴露项目内容。

---

### 需求 3：需求（Requirement）管理

**用户故事：** 作为一名产品经理，我希望能够创建和管理需求条目，以便清晰记录产品功能诉求并跟踪实现进度。

#### 验收标准

1. THE System SHALL 支持 Member 在项目内创建 Requirement，Requirement 包含标题、描述、Priority、负责人、截止日期和 Label 属性。
2. WHEN Requirement 被创建时，THE System SHALL 将其初始 Status 设置为"待处理"，并记录创建时间和创建人。
3. THE System SHALL 支持将 Requirement 拆解为多个 Story，Story 与父 Requirement 保持关联关系。
4. WHEN 所有关联 Story 的 Status 均变更为"已完成"时，THE System SHALL 自动将父 Requirement 的 Status 更新为"已完成"。
5. THE System SHALL 支持 Member 对 Requirement 进行评论、上传 Attachment 和修改 Label。
6. THE System SHALL 支持按 Status、Priority、负责人、Label 和迭代对 Requirement 进行筛选和排序。
7. IF Requirement 的截止日期已过且 Status 不为"已完成"，THEN THE System SHALL 在 Dashboard 中以高亮方式标记该 Requirement。
8. THE System SHALL 记录 Requirement 的全部字段变更历史，包含变更时间、变更人和变更前后的值。

---

### 需求 4：用户故事（Story）与任务（Task）管理

**用户故事：** 作为一名开发工程师，我希望能够管理用户故事和任务，以便明确工作范围并跟踪个人工作进度。

#### 验收标准

1. THE System SHALL 支持 Member 在 Requirement 下创建 Story，Story 包含标题、描述、故事点数、负责人和 Status 属性。
2. THE System SHALL 支持 Member 在 Story 下创建 Task，Task 包含标题、负责人、预估工时和 Status 属性。
3. WHEN Task 的 Status 变更为"已完成"时，THE System SHALL 重新计算父 Story 的完成百分比（已完成 Task 数 / 总 Task 数 × 100%）。
4. WHEN Story 下所有 Task 的 Status 均为"已完成"时，THE System SHALL 将该 Story 的 Status 自动更新为"已完成"。
5. THE System SHALL 支持 Member 记录 Task 的实际工时，实际工时精度为 0.5 小时。
6. THE System SHALL 支持将 Story 或 Task 分配给项目内的任意 Member。
7. IF Task 的负责人不是当前 Member 且当前 Member 的 Role 不是管理员，THEN THE System SHALL 禁止该 Member 修改 Task 的 Status。

---

### 需求 5：迭代（Iteration）管理

**用户故事：** 作为一名项目管理者，我希望能够规划和管理迭代，以便按周期组织团队交付工作。

#### 验收标准

1. THE System SHALL 支持项目管理员创建 Iteration，Iteration 包含名称、开始日期、结束日期和目标描述属性。
2. IF 新建 Iteration 的时间范围与同项目已有 Iteration 的时间范围重叠，THEN THE System SHALL 拒绝创建并提示冲突的 Iteration 名称。
3. THE System SHALL 支持将 Backlog 中的 Story 拖拽或批量分配到指定 Iteration。
4. WHILE Iteration 处于"进行中"状态时，THE System SHALL 在 Dashboard 展示燃尽图，燃尽图数据每日更新一次。
5. WHEN Iteration 结束日期到达时，THE System SHALL 将未完成的 Story 自动移回 Backlog，并生成迭代总结报告。
6. THE System SHALL 支持查看 Iteration 内所有 WorkItem 的 Status 分布统计。
7. THE System SHALL 支持项目管理员修改未开始 Iteration 的名称、时间范围和目标描述。

---

### 需求 6：缺陷（Bug）管理

**用户故事：** 作为一名测试工程师，我希望能够记录和跟踪缺陷，以便确保产品质量并推动问题修复。

#### 验收标准

1. THE System SHALL 支持 Member 创建 Bug，Bug 包含标题、描述、复现步骤、严重程度（致命/严重/一般/提示）、Priority、负责人和关联 Requirement 属性。
2. WHEN Bug 被创建时，THE System SHALL 将其初始 Status 设置为"待处理"，并通知 Bug 负责人。
3. THE System SHALL 支持 Bug 的 Status 按以下流转路径变更：待处理 → 修复中 → 待验证 → 已关闭 / 已拒绝。
4. WHEN Bug 的 Status 变更为"待验证"时，THE System SHALL 通知 Bug 的创建人进行验证。
5. IF Bug 的 Status 为"已关闭"后被重新打开，THEN THE System SHALL 将 Status 重置为"待处理"并记录重开原因。
6. THE System SHALL 支持按严重程度、Priority、Status、负责人和关联 Requirement 对 Bug 进行筛选。
7. THE System SHALL 支持 Member 对 Bug 上传截图等 Attachment，单个 Attachment 大小不超过 20MB。

---

### 需求 7：看板（Kanban）视图

**用户故事：** 作为一名开发工程师，我希望通过看板视图直观地查看工作项状态，以便快速了解团队进度并更新工作状态。

#### 验收标准

1. THE System SHALL 提供看板视图，以列的形式展示 WorkItem 的各个 Status（待处理、进行中、已完成、已关闭）。
2. THE System SHALL 支持 Member 通过拖拽 WorkItem 卡片在看板列之间移动，移动后 WorkItem 的 Status 同步更新。
3. WHEN WorkItem 的 Status 通过其他方式（如详情页）更新时，THE System SHALL 实时刷新看板视图中对应卡片的位置。
4. THE System SHALL 支持在看板视图中按负责人、Label 和 Priority 筛选显示的 WorkItem。
5. THE System SHALL 支持在看板视图中直接创建 WorkItem，新建的 WorkItem 出现在对应 Status 列中。
6. WHERE 项目启用了泳道配置，THE System SHALL 支持按负责人或 Label 对看板进行泳道分组展示。

---

### 需求 8：通知与提醒

**用户故事：** 作为一名用户，我希望及时收到与我相关的工作项变更通知，以便快速响应并保持工作同步。

#### 验收标准

1. WHEN WorkItem 被分配给 Member 时，THE System SHALL 向该 Member 发送站内 Notification。
2. WHEN WorkItem 的 Status、Priority 或负责人发生变更时，THE System SHALL 向 WorkItem 的创建人和当前负责人发送 Notification。
3. WHEN Member 在 WorkItem 下发布 Comment 时，THE System SHALL 向 WorkItem 的创建人、负责人及其他评论者发送 Notification。
4. THE System SHALL 支持 Member 在个人设置中配置各类事件的 Notification 开关。
5. WHILE Member 在线时，THE System SHALL 在 5 秒内将 Notification 推送至 Member 的浏览器。
6. THE System SHALL 支持 Member 查看全部历史 Notification，并支持标记为已读或全部已读操作。
7. IF Notification 推送失败，THEN THE System SHALL 在 1 分钟内重试推送，最多重试 3 次。

---

### 需求 9：统计报表与仪表盘

**用户故事：** 作为一名项目管理者，我希望通过仪表盘和报表了解项目整体进度，以便做出数据驱动的决策。

#### 验收标准

1. THE System SHALL 为每个项目提供 Dashboard，展示当前 Iteration 进度、WorkItem 状态分布、Bug 趋势和成员工作量统计。
2. THE System SHALL 支持生成需求完成率报表，报表数据可按迭代或自定义时间范围筛选。
3. THE System SHALL 支持生成 Bug 统计报表，包含新增数、修复数、遗留数及按严重程度的分布。
4. WHEN 用户导出报表时，THE System SHALL 在 10 秒内生成 Excel 或 CSV 格式的文件供下载。
5. THE System SHALL 支持查看每位 Member 在指定时间范围内的工作项完成数量和实际工时汇总。
6. WHILE Dashboard 页面处于打开状态时，THE System SHALL 每 5 分钟自动刷新统计数据。

---

### 需求 10：搜索与全局导航

**用户故事：** 作为一名用户，我希望能够快速搜索和定位工作项，以便在大量数据中高效找到所需内容。

#### 验收标准

1. THE System SHALL 提供全局搜索功能，支持按标题关键词搜索项目内的 Requirement、Story、Task 和 Bug。
2. WHEN 用户输入搜索关键词时，THE System SHALL 在 1 秒内返回匹配结果，结果按相关度排序。
3. THE System SHALL 支持在搜索结果中按 WorkItem 类型、Status 和 Priority 进行二次筛选。
4. THE System SHALL 支持通过唯一编号（如 REQ-001、BUG-042）直接定位到对应 WorkItem。
5. IF 搜索关键词长度超过 200 个字符，THEN THE System SHALL 截断至 200 个字符后执行搜索，并提示用户已截断。
