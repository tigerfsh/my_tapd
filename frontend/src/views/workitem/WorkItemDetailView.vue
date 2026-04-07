<template>
  <div class="work-item-detail" v-loading="loading">
    <template v-if="item">
      <div class="page-header">
        <div>
          <span class="number">{{ item.number }}</span>
          <h2>{{ item.title }}</h2>
        </div>
        <div class="actions">
          <el-select v-model="item.status" size="small" @change="handleStatusChange">
            <el-option label="待处理" value="pending" />
            <el-option label="进行中" value="in_progress" />
            <el-option label="已完成" value="done" />
            <el-option label="已关闭" value="closed" />
            <el-option label="已拒绝" value="rejected" />
            <el-option label="待验证" value="pending_verify" />
            <el-option label="修复中" value="fixing" />
          </el-select>
          <el-button type="danger" size="small" @click="handleDelete">删除</el-button>
        </div>
      </div>

      <el-row :gutter="24">
        <el-col :span="16">
          <el-card style="margin-bottom:16px">
            <template #header>详情</template>
            <p>{{ item.description || '暂无描述' }}</p>
            <el-divider />
            <el-descriptions :column="2" size="small">
              <el-descriptions-item label="类型">{{ item.itemType }}</el-descriptions-item>
              <el-descriptions-item label="优先级">{{ item.priority }}</el-descriptions-item>
              <el-descriptions-item label="状态"><StatusBadge :status="item.status" /></el-descriptions-item>
              <el-descriptions-item label="指派人">{{ item.assigneeId || '-' }}</el-descriptions-item>
              <el-descriptions-item label="截止日期">{{ item.dueDate?.slice(0, 10) || '-' }}</el-descriptions-item>
              <el-descriptions-item label="故事点">{{ item.storyPoints || '-' }}</el-descriptions-item>
              <el-descriptions-item v-if="item.severity" label="严重程度">{{ item.severity }}</el-descriptions-item>
            </el-descriptions>
          </el-card>

          <el-card>
            <template #header>评论</template>
            <CommentList
              :comments="comments"
              :project-id="projectId"
              :work-item-id="wid"
              @add="addComment"
              @delete="deleteComment"
            />
          </el-card>
        </el-col>

        <el-col :span="8">
          <el-card>
            <template #header>变更历史</template>
            <el-timeline>
              <el-timeline-item
                v-for="log in auditLog"
                :key="log.id"
                :timestamp="log.changedAt?.slice(0, 16)"
              >
                <b>{{ log.fieldName }}</b>: {{ log.oldValue }} → {{ log.newValue }}
              </el-timeline-item>
            </el-timeline>
            <el-empty v-if="auditLog.length === 0" description="暂无记录" :image-size="60" />
          </el-card>
        </el-col>
      </el-row>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { workItemsApi } from '../../api/workItems'
import type { WorkItem, Comment, AuditLog } from '../../types/domain'
import StatusBadge from '../../components/workitem/StatusBadge.vue'
import CommentList from '../../components/workitem/CommentList.vue'

const route = useRoute()
const router = useRouter()
const projectId = Number(route.params.id)
const wid = Number(route.params.wid)

const item = ref<WorkItem | null>(null)
const comments = ref<Comment[]>([])
const auditLog = ref<AuditLog[]>([])
const loading = ref(false)

onMounted(async () => {
  loading.value = true
  try {
    const [itemRes, commentsRes, auditRes] = await Promise.all([
      workItemsApi.get(projectId, wid),
      workItemsApi.getComments(projectId, wid),
      workItemsApi.getAuditLog(projectId, wid),
    ])
    item.value = itemRes.data.data
    comments.value = commentsRes.data.data
    auditLog.value = auditRes.data.data
  } finally {
    loading.value = false
  }
})

async function handleStatusChange() {
  if (!item.value) return
  await workItemsApi.update(projectId, wid, { status: item.value.status })
  ElMessage.success('状态已更新')
}

async function addComment(content: string) {
  const res = await workItemsApi.addComment(projectId, wid, content)
  comments.value.push(res.data.data)
}

async function deleteComment(commentId: number) {
  await workItemsApi.deleteComment(projectId, wid, commentId)
  comments.value = comments.value.filter((c) => c.id !== commentId)
}

async function handleDelete() {
  await ElMessageBox.confirm('确认删除该工作项？')
  await workItemsApi.delete(projectId, wid)
  ElMessage.success('已删除')
  router.back()
}
</script>

<style scoped>
.work-item-detail { padding: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 24px; }
.number { font-size: 12px; color: #909399; display: block; margin-bottom: 4px; }
.actions { display: flex; gap: 8px; align-items: center; }
</style>
