import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { WorkItem } from '../types/domain'
import type { WorkItemFilter } from '../types/api'
import { workItemsApi } from '../api/workItems'

export const useWorkItemStore = defineStore('workitem', () => {
  const items = ref<WorkItem[]>([])
  const currentItem = ref<WorkItem | null>(null)
  const loading = ref(false)
  const filter = ref<WorkItemFilter>({})

  const filteredItems = computed(() => {
    let result = items.value
    if (filter.value.status) result = result.filter((i) => i.status === filter.value.status)
    if (filter.value.priority) result = result.filter((i) => i.priority === filter.value.priority)
    if (filter.value.assignee_id) result = result.filter((i) => i.assigneeId === filter.value.assignee_id)
    if (filter.value.iteration_id) result = result.filter((i) => i.iterationId === filter.value.iteration_id)
    if (filter.value.item_type) result = result.filter((i) => i.itemType === filter.value.item_type)
    return result
  })

  async function fetchItems(projectId: number, f?: WorkItemFilter) {
    loading.value = true
    try {
      const res = await workItemsApi.list(projectId, f)
      items.value = res.data.data
    } finally {
      loading.value = false
    }
  }

  async function fetchItem(projectId: number, id: number) {
    const res = await workItemsApi.get(projectId, id)
    currentItem.value = res.data.data
    // Update cache
    const idx = items.value.findIndex((i) => i.id === id)
    if (idx >= 0) items.value[idx] = res.data.data
    return res.data.data
  }

  function setFilter(f: WorkItemFilter) {
    filter.value = f
  }

  function clearItems() {
    items.value = []
    currentItem.value = null
  }

  return { items, currentItem, loading, filter, filteredItems, fetchItems, fetchItem, setFilter, clearItems }
})
