import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Notification } from '../types/domain'
import { notificationsApi } from '../api/notifications'

export const useNotificationStore = defineStore('notification', () => {
  const notifications = ref<Notification[]>([])
  const unreadCount = ref(0)
  const loading = ref(false)

  const unreadNotifications = computed(() => notifications.value.filter((n) => !n.isRead))

  async function fetchNotifications(unreadOnly = false) {
    loading.value = true
    try {
      const res = await notificationsApi.list({ unread_only: unreadOnly })
      notifications.value = res.data.data
    } finally {
      loading.value = false
    }
  }

  async function fetchUnreadCount() {
    const res = await notificationsApi.getUnreadCount()
    unreadCount.value = res.data.data.count
  }

  async function markRead(id: number) {
    await notificationsApi.markRead(id)
    const n = notifications.value.find((n) => n.id === id)
    if (n) {
      n.isRead = true
      unreadCount.value = Math.max(0, unreadCount.value - 1)
    }
  }

  async function markAllRead() {
    await notificationsApi.markAllRead()
    notifications.value.forEach((n) => (n.isRead = true))
    unreadCount.value = 0
  }

  function addNotification(notification: Notification) {
    notifications.value.unshift(notification)
    if (!notification.isRead) unreadCount.value++
  }

  return {
    notifications,
    unreadCount,
    loading,
    unreadNotifications,
    fetchNotifications,
    fetchUnreadCount,
    markRead,
    markAllRead,
    addNotification,
  }
})
