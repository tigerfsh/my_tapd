<template>
  <div class="app-header">
    <SearchBar class="header-search" />
    <div class="header-right">
      <NotificationBell />
      <el-dropdown @command="handleCommand">
        <div class="user-info">
          <el-avatar :size="32" :src="auth.user?.avatarUrl">
            {{ auth.user?.nickname?.charAt(0) }}
          </el-avatar>
          <span class="username">{{ auth.user?.nickname }}</span>
        </div>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item command="logout">退出登录</el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useAuthStore } from '../../stores/auth'
import NotificationBell from '../common/NotificationBell.vue'
import SearchBar from '../common/SearchBar.vue'

const auth = useAuthStore()
const router = useRouter()

function handleCommand(cmd: string) {
  if (cmd === 'logout') {
    auth.logout()
    router.push('/login')
  }
}
</script>

<style scoped>
.app-header {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
}
.header-search {
  width: 300px;
}
.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}
.user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}
.username {
  font-size: 14px;
}
</style>
