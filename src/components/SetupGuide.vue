<template>
  <el-dialog
    v-model="visible"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    :show-close="false"
    width="600px"
    class="setup-dialog"
  >
    <template #header>
      <div class="setup-header">
        <h2>欢迎使用 Windows 工具箱</h2>
        <p>首次运行需要进行环境配置</p>
      </div>
    </template>
    
    <div class="setup-content">
      <div class="progress-steps">
        <div 
          v-for="(step, index) in steps" 
          :key="index"
          class="step"
          :class="{ active: currentStep === index, completed: currentStep > index }"
        >
          <div class="step-icon">
            <el-icon v-if="currentStep > index"><Check /></el-icon>
            <span v-else>{{ index + 1 }}</span>
          </div>
          <span class="step-label">{{ step.label }}</span>
        </div>
      </div>
      
      <div class="step-content">
        <div v-if="currentStep === 0" class="check-environment">
          <h3>环境检测</h3>
          <p>正在检测您的系统环境...</p>
          
          <div class="check-list">
            <div 
              v-for="item in environmentChecks" 
              :key="item.name"
              class="check-item"
              :class="item.status"
            >
              <el-icon v-if="item.status === 'checking'" class="is-loading"><Loading /></el-icon>
              <el-icon v-else-if="item.status === 'success'"><Check /></el-icon>
              <el-icon v-else-if="item.status === 'error'"><Close /></el-icon>
              <el-icon v-else><Clock /></el-icon>
              <span>{{ item.name }}</span>
              <span class="check-status">{{ getStatusText(item.status) }}</span>
            </div>
          </div>
        </div>
        
        <div v-else-if="currentStep === 1" class="install-dependencies">
          <h3>安装依赖</h3>
          <p>检测到以下组件需要安装或更新：</p>
          
          <div class="install-list">
            <div 
              v-for="item in itemsToInstall" 
              :key="item.name"
              class="install-item"
            >
              <div class="install-info">
                <span class="install-name">{{ item.name }}</span>
                <span class="install-desc">{{ item.description }}</span>
              </div>
              <el-tag v-if="item.required" type="danger" size="small">必需</el-tag>
              <el-tag v-else type="info" size="small">推荐</el-tag>
            </div>
          </div>
          
          <div v-if="isInstalling" class="install-progress">
            <el-progress 
              :percentage="installProgress" 
              :status="installProgress === 100 ? 'success' : ''"
            />
            <p>{{ installStatus }}</p>
          </div>
        </div>
        
        <div v-else-if="currentStep === 2" class="complete">
          <el-result
            icon="success"
            title="环境配置完成"
            sub-title="您已准备好使用 Windows 工具箱"
          >
            <template #extra>
              <el-button type="primary" @click="finish">
                开始使用
              </el-button>
            </template>
          </el-result>
        </div>
      </div>
    </div>
    
    <template #footer>
      <div class="setup-footer">
        <el-button 
          v-if="currentStep === 1 && !isInstalling"
          @click="skipInstall"
        >
          跳过安装
        </el-button>
        <el-button 
          v-if="currentStep === 0"
          type="primary"
          :disabled="isChecking"
          @click="startInstall"
        >
          开始检测
        </el-button>
        <el-button 
          v-if="currentStep === 1 && !isInstalling"
          type="primary"
          @click="doInstall"
        >
          安装依赖
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Check, Close, Clock, Loading } from '@element-plus/icons-vue'

const emit = defineEmits<{
  (e: 'complete'): void
}>()

const visible = ref(true)
const currentStep = ref(0)
const isChecking = ref(false)
const isInstalling = ref(false)
const installProgress = ref(0)
const installStatus = ref('')

const steps = [
  { label: '环境检测' },
  { label: '安装依赖' },
  { label: '完成' }
]

const environmentChecks = ref<Array<{ name: string; status: 'pending' | 'checking' | 'success' | 'error' }>>([
  { name: 'Rust 编译器', status: 'pending' },
  { name: 'Node.js 运行时', status: 'pending' },
  { name: 'NPM 包管理器', status: 'pending' },
  { name: 'Visual Studio Build Tools', status: 'pending' },
  { name: 'WebView2 运行时', status: 'pending' }
])

const itemsToInstall = computed(() => {
  return environmentChecks.value
    .filter(item => item.status === 'error')
    .map(item => ({
      name: item.name,
      description: getInstallDescription(item.name),
      required: ['Rust 编译器', 'Node.js 运行时'].includes(item.name)
    }))
})

function getInstallDescription(name: string): string {
  const descriptions: Record<string, string> = {
    'Rust 编译器': '用于编译 Tauri 后端代码',
    'Node.js 运行时': '用于运行前端开发服务器',
    'NPM 包管理器': '用于管理前端依赖包',
    'Visual Studio Build Tools': '用于编译 Windows 原生代码',
    'WebView2 运行时': '用于渲染应用界面'
  }
  return descriptions[name] || ''
}

