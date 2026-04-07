<template>
  <el-container class="app-layout">
    <el-aside width="220px">
      <AppSidebar />
    </el-aside>
    <el-container>
      <el-header>
        <AppHeader />
      </el-header>
      <el-main>
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import AppSidebar from './AppSidebar.vue'
import AppHeader from './AppHeader.vue'
</script>

<style scoped>
.app-layout {
  height: 100vh;
}
.el-header {
  padding: 0;
  border-bottom: 1px solid #e4e7ed;
}
.el-aside {
  border-right: 1px solid #e4e7ed;
  overflow: hidden;
}
</style>
