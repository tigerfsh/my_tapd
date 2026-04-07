import client from './client'

export const reportsApi = {
  getProjectStats: (projectId: number) =>
    client.get<{ data: Record<string, unknown> }>(`/projects/${projectId}/reports/stats`),
  getBurndown: (projectId: number, iterationId: number) =>
    client.get<{ data: Record<string, unknown> }>(`/projects/${projectId}/reports/burndown`, {
      params: { iteration_id: iterationId },
    }),
  getMemberWorkload: (projectId: number) =>
    client.get<{ data: Record<string, unknown> }>(`/projects/${projectId}/reports/workload`),
  getBugTrend: (projectId: number, params?: { start_date?: string; end_date?: string }) =>
    client.get<{ data: Record<string, unknown> }>(`/projects/${projectId}/reports/bug-trend`, {
      params,
    }),
}
