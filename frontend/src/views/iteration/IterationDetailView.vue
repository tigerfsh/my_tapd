<template>
  <div class="iteration-detail" v-loading="loading">
    <template v-if="iteration">
      <div class="page-header">
        <div>
          <h2>{{ iteration.name }}</h2>
          <p class="goal">{{ iteration.goal }}</p>
        </div>
        <el-tag :type="statusType(iteration.status)">{{ statusLabel(iteration.status) }}</el-tag>
      </div>

      <el-row :gutter="16">
        <el-col :span="16">
          <el-card style="margin-bottom:16px">
            <template #header>工作项</template>
            <el-table :data="workItems" size="small">
              <el-table-column label="编号" prop="number" width="100" />
              <el-table-column label="标题" prop="title" />
              <el-table-column label="状态" width="100">
                <template #default="{ row }"><StatusBadge :status="row.status" /></template>
              </el-table-column>
              <el-table-column label="故事点" prop="storyPoints" width="80" />
            </el-table>
          </el-card>
          <el-card>
            <template #header>燃尽图</template>
            <BurndownChart :snapshots="burndown" />
          </el-card>
        </el-col>
        <el-col :span="8">
          <el-card>
            <template #header>状态分布</template>
            <StatusPieChart :data="pieData" />
          </el-card>
        </el-col>
      </el-row>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { iterationsApi } from '../../api/iterations'
import type { Iteration, WorkItem, BurndownSnapshot } from '../../types/domain'
import StatusBadge from '../../components/workitem/StatusBadge.vue'
import BurndownChart from '../../components/chart/BurndownChart.vue'
import StatusPieChart from '../../components/chart/StatusPieChart.vue'

const route = useRoute()
const projectId = Number(route.params.id)
const iid = Number(route.params.iid)

const iteration = ref<Iteration | null>(null)
const workItems = ref<WorkItem[]>([])
const burndown = ref<BurndownSnapshot[]>([])
const loading = ref(false)

const pieData = computed(() => {
  const counts: Record<string, number> = {}
  workItems.value.forEach((w) => { counts[w.status] = (counts[w.status] || 0) + 1 })
  return Object.entries(counts).map(([name, value]) => ({ name, value }))
})

onMounted(async () => {
  loading.value = true
  try {
    const [iterRes, itemsRes, burndownRes] = await Promise.all([
      iterationsApi.get(projectId, iid),
      iterationsApi.getWorkItems(projectId, iid),
      iterationsApi.getBurndown(projectId, iid),
    ])
    iteration.value = iterRes.data.data
    workItems.value = itemsRes.data.data
    burndown.value = burndownRes.data.data
  } finally {
    loading.value = false
  }
})

function statusType(s: string) {
  return s === 'completed' ? 'success' : s === 'in_progress' ? 'primary' : 'info'
}
function statusLabel(s: string) {
  return { not_started: '未开始', in_progress: '进行中', completed: '已完成' }[s] || s
}
</script>

<style scoped>
.iteration-detail { padding: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 24px; }
.goal { color: #909399; margin-top: 4px; }
</style>
