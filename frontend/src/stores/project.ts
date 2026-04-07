import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Project, Member } from '../types/domain'
import { projectsApi } from '../api/projects'

export const useProjectStore = defineStore('project', () => {
  const projects = ref<Project[]>([])
  const currentProject = ref<Project | null>(null)
  const members = ref<Member[]>([])
  const loading = ref(false)

  const currentProjectId = computed(() => currentProject.value?.id ?? null)

  async function fetchProjects() {
    loading.value = true
    try {
      const res = await projectsApi.list()
      projects.value = res.data.data
    } finally {
      loading.value = false
    }
  }

  async function selectProject(id: number) {
    const res = await projectsApi.get(id)
    currentProject.value = res.data.data
    const membersRes = await projectsApi.getMembers(id)
    members.value = membersRes.data.data
  }

  function clearProject() {
    currentProject.value = null
    members.value = []
  }

  return { projects, currentProject, members, loading, currentProjectId, fetchProjects, selectProject, clearProject }
})
