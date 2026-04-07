import client from './client'
import type { Iteration, WorkItem, BurndownSnapshot } from '../types/domain'

export const iterationsApi = {
  list: (projectId: number) =>
    client.get<{ data: Iteration[] }>(`/projects/${projectId}/iterations`),
  get: (projectId: number, id: number) =>
    client.get<{ data: Iteration }>(`/projects/${projectId}/iterations/${id}`),
  create: (projectId: number, data: Partial<Iteration>) =>
    client.post<{ data: Iteration }>(`/projects/${projectId}/iterations`, data),
  update: (projectId: number, id: number, data: Partial<Iteration>) =>
    client.put<{ data: Iteration }>(`/projects/${projectId}/iterations/${id}`, data),
  delete: (projectId: number, id: number) =>
    client.delete(`/projects/${projectId}/iterations/${id}`),

  // Work items in iteration
  getWorkItems: (projectId: number, id: number) =>
    client.get<{ data: WorkItem[] }>(`/projects/${projectId}/iterations/${id}/work-items`),
  addWorkItem: (projectId: number, id: number, workItemId: number) =>
    client.post(`/projects/${projectId}/iterations/${id}/work-items`, { work_item_id: workItemId }),
  removeWorkItem: (projectId: number, id: number, workItemId: number) =>
    client.delete(`/projects/${projectId}/iterations/${id}/work-items/${workItemId}`),

  // Burndown
  getBurndown: (projectId: number, id: number) =>
    client.get<{ data: BurndownSnapshot[] }>(`/projects/${projectId}/iterations/${id}/burndown`),
}
