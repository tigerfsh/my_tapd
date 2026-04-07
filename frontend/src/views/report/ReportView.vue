<template>
  <div class="report-view">
    <h2>报表中心</h2>
    <el-tabs v-model="activeTab" @tab-change="loadReport">
      <el-tab-pane label="项目统计" name="stats">
        <el-card v-loading="loading">
          <el-descriptions :column="3" v-if="stats">
            <el-descriptions-item
              v-for="(val, key) in stats"
              :key="key"
              :label="String(key)"
            >{{ val }}</el-descriptions-item>
          </el-descriptions>
          <el-empty v-else description="暂无数据" />
        </el-card>
      </el-tab-pane>

      <el-tab-pane label="成员工作量" name="workload">
        <el-card v-loading="loading">
          <el-table :data="workloadRows" v-if="workloadRows.length">
            <el-table-column label="成员" prop="member" />
            <el-table-column label="工作项数" prop="count" />
          </el-table>
          <el-empty v-else description="暂无数据" />
        </el-card>
      </el-tab-pane>

      <el-tab-pane label="缺陷趋势" name="bug_trend">
        <el-card v-loading="loading">
          <el-empty description="暂无数据" />
        </el-card>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { reportsApi } from '../../api/reports'

const route = useRoute()
const projectId = Number(route.params.id)

const activeTab = ref('stats')
const loading = ref(false)
const stats = ref<Record<string, unknown> | null>(null)
const workloadRows = ref<{ member: string; count: number }[]>([])

onMounted(() => loadReport('stats'))

async function loadReport(tab: string) {
  loading.value = true
  try {
    if (tab === 'stats') {
      const res = await reportsApi.getProjectStats(projectId)
      stats.value = res.data.data
    } else if (tab === 'workload') {
      const res = await reportsApi.getMemberWorkload(projectId)
      const data = res.data.data as Record<string, unknown>
      workloadRows.value = Object.entries(data).map(([member, count]) => ({
        member,
        count: Number(count),
      }))
    }
  } catch {
    // ignore
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.report-view { padding: 24px; }
</style>
