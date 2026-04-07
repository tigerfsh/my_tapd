import client from './client'
import type { CreateRequirementRequest, CreateBugRequest, WorkItemFilter } from '../types/api'
import type { WorkItem, Comment, Attachment, AuditLog } from '../types/domain'

export const workItemsApi = {
  list: (projectId: number, filter?: WorkItemFilter) =>
    client.get<{ data: WorkItem[] }>(`/projects/${projectId}/work-items`, { params: filter }),
  get: (projectId: number, id: number) =>
    client.get<{ data: WorkItem }>(`/projects/${projectId}/work-items/${id}`),
  createRequirement: (projectId: number, data: CreateRequirementRequest) =>
    client.post<{ data: WorkItem }>(`/projects/${projectId}/requirements`, data),
  createBug: (projectId: number, data: CreateBugRequest) =>
    client.post<{ data: WorkItem }>(`/projects/${projectId}/bugs`, data),
  update: (projectId: number, id: number, data: Partial<WorkItem>) =>
    client.put<{ data: WorkItem }>(`/projects/${projectId}/work-items/${id}`, data),
  delete: (projectId: number, id: number) =>
    client.delete(`/projects/${projectId}/work-items/${id}`),

  // Comments
  getComments: (projectId: number, id: number) =>
    client.get<{ data: Comment[] }>(`/projects/${projectId}/work-items/${id}/comments`),
  addComment: (projectId: number, id: number, content: string) =>
    client.post<{ data: Comment }>(`/projects/${projectId}/work-items/${id}/comments`, { content }),
  deleteComment: (projectId: number, id: number, commentId: number) =>
    client.delete(`/projects/${projectId}/work-items/${id}/comments/${commentId}`),

  // Attachments
  getAttachments: (projectId: number, id: number) =>
    client.get<{ data: Attachment[] }>(`/projects/${projectId}/work-items/${id}/attachments`),
  deleteAttachment: (projectId: number, id: number, attachmentId: number) =>
    client.delete(`/projects/${projectId}/work-items/${id}/attachments/${attachmentId}`),

  // Audit log
  getAuditLog: (projectId: number, id: number) =>
    client.get<{ data: AuditLog[] }>(`/projects/${projectId}/work-items/${id}/audit-log`),
}
