<template>
  <el-tour
    v-model="current"
    :steps="steps"
    :show-close="false"
    @finish="handleFinish"
    @close="handleClose"
  >
    <template #default="{ step, current: currentStep }">
      <div class="tour-content">
        <div class="tour-header">
          <h3>{{ step.title }}</h3>
          <el-tag size="small">{{ currentStep + 1 }} / {{ steps.length }}</el-tag>
        </div>
        <p>{{ step.description }}</p>
        <div v-if="step.tips" class="tour-tips">
          <el-alert
            v-for="(tip, index) in step.tips"
            :key="index"
            :title="tip"
            type="info"
            :closable="false"
            show-icon
            style="margin-bottom: 8px;"
          />
        </div>
        <div class="tour-footer">
          <el-checkbox v-model="dontShowAgain">{{ $t('guide.dontShowAgain') }}</el-checkbox>
          <div class="tour-buttons">
            <el-button v-if="currentStep > 0" @click="prev">{{ $t('guide.prev') }}</el-button>
            <el-button v-if="currentStep < steps.length - 1" type="primary" @click="next">
              {{ $t('guide.next') }}
            </el-button>
            <el-button v-else type="primary" @click="handleFinish">
              {{ $t('guide.finish') }}
            </el-button>
          </div>
        </div>
      </div>
    </template>
  </el-tour>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const emit = defineEmits<{
  (e: 'finish'): void
  (e: 'close'): void
}>()

const current = ref(0)
const dontShowAgain = ref(false)

const steps = computed(() => [
  {
    target: '.sidebar',
    title: t('guide.step1Title'),
    description: t('guide.step1Desc'),
    tips: [t('guide.step1Tip')]
  },
  {
    target: '.main-header',
    title: t('guide.step2Title'),
    description: t('guide.step2Desc'),
    tips: [t('guide.step2Tip')]
  },
  {
    target: '.sidebar-menu',
    title: t('guide.step3Title'),
    description: t('guide.step3Desc'),
    tips: [t('guide.step3Tip1'), t('guide.step3Tip2')]
  },
  {
    target: '.sidebar-footer',
    title: t('guide.step4Title'),
    description: t('guide.step4Desc'),
    tips: [t('guide.step4Tip')]
  }
])

function prev() {
  current.value--
}

function next() {
  current.value++
}

function handleFinish() {
  if (dontShowAgain.value) {
    localStorage.setItem('visual-spider-guide-completed', 'true')
  }
  emit('finish')
}

function handleClose() {
  if (dontShowAgain.value) {
    localStorage.setItem('visual-spider-guide-completed', 'true')
  }
  emit('close')
}

onMounted(() => {
  const completed = localStorage.getItem('visual-spider-guide-completed')
  if (completed === 'true') {
    emit('finish')
  }
})
</script>

<style scoped>
.tour-content {
  padding: 8px;
}

.tour-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.tour-header h3 {
  margin: 0;
  font-size: 16px;
  color: #303133;
}

.tour-content > p {
  margin: 0 0 16px 0;
  color: #606266;
  line-height: 1.6;
}

.tour-tips {
  margin-bottom: 16px;
}

.tour-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: 12px;
  border-top: 1px solid #ebeef5;
}

.tour-buttons {
  display: flex;
  gap: 8px;
}
</style>
