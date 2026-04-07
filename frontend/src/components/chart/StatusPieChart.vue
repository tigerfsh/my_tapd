<template>
  <div ref="chartEl" class="pie-chart" />
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'

const props = defineProps<{ data: { name: string; value: number }[] }>()
const chartEl = ref<HTMLElement>()

async function renderChart() {
  if (!chartEl.value || props.data.length === 0) return
  try {
    const echarts = await import('echarts')
    const chart = echarts.init(chartEl.value)
    chart.setOption({
      title: { text: '状态分布', left: 'center' },
      tooltip: { trigger: 'item' },
      series: [{
        type: 'pie',
        radius: '60%',
        data: props.data,
        emphasis: { itemStyle: { shadowBlur: 10 } },
      }],
    })
  } catch {
    // echarts not available
  }
}

onMounted(renderChart)
watch(() => props.data, renderChart)
</script>

<style scoped>
.pie-chart { width: 100%; height: 300px; }
</style>
