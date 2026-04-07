<template>
  <div class="iteration-list">
    <div class="page-header">
      <h2>迭代管理</h2>
      <el-button type="primary" :icon="Plus" @click="showCreate = true">新建迭代</el-button>
    </div>

    <el-table :data="iterations" v-loading="loading" @row-click="goDetail">
      <el-table-column label="名称" prop="name" />
      <el-table-column label="目标" prop="goal" />
      <el-table-column label="开始日期" width="120">
        <template #default="{ row }">{{ row.startDate?.slice(0, 10) }}</template>
      </el-table-column>
      <el-table-column label="结束日期" width="120">
        <template #default="{ row }">{{ row.endDate?.slice(0, 10) }}</template>
      </el-table-column>
      <el-table-column label="状态" width="100">
        <template #default="{ row }">
          <el-tag :type="statusType(row.status)" size="small">{{ statusLabel(row.status) }}</el-tag>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="showCreate" title="新建迭代" width="500px">
      <el-form :model="createForm" :rules="createRules" ref="createFormRef" label-width="90px">
        <el-form-item label="名称" prop="name">
          <el-input v-model="createForm.name" />
        </el-form-item>
        <el-form-item label="目标">
          <el-input v-model="createForm.goal" type="textarea" :rows="2" />
        </el-form-item>
        <el-form-item label="开始日期" prop="start_date">
          <el-date-picker v-model="createForm.start_date" type="date" value-format="YYYY-MM-DD" />
        </el-form-item>
        <el-form-item label="结束日期" prop="end_date">
          <el-date-picker v-model="createForm.end_date" type="date" value-format="YYYY-MM-DD" />
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
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { iterationsApi } from '../../api/iterations'
import type { Iteration } from '../../types/domain'

const route = useRoute()
const router = useRouter()
const projectId = Number(route.params.id)

const iterations = ref<Iteration[]>([])
const loading = ref(false)
const showCreate = ref(false)
const creating = ref(false)
const createFormRef = ref()
const createForm = reactive({ name: '', goal: '', start_date: '', end_date: '' })
const createRules = {
  name: [{ required: true, message: '请输入迭代名称' }],
  start_date: [{ required: true, message: '请选择开始日期' }],
  end_date: [{ required: true, message: '请选择结束日期' }],
}

onMounted(async () => {
  loading.value = true
  try {
    const res = await iterationsApi.list(projectId)
    iterations.value = res.data.data
  } finally {
    loading.value = false
  }
})

function goDetail(row: Iteration) {
  router.push(`/projects/${projectId}/iterations/${row.id}`)
}

function statusType(s: string) {
  return s === 'completed' ? 'success' : s === 'in_progress' ? 'primary' : 'info'
}
function statusLabel(s: string) {
  return { not_started: '未开始', in_progress: '进行中', completed: '已完成' }[s] || s
}

async function handleCreate() {
  await createFormRef.value?.validate()
  creating.value = true
  try {
    await iterationsApi.create(projectId, createForm as any)
    ElMessage.success('迭代创建成功')
    showCreate.value = false
    const res = await iterationsApi.list(projectId)
    iterations.value = res.data.data
  } catch { ElMessage.error('创建失败') } finally { creating.value = false }
}
</script>

<style scoped>
.iteration-list { padding: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
</style>
