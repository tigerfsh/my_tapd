import client from './client'
import type { Notification } from '../types/domain'

export const notificationsApi = {
  list: (params?: { unread_only?: boolean; page?: number; page_size?: number }) =>
    client.get<{ data: Notification[]; total: number }>('/notifications', { params }),
  markRead: (id: number) => client.post(`/notifications/${id}/read`),
  markAllRead: () => client.post('/notifications/read-all'),
  delete: (id: number) => client.delete(`/notifications/${id}`),
  getUnreadCount: () => client.get<{ data: { count: number } }>('/notifications/unread-count'),
}
