export enum WorkItemType {
  Requirement = 'requirement',
  Story = 'story',
  Task = 'task',
  Bug = 'bug',
}

export enum Status {
  Pending = 'pending',
  InProgress = 'in_progress',
  Done = 'done',
  Closed = 'closed',
  Rejected = 'rejected',
  PendingVerify = 'pending_verify',
  Fixing = 'fixing',
  Unassigned = 'unassigned',
}

export enum Priority {
  Urgent = 'urgent',
  High = 'high',
  Medium = 'medium',
  Low = 'low',
}

export enum Role {
  Admin = 'admin',
  Developer = 'developer',
  Tester = 'tester',
  Observer = 'observer',
}

export enum ProjectType {
  Agile = 'agile',
  Waterfall = 'waterfall',
}

export enum Severity {
  Fatal = 'fatal',
  Critical = 'critical',
  Normal = 'normal',
  Hint = 'hint',
}

export enum IterationStatus {
  NotStarted = 'not_started',
  InProgress = 'in_progress',
  Completed = 'completed',
}
