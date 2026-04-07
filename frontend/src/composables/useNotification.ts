import { onMounted } from 'vue'
import { useNotificationStore } from '../stores/notification'
import { useAuthStore } from '../stores/auth'
import { useWebSocket } from './useWebSocket'
import type { Notification } from '../types/domain'

export function useNotification() {
  const notificationStore = useNotificationStore()
  const authStore = useAuthStore()
  const { isConnected, connect, disconnect } = useWebSocket()

  function startListening() {
    if (!authStore.token) return
    connect(authStore.token, (data: unknown) => {
      const msg = data as { type: string; payload: Notification }
      if (msg.type === 'notification') {
        notificationStore.addNotification(msg.payload)
      }
    })
  }

  onMounted(async () => {
    await notificationStore.fetchUnreadCount()
    startListening()
  })

  return {
    notifications: notificationStore.notifications,
    unreadCount: notificationStore.unreadCount,
    unreadNotifications: notificationStore.unreadNotifications,
    isConnected,
    fetchNotifications: notificationStore.fetchNotifications,
    markRead: notificationStore.markRead,
    markAllRead: notificationStore.markAllRead,
    disconnect,
  }
}
