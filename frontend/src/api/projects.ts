import client from './client'
import type { CreateProjectRequest } from '../types/api'
import type { Project, Member } from '../types/domain'
import type { Role } from '../types/enums'

export const projectsApi = {
  list: () => client.get<{ data: Project[] }>('/projects'),
  get: (id: number) => client.get<{ data: Project }>(`/projects/${id}`),
  create: (data: CreateProjectRequest) => client.post<{ data: Project }>('/projects', data),
  update: (id: number, data: Partial<CreateProjectRequest>) =>
    client.put<{ data: Project }>(`/projects/${id}`, data),
  delete: (id: number) => client.delete(`/projects/${id}`),
  archive: (id: number) => client.post(`/projects/${id}/archive`),

  // Member management
  getMembers: (id: number) => client.get<{ data: Member[] }>(`/projects/${id}/members`),
  addMember: (id: number, userId: number, role: Role) =>
    client.post<{ data: Member }>(`/projects/${id}/members`, { user_id: userId, role }),
  updateMember: (id: number, userId: number, role: Role) =>
    client.put<{ data: Member }>(`/projects/${id}/members/${userId}`, { role }),
  removeMember: (id: number, userId: number) =>
    client.delete(`/projects/${id}/members/${userId}`),
}
