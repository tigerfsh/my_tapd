<template>
  <el-tag :type="tagType" size="small">{{ label }}</el-tag>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Status } from '../../types/enums'

const props = defineProps<{ status: Status }>()

const statusMap: Record<Status, { label: string; type: string }> = {
  [Status.Pending]: { label: '待处理', type: 'info' },
  [Status.InProgress]: { label: '进行中', type: 'primary' },
  [Status.Done]: { label: '已完成', type: 'success' },
  [Status.Closed]: { label: '已关闭', type: 'info' },
  [Status.Rejected]: { label: '已拒绝', type: 'danger' },
  [Status.PendingVerify]: { label: '待验证', type: 'warning' },
  [Status.Fixing]: { label: '修复中', type: 'warning' },
  [Status.Unassigned]: { label: '未分配', type: '' },
}

const tagType = computed(() => statusMap[props.status]?.type || '')
const label = computed(() => statusMap[props.status]?.label || props.status)
</script>
