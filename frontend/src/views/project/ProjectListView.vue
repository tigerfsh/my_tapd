<template>
  <div class="project-list">
    <div class="page-header">
      <h2>我的项目</h2>
      <el-button type="primary" :icon="Plus" @click="showCreate = true">新建项目</el-button>
    </div>

    <el-row :gutter="16" v-loading="store.loading">
      <el-col :span="8" v-for="p in store.projects" :key="p.id">
        <el-card class="project-card" @click="router.push(`/projects/${p.id}`)">
          <div class="project-name">{{ p.name }}</div>
          <div class="project-desc">{{ p.description || '暂无描述' }}</div>
          <div class="project-meta">
            <el-tag size="small">{{ p.projectType }}</el-tag>
            <el-tag v-if="p.isArchived" type="info" size="small">已归档</el-tag>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-empty v-if="!store.loading && store.projects.length === 0" description="暂无项目" />

    <!-- Create Dialog -->
    <el-dialog v-model="showCreate" title="新建项目" width="500px">
      <el-form :model="createForm" :rules="createRules" ref="createFormRef">
        <el-form-item label="项目名称" prop="name">
          <el-input v-model="createForm.name" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input v-model="createForm.description" type="textarea" :rows="3" />
        </el-form-item>
        <el-form-item label="类型" prop="project_type">
          <el-select v-model="createForm.project_type">
            <el-option label="敏捷" value="agile" />
            <el-option label="瀑布" value="waterfall" />
          </el-select>
        </el-form-item>
        <el-form-item label="公开">
          <el-switch v-model="createForm.is_public" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showCreate = false">取消</el-button>
        <el-button type="primary" :loading="creating" @click="handleCreate">创建</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { useProjectStore } from '../../stores/project'
import { projectsApi } from '../../api/projects'

const store = useProjectStore()
const router = useRouter()
const showCreate = ref(false)
const creating = ref(false)
const createFormRef = ref()

const createForm = reactive({ name: '', description: '', project_type: 'agile', is_public: false })
const createRules = { name: [{ required: true, message: '请输入项目名称' }] }

onMounted(() => store.fetchProjects())

async function handleCreate() {
  await createFormRef.value?.validate()
  creating.value = true
  try {
    await projectsApi.create(createForm as any)
    ElMessage.success('项目创建成功')
    showCreate.value = false
    store.fetchProjects()
  } catch {
    ElMessage.error('创建失败')
  } finally {
    creating.value = false
  }
}
</script>

<style scoped>
.project-list { padding: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
.project-card { cursor: pointer; margin-bottom: 16px; }
.project-card:hover { box-shadow: 0 4px 12px rgba(0,0,0,0.1); }
.project-name { font-size: 16px; font-weight: bold; margin-bottom: 8px; }
.project-desc { color: #909399; font-size: 13px; margin-bottom: 12px; }
.project-meta { display: flex; gap: 8px; }
</style>
