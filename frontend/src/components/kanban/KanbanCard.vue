<template>
  <div class="kanban-card" draggable="true" @dragstart="onDragStart">
    <div class="card-number">{{ item.number }}</div>
    <div class="card-title">{{ item.title }}</div>
    <div class="card-meta">
      <el-tag size="small" :type="priorityType">{{ item.priority }}</el-tag>
      <span v-if="item.assigneeId" class="assignee">{{ item.assigneeId }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { WorkItem } from '../../types/domain'
import { Priority } from '../../types/enums'

const props = defineProps<{ item: WorkItem }>()
const emit = defineEmits<{ (e: 'dragstart', item: WorkItem): void }>()

const priorityMap: Record<Priority, string> = {
  [Priority.Urgent]: 'danger',
  [Priority.High]: 'warning',
  [Priority.Medium]: '',
  [Priority.Low]: 'info',
}
const priorityType = computed(() => priorityMap[props.item.priority] || '')

function onDragStart() {
  emit('dragstart', props.item)
}
</script>

<style scoped>
.kanban-card {
  background: #fff;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  padding: 10px;
  margin-bottom: 8px;
  cursor: grab;
}
.kanban-card:hover { box-shadow: 0 2px 6px rgba(0,0,0,0.1); }
.card-number { font-size: 11px; color: #909399; margin-bottom: 4px; }
.card-title { font-size: 13px; margin-bottom: 8px; }
.card-meta { display: flex; align-items: center; justify-content: space-between; }
.assignee { font-size: 12px; color: #606266; }
</style>
