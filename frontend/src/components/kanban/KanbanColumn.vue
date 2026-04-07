<template>
  <div
    class="kanban-column"
    @dragover.prevent
    @drop="onDrop"
  >
    <div class="column-header">
      <span class="column-title">{{ title }}</span>
      <el-badge :value="items.length" type="info" />
    </div>
    <div class="column-body">
      <KanbanCard
        v-for="item in items"
        :key="item.id"
        :item="item"
        @dragstart="emit('dragstart', $event)"
        @click="emit('click', item)"
      />
      <el-empty v-if="items.length === 0" description="暂无工作项" :image-size="40" />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { WorkItem } from '../../types/domain'
import KanbanCard from './KanbanCard.vue'

const props = defineProps<{ title: string; items: WorkItem[]; status: string }>()
const emit = defineEmits<{
  (e: 'dragstart', item: WorkItem): void
  (e: 'drop', status: string): void
  (e: 'click', item: WorkItem): void
}>()

function onDrop() {
  emit('drop', props.status)
}
</script>

<style scoped>
.kanban-column {
  width: 260px;
  min-height: 400px;
  background: #f5f7fa;
  border-radius: 6px;
  padding: 12px;
  flex-shrink: 0;
}
.column-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}
.column-title { font-weight: bold; font-size: 14px; }
.column-body { min-height: 200px; }
</style>
