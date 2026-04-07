<template>
  <div class="project-detail" v-loading="loading">
    <template v-if="store.currentProject">
      <div class="page-header">
        <div>
          <h2>{{ store.currentProject.name }}</h2>
          <p class="desc">{{ store.currentProject.description }}</p>
        </div>
        <div class="actions">
          <el-button @click="router.push(`/projects/${id}/settings`)">项目设置</el-button>
        </div>
      </div>

      <el-row :gutter="16">
        <el-col :span="8">
          <el-card>
            <template #header>快速导航</template>
            <el-button text @click="router.push(`/projects/${id}/requirements`)">需求列表</el-button>
            <el-button text @click="router.push(`/projects/${id}/bugs`)">缺陷列表</el-button>
            <el-button text @click="router.push(`/projects/${id}/kanban`)">看板视图</el-button>
            <el-button text @click="router.push(`/projects/${id}/iterations`)">迭代管理</el-button>
            <el-button text @click="router.push(`/projects/${id}/reports`)">报表中心</el-button>
          </el-card>
        </el-col>
        <el-col :span="16">
          <el-card>
            <template #header>项目成员 ({{ store.members.length }})</template>
            <el-table :data="store.members" size="small">
              <el-table-column label="用户ID" prop="userId" />
              <el-table-column label="角色" prop="role">
                <template #default="{ row }">
                  <el-tag size="small">{{ row.role }}</el-tag>
                </template>
              </el-table-column>
              <el-table-column label="加入时间" prop="joinedAt">
                <template #default="{ row }">{{ row.joinedAt?.slice(0, 10) }}</template>
              </el-table-column>
            </el-table>
          </el-card>
        </el-col>
      </el-row>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useProjectStore } from '../../stores/project'

const route = useRoute()
const router = useRouter()
const store = useProjectStore()
const loading = ref(false)
const id = Number(route.params.id)

onMounted(async () => {
  loading.value = true
  try {
    await store.selectProject(id)
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
.project-detail { padding: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 24px; }
.desc { color: #909399; margin-top: 4px; }
.el-button { display: block; margin-bottom: 8px; }
</style>
