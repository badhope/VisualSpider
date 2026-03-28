<template>
  <div class="dashboard">
    <el-row :gutter="20">
      <el-col :span="16">
        <el-card class="system-info-card">
          <template #header>
            <div class="card-header">
              <el-icon><Monitor /></el-icon>
              <span>{{ $t('dashboard.systemInfo') }}</span>
            </div>
          </template>
          <el-descriptions :column="2" border v-loading="loading">
            <el-descriptions-item :label="$t('dashboard.osName')">
              {{ systemInfo.osName }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('dashboard.osVersion')">
              {{ systemInfo.osVersion }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('dashboard.computerName')">
              {{ systemInfo.computerName }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('dashboard.userName')">
              {{ systemInfo.userName }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('dashboard.cpu')">
              {{ systemInfo.cpu }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('dashboard.ram')">
              {{ formatBytes(systemInfo.ram) }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('dashboard.architecture')">
              {{ systemInfo.architecture }}
            </el-descriptions-item>
          </el-descriptions>
        </el-card>
      </el-col>
      
      <el-col :span="8">
        <el-card class="quick-access-card">
          <template #header>
            <div class="card-header">
              <el-icon><Position /></el-icon>
              <span>{{ $t('dashboard.quickAccess') }}</span>
            </div>
          </template>
          <div class="quick-buttons">
            <el-button type="primary" @click="$router.push('/powershell')">
              <el-icon><SetUp /></el-icon>
              PowerShell
            </el-button>
            <el-button type="success" @click="$router.push('/registry')">
              <el-icon><Collection /></el-icon>
              {{ $t('nav.registry') }}
            </el-button>
            <el-button type="warning" @click="$router.push('/services')">
              <el-icon><Service /></el-icon>
              {{ $t('nav.services') }}
            </el-button>
            <el-button type="info" @click="$router.push('/processes')">
              <el-icon><DataLine /></el-icon>
              {{ $t('nav.processes') }}
            </el-button>
            <el-button @click="$router.push('/network')">
              <el-icon><Connection /></el-icon>
              {{ $t('nav.network') }}
            </el-button>
            <el-button @click="$router.push('/disk')">
              <el-icon><FolderOpened /></el-icon>
              {{ $t('nav.disk') }}
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="20" style="margin-top: 20px;">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header">
              <el-icon><Clock /></el-icon>
              <span>{{ $t('dashboard.recentActions') }}</span>
            </div>
          </template>
          <el-timeline>
            <el-timeline-item
              v-for="(action, index) in recentActions"
              :key="index"
              :timestamp="action.time"
              placement="top"
            >
              <el-card>
                <h4>{{ action.title }}</h4>
                <p>{{ action.description }}</p>
              </el-card>
            </el-timeline-item>
          </el-timeline>
          <el-empty v-if="recentActions.length === 0" :description="$t('dashboard.noRecentActions')" />
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Monitor, Position, SetUp, Collection, Service, DataLine, Connection, FolderOpened, Clock } from '@element-plus/icons-vue'
import type { SystemInfo } from '@/types'

const loading = ref(false)
const systemInfo = ref<SystemInfo>({
  osName: '',
  osVersion: '',
  osBuild: '',
  computerName: '',
  userName: '',
  cpu: '',
  ram: 0,
  architecture: ''
})

const recentActions = ref<Array<{ title: string; description: string; time: string }>>([])

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

async function loadSystemInfo() {
  loading.value = true
  try {
    const result = await invoke<SystemInfo>('get_system_info')
    systemInfo.value = result
  } catch (error) {
    console.error('Failed to load system info:', error)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadSystemInfo()
})
</script>

<style scoped>
.dashboard {
  padding: 0;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
}

.quick-buttons {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.quick-buttons .el-button {
  width: 100%;
  justify-content: flex-start;
}

:deep(.el-timeline-item__timestamp) {
  font-size: 12px;
}
</style>
