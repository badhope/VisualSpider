<template>
  <el-tooltip
    :content="content"
    :placement="placement"
    :effect="effect"
    :disabled="!showTooltip"
  >
    <el-icon 
      class="help-tip-icon" 
      :size="size"
      @click="handleClick"
    >
      <QuestionFilled />
    </el-icon>
  </el-tooltip>
</template>

<script setup lang="ts">
import { QuestionFilled } from '@element-plus/icons-vue'

withDefaults(defineProps<{
  content?: string
  placement?: 'top' | 'bottom' | 'left' | 'right' | 'top-start' | 'top-end' | 'bottom-start' | 'bottom-end' | 'left-start' | 'left-end' | 'right-start' | 'right-end'
  effect?: 'dark' | 'light'
  size?: number
  showTooltip?: boolean
}>(), {
  content: '',
  placement: 'top',
  effect: 'dark',
  size: 16,
  showTooltip: true
})

const emit = defineEmits<{
  (e: 'click'): void
}>()

function handleClick() {
  emit('click')
}
</script>

<style scoped>
.help-tip-icon {
  color: #909399;
  cursor: pointer;
  transition: color 0.2s;
}

.help-tip-icon:hover {
  color: #409eff;
}
</style>
