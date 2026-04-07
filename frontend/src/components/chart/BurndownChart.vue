<template>
  <div ref="chartEl" class="burndown-chart" />
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import type { BurndownSnapshot } from '../../types/domain'

const props = defineProps<{ snapshots: BurndownSnapshot[] }>()
const chartEl = ref<HTMLElement>()

async function renderChart() {
  if (!chartEl.value || props.snapshots.length === 0) return
  try {
    const echarts = await import('echarts')
    const chart = echarts.init(chartEl.value)
    const dates = props.snapshots.map((s) => s.snapshotDate.slice(0, 10))
    const remaining = props.snapshots.map((s) => s.remainingPoints)
    const total = props.snapshots[0]?.totalPoints ?? 0
    const ideal = dates.map((_, i) => {
      const step = total / (dates.length - 1 || 1)
      return Math.max(0, total - step * i)
    })
    chart.setOption({
      title: { text: '燃尽图' },
      tooltip: { trigger: 'axis' },
      legend: { data: ['剩余点数', '理想线'] },
      xAxis: { type: 'category', data: dates },
      yAxis: { type: 'value' },
      series: [
        { name: '剩余点数', type: 'line', data: remaining },
        { name: '理想线', type: 'line', data: ideal, lineStyle: { type: 'dashed' } },
      ],
    })
  } catch {
    // echarts not available
  }
}

onMounted(renderChart)
watch(() => props.snapshots, renderChart)
</script>

<style scoped>
.burndown-chart { width: 100%; height: 300px; }
</style>
