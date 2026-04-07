<template>
  <el-card class="work-item-card" @click="emit('click', item)">
    <div class="card-header">
      <span class="number">{{ item.number }}</span>
      <StatusBadge :status="item.status" />
    </div>
    <div class="title">{{ item.title }}</div>
    <div class="meta">
      <el-tag size="small" :type="priorityType">{{ item.priority }}</el-tag>
      <span v-if="item.assigneeId" class="assignee">
        <el-icon><User /></el-icon> {{ item.assigneeId }}
      </span>
      <span v-if="item.storyPoints" class="points">{{ item.storyPoints }}pt</span>
    </div>
  </el-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { User } from '@element-plus/icons-vue'
import type { WorkItem } from '../../types/domain'
import { Priority } from '../../types/enums'
import StatusBadge from './StatusBadge.vue'

const props = defineProps<{ item: WorkItem }>()
const emit = defineEmits<{ (e: 'click', item: WorkItem): void }>()

const priorityMap: Record<Priority, string> = {
  [Priority.Urgent]: 'danger',
  [Priority.High]: 'warning',
  [Priority.Medium]: '',
  [Priority.Low]: 'info',
}
const priorityType = computed(() => priorityMap[props.item.priority] || '')
</script>

<style scoped>
.work-item-card { cursor: pointer; margin-bottom: 8px; }
.work-item-card:hover { box-shadow: 0 2px 8px rgba(0,0,0,0.1); }
.card-header { display: flex; justify-content: space-between; margin-bottom: 6px; }
.number { font-size: 12px; color: #909399; }
.title { font-size: 14px; margin-bottom: 8px; }
.meta { display: flex; align-items: center; gap: 8px; font-size: 12px; color: #606266; }
</style>