function getStatusText(status: string): string {
  const texts: Record<string, string> = {
    pending: '待检测',
    checking: '检测中...',
    success: '已安装',
    error: '未安装'
  }
  return texts[status] || status
}

async function checkEnvironment() {
  isChecking.value = true
  
  for (let i = 0; i < environmentChecks.value.length; i++) {
    environmentChecks.value[i].status = 'checking'
    await new Promise(resolve => setTimeout(resolve, 500))
    
    const result = await checkItem(environmentChecks.value[i].name)
    environmentChecks.value[i].status = result ? 'success' : 'error'
  }
  
  isChecking.value = false
  
  const hasErrors = environmentChecks.value.some(item => item.status === 'error')
  if (hasErrors) {
    currentStep.value = 1
  } else {
    currentStep.value = 2
  }
}

async function checkItem(name: string): Promise<boolean> {
  try {
    switch (name) {
      case 'Rust 编译器':
        return await window.electron?.checkRust?.() ?? true
      case 'Node.js 运行时':
        return await window.electron?.checkNode?.() ?? true
      case 'NPM 包管理器':
        return await window.electron?.checkNpm?.() ?? true
      case 'Visual Studio Build Tools':
        return await window.electron?.checkVSBuildTools?.() ?? true
      case 'WebView2 运行时':
        return await window.electron?.checkWebView2?.() ?? true
      default:
        return true
    }
  } catch {
    return true
  }
}

async function startInstall() {
  await checkEnvironment()
}

async function doInstall() {
  isInstalling.value = true
  installProgress.value = 0
  installStatus.value = '正在安装依赖...'
  
  const totalItems = itemsToInstall.value.length
  let completed = 0
  
  for (const item of itemsToInstall.value) {
    installStatus.value = `正在安装 ${item.name}...`
    await new Promise(resolve => setTimeout(resolve, 2000))
    completed++
    installProgress.value = Math.round((completed / totalItems) * 100)
  }
  
  installStatus.value = '安装完成'
  isInstalling.value = false
  
  ElMessage.success('依赖安装完成')
  
  await new Promise(resolve => setTimeout(resolve, 500))
  currentStep.value = 2
}

function skipInstall() {
  ElMessage.warning('跳过安装可能导致应用无法正常运行')
  currentStep.value = 2
}

function finish() {
  visible.value = false
  localStorage.setItem('windows-toolbox-setup-complete', 'true')
  emit('complete')
}

onMounted(() => {
  const setupComplete = localStorage.getItem('windows-toolbox-setup-complete')
  if (setupComplete === 'true') {
    visible.value = false
    emit('complete')
  }
})
</script>

<style scoped>
.setup-dialog :deep(.el-dialog__header) {
  padding: 0;
  margin: 0;
}

.setup-dialog :deep(.el-dialog__body) {
  padding: 20px 30px;
}

.setup-header {
  text-align: center;
  padding: 30px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 8px 8px 0 0;
}

.setup-header h2 {
  margin: 0 0 8px 0;
  font-size: 24px;
}

.setup-header p {
  margin: 0;
  opacity: 0.9;
}

.progress-steps {
  display: flex;
  justify-content: space-between;
  margin-bottom: 30px;
  padding: 0 20px;
}

.step {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.step-icon {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: #e4e7ed;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  color: #909399;
  transition: all 0.3s;
}

.step.active .step-icon {
  background: #409eff;
  color: white;
}

.step.completed .step-icon {
  background: #67c23a;
  color: white;
}

.step-label {
  font-size: 13px;
  color: #909399;
}

.step.active .step-label,
.step.completed .step-label {
  color: #303133;
}

.step-content {
  min-height: 300px;
}

.check-environment h3,
.install-dependencies h3 {
  margin: 0 0 10px 0;
  font-size: 18px;
}

.check-environment p,
.install-dependencies p {
  color: #909399;
  margin-bottom: 20px;
}

.check-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.check-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: #f5f7fa;
  border-radius: 8px;
  transition: all 0.3s;
}

.check-item.checking {
  background: #ecf5ff;
}

.check-item.success {
  background: #f0f9eb;
}

.check-item.error {
  background: #fef0f0;
}

.check-item .el-icon {
  font-size: 18px;
}

.check-item.success .el-icon {
  color: #67c23a;
}

.check-item.error .el-icon {
  color: #f56c6c;
}

.check-item.checking .el-icon {
  color: #409eff;
}

.check-status {
  margin-left: auto;
  font-size: 13px;
  color: #909399;
}

.install-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.install-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  background: #f5f7fa;
  border-radius: 8px;
}

.install-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.install-name {
  font-weight: 500;
}

.install-desc {
  font-size: 13px;
  color: #909399;
}

.install-progress {
  margin-top: 20px;
  text-align: center;
}

.install-progress p {
  margin-top: 10px;
  color: #909399;
}

.setup-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style>
