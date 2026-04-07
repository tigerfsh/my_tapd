import type { WorkItemType, Status, Priority, Role, ProjectType, Severity, IterationStatus } from './enums'

export interface User {
  id: number
  email: string
  nickname: string
  avatarUrl?: string
  phone?: string
  isActive: boolean
  createdAt: string
  updatedAt: string
}

export interface Project {
  id: number
  name: string
  description?: string
  projectType: ProjectType
  isPublic: boolean
  isArchived: boolean
  createdBy: number
  createdAt: string
  updatedAt: string
}

export interface Member {
  projectId: number
  userId: number
  role: Role
  joinedAt: string
}

export interface WorkItem {
  id: number
  projectId: number
  itemType: WorkItemType
  number: string
  title: string
  description?: string
  status: Status
  priority: Priority
  assigneeId?: number
  creatorId: number
  parentId?: number
  iterationId?: number
  dueDate?: string
  storyPoints?: number
  estimatedHours?: number
  actualHours?: number
  severity?: Severity
  completionPct?: number
  createdAt: string
  updatedAt: string
}

export interface Iteration {
  id: number
  projectId: number
  name: string
  goal?: string
  startDate: string
  endDate: string
  status: IterationStatus
  createdBy: number
  createdAt: string
  updatedAt: string
}

export interface Comment {
  id: number
  workItemId: number
  authorId: number
  content: string
  createdAt: string
}

export interface Attachment {
  id: number
  workItemId: number
  uploaderId: number
  filename: string
  fileSize: number
  storageKey: string
  createdAt: string
}

export interface AuditLog {
  id: number
  workItemId: number
  operatorId: number
  fieldName: string
  oldValue?: string
  newValue?: string
  changedAt: string
}

export interface Notification {
  id: number
  userId: number
  eventType: string
  workItemId?: number
  content: string
  isRead: boolean
  retryCount: number
  createdAt: string
}

export interface BurndownSnapshot {
  id: number
  iterationId: number
  snapshotDate: string
  remainingPoints: number
  totalPoints: number
}
