<template>
  <div class="project-settings" v-loading="loading">
    <h2>项目设置</h2>
    <el-tabs>
      <el-tab-pane label="基本信息">
        <el-form :model="form" label-width="100px" style="max-width:500px">
          <el-form-item label="项目名称">
            <el-input v-model="form.name" />
          </el-form-item>
          <el-form-item label="描述">
            <el-input v-model="form.description" type="textarea" :rows="3" />
          </el-form-item>
          <el-form-item label="公开">
            <el-switch v-model="form.is_public" />
          </el-form-item>
          <el-form-item>
            <el-button type="primary" :loading="saving" @click="handleSave">保存</el-button>
          </el-form-item>
        </el-form>
      </el-tab-pane>
      <el-tab-pane label="成员管理">
        <el-table :data="store.members" style="margin-bottom:16px">
          <el-table-column label="用户ID" prop="userId" />
          <el-table-column label="角色" prop="role">
            <template #default="{ row }">
              <el-select v-model="row.role" size="small" @change="updateRole(row)">
                <el-option label="管理员" value="admin" />
                <el-option label="开发者" value="developer" />
                <el-option label="测试" value="tester" />
                <el-option label="观察者" value="observer" />
              </el-select>
            </template>
          </el-table-column>
          <el-table-column label="操作">
            <template #default="{ row }">
              <el-button type="danger" size="small" text @click="removeMember(row.userId)">移除</el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>
      <el-tab-pane label="危险操作">
        <el-button type="warning" @click="handleArchive">归档项目</el-button>
        <el-button type="danger" @click="handleDelete">删除项目</el-button>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useProjectStore } from '../../stores/project'
import { projectsApi } from '../../api/projects'
import type { Role } from '../../types/enums'

const route = useRoute()
const router = useRouter()
const store = useProjectStore()
const loading = ref(false)
const saving = ref(false)
const id = Number(route.params.id)

const form = reactive({ name: '', description: '', is_public: false })

onMounted(async () => {
  loading.value = true
  await store.selectProject(id)
  const p = store.currentProject
  if (p) { form.name = p.name; form.description = p.description || ''; form.is_public = p.isPublic }
  loading.value = false
})

async function handleSave() {
  saving.value = true
  try {
    await projectsApi.update(id, form as any)
    ElMessage.success('保存成功')
  } catch { ElMessage.error('保存失败') } finally { saving.value = false }
}

async function updateRole(member: { userId: number; role: Role }) {
  await projectsApi.updateMember(id, member.userId, member.role)
  ElMessage.success('角色已更新')
}

async function removeMember(userId: number) {
  await ElMessageBox.confirm('确认移除该成员？')
  await projectsApi.removeMember(id, userId)
  await store.selectProject(id)
  ElMessage.success('已移除')
}

async function handleArchive() {
  await ElMessageBox.confirm('确认归档该项目？')
  await projectsApi.archive(id)
  ElMessage.success('已归档')
  router.push('/projects')
}

async function handleDelete() {
  await ElMessageBox.confirm('确认删除该项目？此操作不可恢复！', '警告', { type: 'warning' })
  await projectsApi.delete(id)
  ElMessage.success('已删除')
  router.push('/projects')
}
</script>

<style scoped>
.project-settings { padding: 24px; }
</style>
