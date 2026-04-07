<template>
  <div class="comment-list">
    <div v-for="c in comments" :key="c.id" class="comment-item">
      <div class="comment-header">
        <span class="author">用户 {{ c.authorId }}</span>
        <span class="time">{{ c.createdAt?.slice(0, 16) }}</span>
        <el-button v-if="canDelete(c)" type="danger" size="small" text @click="handleDelete(c.id)">
          删除
        </el-button>
      </div>
      <div class="comment-content">{{ c.content }}</div>
    </div>
    <el-empty v-if="comments.length === 0" description="暂无评论" :image-size="60" />
    <div class="add-comment">
      <el-input v-model="newComment" type="textarea" :rows="2" placeholder="添加评论..." />
      <el-button type="primary" size="small" :loading="submitting" @click="handleSubmit" style="margin-top:8px">
        提交
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import type { Comment } from '../../types/domain'
import { useAuthStore } from '../../stores/auth'

const props = defineProps<{
  comments: Comment[]
  projectId: number
  workItemId: number
}>()
const emit = defineEmits<{
  (e: 'add', content: string): void
  (e: 'delete', id: number): void
}>()

const auth = useAuthStore()
const newComment = ref('')
const submitting = ref(false)

function canDelete(c: Comment) {
  return auth.user?.id === c.authorId
}

async function handleSubmit() {
  if (!newComment.value.trim()) return
  submitting.value = true
  try {
    emit('add', newComment.value)
    newComment.value = ''
  } finally {
    submitting.value = false
  }
}

function handleDelete(id: number) {
  emit('delete', id)
}
</script>

<style scoped>
.comment-list { padding: 8px 0; }
.comment-item { padding: 12px 0; border-bottom: 1px solid #f0f0f0; }
.comment-header { display: flex; align-items: center; gap: 12px; margin-bottom: 6px; }
.author { font-weight: bold; font-size: 13px; }
.time { color: #909399; font-size: 12px; }
.comment-content { font-size: 14px; color: #303133; }
.add-comment { margin-top: 16px; }
</style>
