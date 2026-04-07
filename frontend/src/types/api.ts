import type { WorkItemType, Status, Priority, Severity, ProjectType, Role } from './enums'

export interface RegisterRequest {
  email: string
  password: string
  nickname: string
}

export interface LoginRequest {
  email: string
  password: string
}

export interface AuthToken {
  access_token: string
  token_type: string
  expires_in: number
}

export interface ApiResponse<T> {
  data: T
}

export interface CreateProjectRequest {
  name: string
  description?: string
  project_type: ProjectType
  is_public: boolean
}

export interface CreateRequirementRequest {
  title: string
  description?: string
  priority: Priority
  assignee_id?: number
  due_date?: string
  labels: string[]
}

export interface CreateBugRequest {
  title: string
  description?: string
  repro_steps?: string
  severity: Severity
  priority: Priority
  assignee_id?: number
}

export interface WorkItemFilter {
  status?: Status
  priority?: Priority
  assignee_id?: number
  label?: string
  iteration_id?: number
  item_type?: WorkItemType
}

export interface SearchQuery {
  keyword: string
  item_type?: WorkItemType
  status?: Status
  priority?: Priority
}
