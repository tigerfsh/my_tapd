<template>
  <div class="kanban-view">
    <div class="page-header">
      <h2>看板</h2>
    </div>
    <div class="kanban-board" v-loading="loading">
      <KanbanColumn
        v-for="col in columns"
        :key="col.status"
        :title="col.title"
        :status="col.status"
        :items="itemsByStatus(col.status)"
        @dragstart="dragging = $event"
        @drop="handleDrop($event)"
        @click="goDetail"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { workItemsApi } from '../../api/workItems'
import type { WorkItem } from '../../types/domain'
import KanbanColumn from '../../components/kanban/KanbanColumn.vue'

const route = useRoute()
const router = useRouter()
const projectId = Number(route.params.id)

const items = ref<WorkItem[]>([])
const loading = ref(false)
const dragging = ref<WorkItem | null>(null)

const columns = [
  { status: 'pending', title: '待处理' },
  { status: 'in_progress', title: '进行中' },
  { status: 'pending_verify', title: '待验证' },
  { status: 'done', title: '已完成' },
  { status: 'closed', title: '已关闭' },
]

onMounted(async () => {
  loading.value = true
  try {
    const res = await workItemsApi.list(projectId)
    items.value = res.data.data
  } finally {
    loading.value = false
  }
})

function itemsByStatus(status: string) {
  return items.value.filter((i) => i.status === status)
}

async function handleDrop(targetStatus: string) {
  if (!dragging.value || dragging.value.status === targetStatus) return
  const item = dragging.value
  try {
    await workItemsApi.update(projectId, item.id, { status: targetStatus as any })
    item.status = targetStatus as any
    ElMessage.success('状态已更新')
  } catch {
    ElMessage.error('更新失败')
  } finally {
    dragging.value = null
  }
}

function goDetail(item: WorkItem) {
  router.push(`/projects/${projectId}/work-items/${item.id}`)
}
</script>

<style scoped>
.kanban-view { padding: 24px; }
.page-header { margin-bottom: 16px; }
.kanban-board {
  display: flex;
  gap: 16px;
  overflow-x: auto;
  padding-bottom: 16px;
}
</style>
