<template>
  <el-form :model="form" :rules="rules" ref="formRef" label-width="80px">
    <el-form-item label="标题" prop="title">
      <el-input v-model="form.title" />
    </el-form-item>
    <el-form-item label="描述">
      <el-input v-model="form.description" type="textarea" :rows="4" />
    </el-form-item>
    <el-form-item label="优先级" prop="priority">
      <el-select v-model="form.priority">
        <el-option label="紧急" value="urgent" />
        <el-option label="高" value="high" />
        <el-option label="中" value="medium" />
        <el-option label="低" value="low" />
      </el-select>
    </el-form-item>
    <template v-if="itemType === 'bug'">
      <el-form-item label="严重程度" prop="severity">
        <el-select v-model="form.severity">
          <el-option label="致命" value="fatal" />
          <el-option label="严重" value="critical" />
          <el-option label="一般" value="normal" />
          <el-option label="提示" value="hint" />
        </el-select>
      </el-form-item>
      <el-form-item label="复现步骤">
        <el-input v-model="form.repro_steps" type="textarea" :rows="3" />
      </el-form-item>
    </template>
    <el-form-item label="指派给">
      <el-input-number v-model="form.assignee_id" :min="1" placeholder="用户ID" />
    </el-form-item>
    <el-form-item label="截止日期">
      <el-date-picker v-model="form.due_date" type="date" value-format="YYYY-MM-DD" />
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import type { Priority, Severity } from '../../types/enums'

const props = defineProps<{ itemType: 'requirement' | 'bug' }>()
const emit = defineEmits<{ (e: 'submit', data: Record<string, unknown>): void }>()

const formRef = ref()
const form = reactive({
  title: '',
  description: '',
  priority: 'medium' as Priority,
  severity: 'normal' as Severity,
  repro_steps: '',
  assignee_id: undefined as number | undefined,
  due_date: '',
  labels: [] as string[],
})

const rules = {
  title: [{ required: true, message: '请输入标题' }],
  priority: [{ required: true, message: '请选择优先级' }],
  severity: props.itemType === 'bug' ? [{ required: true, message: '请选择严重程度' }] : [],
}

async function validate() {
  await formRef.value?.validate()
  return { ...form }
}

defineExpose({ validate })
</script>
