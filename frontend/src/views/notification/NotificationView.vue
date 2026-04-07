<template>
  <div class="notification-view">
    <div class="page-header">
      <h2>通知中心</h2>
      <el-button @click="markAll" :disabled="store.unreadCount === 0">全部标为已读</el-button>
    </div>

    <el-tabs v-model="tab" @tab-change="loadNotifications">
      <el-tab-pane label="全部" name="all" />
      <el-tab-pane label="未读" name="unread" />
    </el-tabs>

    <div v-loading="store.loading">
      <div
        v-for="n in store.notifications"
        :key="n.id"
        class="notification-item"
        :class="{ unread: !n.isRead }"
        @click="handleRead(n)"
      >
        <div class="notification-content">{{ n.content }}</div>
        <div class="notification-meta">
          <span>{{ n.eventType }}</span>
          <span>{{ n.createdAt?.slice(0, 16) }}</span>
        </div>
      </div>
      <el-empty v-if="!store.loading && store.notifications.length === 0" description="暂无通知" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { useNotificationStore } from '../../stores/notification'
import type { Notification } from '../../types/domain'

const store = useNotificationStore()
const tab = ref('all')

onMounted(() => loadNotifications('all'))

async function loadNotifications(t: string) {
  await store.fetchNotifications(t === 'unread')
}

async function handleRead(n: Notification) {
  if (!n.isRead) await store.markRead(n.id)
}

async function markAll() {
  await store.markAllRead()
  ElMessage.success('已全部标为已读')
}
</script>

<style scoped>
.notification-view { padding: 24px; max-width: 800px; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
.notification-item {
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
}
.notification-item:hover { background: #f5f7fa; }
.notification-item.unread { background: #ecf5ff; }
.notification-content { font-size: 14px; margin-bottom: 4px; }
.notification-meta { display: flex; gap: 16px; font-size: 12px; color: #909399; }
</style>
