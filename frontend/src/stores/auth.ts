import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { User } from '../types/domain'
import { authApi } from '../api/auth'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const token = ref<string | null>(localStorage.getItem('access_token'))

  const isLoggedIn = computed(() => !!token.value)

  async function login(email: string, password: string) {
    const res = await authApi.login({ email, password })
    token.value = res.data.data.access_token
    localStorage.setItem('access_token', token.value)
    await fetchMe()
  }

  async function fetchMe() {
    const res = await authApi.getMe()
    user.value = res.data.data
  }

  function logout() {
    authApi.logout().catch(() => {})
    token.value = null
    user.value = null
    localStorage.removeItem('access_token')
  }

  return { user, token, isLoggedIn, login, fetchMe, logout }
})
