<template>
  <el-badge :value="store.unreadCount || ''" :hidden="store.unreadCount === 0">
    <el-button :icon="Bell" circle text @click="router.push('/notifications')" />
  </el-badge>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Bell } from '@element-plus/icons-vue'
import { useNotificationStore } from '../../stores/notification'

const store = useNotificationStore()
const router = useRouter()

onMounted(() => store.fetchUnreadCount())
</script>
