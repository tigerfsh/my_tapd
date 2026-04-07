import { ref, computed } from 'vue'

export function usePagination(defaultPageSize = 20) {
  const page = ref(1)
  const pageSize = ref(defaultPageSize)
  const total = ref(0)

  const totalPages = computed(() => Math.ceil(total.value / pageSize.value))
  const hasNext = computed(() => page.value < totalPages.value)
  const hasPrev = computed(() => page.value > 1)

  function nextPage() {
    if (hasNext.value) page.value++
  }

  function prevPage() {
    if (hasPrev.value) page.value--
  }

  function goToPage(p: number) {
    if (p >= 1 && p <= totalPages.value) page.value = p
  }

  function reset() {
    page.value = 1
    total.value = 0
  }

  return { page, pageSize, total, totalPages, hasNext, hasPrev, nextPage, prevPage, goToPage, reset }
}
