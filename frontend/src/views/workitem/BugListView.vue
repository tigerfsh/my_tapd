<template>
  <div class="bug-list">
    <div class="page-header">
      <h2>缺陷列表</h2>
      <el-button type="primary" :icon="Plus" @click="showCreate = true">新建缺陷</el-button>
    </div>

    <el-form inline class="filter-bar">
      <el-form-item label="状态">
        <el-select v-model="filter.status" clearable placeholder="全部" @change="loadItems">
          <el-option label="未分配" value="unassigned" />
          <el-option label="修复中" value="fixing" />
          <el-option label="待验证" value="pending_verify" />
          <el-option label="已关闭" value="closed" />
          <el-option label="已拒绝" value="rejected" />
        </el-select>
      </el-form-item>
      <el-form-item label="优先级">
        <el-select v-model="filter.priority" clearable placeholder="全部" @change="loadItems">
          <el-option label="紧急" value="urgent" />
          <el-option label="高" value="high" />
          <el-option label="中" value="medium" />
          <el-option label="低" value="low" />
        </el-select>
      </el-form-item>
    </el-form>

    <el-table :data="items" v-loading="loading" @row-click="goDetail">
      <el-table-column label="编号" prop="number" width="100" />
      <el-table-column label="标题" prop="title" />
      <el-table-column label="状态" width="100">
        <template #default="{ row }"><StatusBadge :status="row.status" /></template>
      </el-table-column>
      <el-table-column label="严重程度" prop="severity" width="100" />
      <el-table-column label="优先级" prop="priority" width="80" />
      <el-table-column label="指派人" prop="assigneeId" width="80" />
    </el-table>

    <el-dialog v-model="showCreate" title="新建缺陷" width="600px">
      <WorkItemForm ref="formRef" item-type="bug" />
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
import { workItemsApi } from '../../api/workItems'
import type { WorkItem } from '../../types/domain'
import type { WorkItemFilter } from '../../types/api'
import StatusBadge from '../../components/workitem/StatusBadge.vue'
import WorkItemForm from '../../components/workitem/WorkItemForm.vue'

const route = useRoute()
const router = useRouter()
const projectId = Number(route.params.id)

const items = ref<WorkItem[]>([])
const loading = ref(false)
const showCreate = ref(false)
const creating = ref(false)
const formRef = ref()
const filter = reactive<WorkItemFilter>({})

onMounted(loadItems)

async function loadItems() {
  loading.value = true
  try {
    const res = await workItemsApi.list(projectId, { ...filter, item_type: 'bug' as any })
    items.value = res.data.data
  } finally {
    loading.value = false
  }
}

function goDetail(row: WorkItem) {
  router.push(`/projects/${projectId}/work-items/${row.id}`)
}

async function handleCreate() {
  const data = await formRef.value?.validate()
  creating.value = true
  try {
    await workItemsApi.createBug(projectId, data)
    ElMessage.success('缺陷创建成功')
    showCreate.value = false
    loadItems()
  } catch { ElMessage.error('创建失败') } finally { creating.value = false }
}
</script>

<style scoped>
.bug-list { padding: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
.filter-bar { margin-bottom: 16px; }
</style>
