<template>
  <div class="sidebar">
    <div class="logo">IT需求管理</div>
    <el-menu :default-active="activeMenu" router>
      <el-menu-item index="/projects">
        <el-icon><Folder /></el-icon>
        <span>项目列表</span>
      </el-menu-item>
      <template v-if="projectId">
        <el-menu-item :index="`/projects/${projectId}`">
          <el-icon><House /></el-icon>
          <span>项目概览</span>
        </el-menu-item>
        <el-menu-item :index="`/projects/${projectId}/requirements`">
          <el-icon><Document /></el-icon>
          <span>需求管理</span>
        </el-menu-item>
        <el-menu-item :index="`/projects/${projectId}/bugs`">
          <el-icon><Warning /></el-icon>
          <span>缺陷管理</span>
        </el-menu-item>
        <el-menu-item :index="`/projects/${projectId}/kanban`">
          <el-icon><Grid /></el-icon>
          <span>看板</span>
        </el-menu-item>
        <el-menu-item :index="`/projects/${projectId}/iterations`">
          <el-icon><Calendar /></el-icon>
          <span>迭代管理</span>
        </el-menu-item>
        <el-menu-item :index="`/projects/${projectId}/reports`">
          <el-icon><DataAnalysis /></el-icon>
          <span>报表</span>
        </el-menu-item>
        <el-menu-item :index="`/projects/${projectId}/settings`">
          <el-icon><Setting /></el-icon>
          <span>项目设置</span>
        </el-menu-item>
      </template>
    </el-menu>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { Folder, House, Document, Warning, Grid, Calendar, DataAnalysis, Setting } from '@element-plus/icons-vue'

const route = useRoute()
const activeMenu = computed(() => route.path)
const projectId = computed(() => route.params.id as string | undefined)
</script>

<style scoped>
.sidebar {
  height: 100%;
  display: flex;
  flex-direction: column;
}
.logo {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  font-weight: bold;
  color: #409eff;
  border-bottom: 1px solid #e4e7ed;
}
.el-menu {
  border-right: none;
  flex: 1;
}
</style>
