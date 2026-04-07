<template>
  <el-autocomplete
    v-model="keyword"
    :fetch-suggestions="search"
    placeholder="搜索工作项..."
    :trigger-on-focus="false"
    clearable
    @select="handleSelect"
  >
    <template #default="{ item }">
      <div class="search-item">
        <span class="item-number">{{ item.number }}</span>
        <span class="item-title">{{ item.title }}</span>
        <el-tag size="small" style="margin-left:8px">{{ item.itemType }}</el-tag>
      </div>
    </template>
  </el-autocomplete>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import client from '../../api/client'
import type { WorkItem } from '../../types/domain'

const router = useRouter()
const keyword = ref('')

async function search(query: string, cb: (results: WorkItem[]) => void) {
  if (!query.trim()) { cb([]); return }
  try {
    const res = await client.get<{ data: WorkItem[] }>('/search', { params: { keyword: query } })
    cb(res.data.data || [])
  } catch {
    cb([])
  }
}

function handleSelect(item: WorkItem) {
  router.push(`/projects/${item.projectId}/work-items/${item.id}`)
  keyword.value = ''
}
</script>

<style scoped>
.search-item { display: flex; align-items: center; }
.item-number { font-size: 12px; color: #909399; margin-right: 8px; }
.item-title { flex: 1; font-size: 13px; }
</style>
