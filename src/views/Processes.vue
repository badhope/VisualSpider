<template>
  <div class="processes">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>{{ $t('processes.title') }}</span>
          <div class="header-actions">
            <el-input
              v-model="searchText"
              :placeholder="$t('common.search')"
              clearable
              style="width: 200px; margin-right: 12px;"
            >
              <template #prefix>
                <el-icon><Search /></el-icon>
              </template>
            </el-input>
            <el-button @click="loadProcesses">
              <el-icon><Refresh /></el-icon>
              {{ $t('processes.refresh') }}
            </el-button>
          </div>
        </div>
      </template>
      
      <el-table :data="filteredProcesses" v-loading="loading" border stripe max-height="600">
        <el-table-column prop="pid" :label="$t('processes.pid')" width="100" />
        <el-table-column prop="name" :label="$t('processes.name')" width="200" show-overflow-tooltip />
        <el-table-column prop="cpu" :label="$t('processes.cpu')" width="100">
          <template #default="{ row }">
            {{ row.cpu ? row.cpu.toFixed(2) + '%' : '0%' }}
          </template>
        </el-table-column>
        <el-table-column prop="memory" :label="$t('processes.memory')" width="120">
          <template #default="{ row }">
            {{ formatBytes(row.memory) }}
          </template>
        </el-table-column>
        <el-table-column prop="user" :label="$t('processes.user')" width="120" show-overflow-tooltip />
        <el-table-column prop="priority" :label="$t('processes.priority')" width="100" />
        <el-table-column prop="path" label="路径" show-overflow-tooltip />
        <el-table-column label="操作" width="120" fixed="right">
          <template #default="{ row }">
            <el-button type="danger" size="small" @click="endProcess(row)">
              {{ $t('processes.endProcess') }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search, Refresh } from '@element-plus/icons-vue'
import { useAppStore } from '@/stores'
import type { ProcessInfo } from '@/types'

const appStore = useAppStore()
const loading = ref(false)
const searchText = ref('')
const processes = ref<ProcessInfo[]>([])
let refreshInterval: number | null = null

const filteredProcesses = computed(() => {
  if (!searchText.value) return processes.value
  const search = searchText.value.toLowerCase()
  return processes.value.filter(p => 
    p.name.toLowerCase().includes(search) ||
    String(p.pid).includes(search)
  )
})

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

async function loadProcesses() {
  loading.value = true
  try {
    const result = await invoke<ProcessInfo[]>('get_processes')
    processes.value = result
  } catch (error) {
    ElMessage.error(`加载进程列表失败: ${error}`)
  } finally {
    loading.value = false
  }
}

async function endProcess(process: ProcessInfo) {
  if (appStore.settings.general.confirmDangerousActions) {
    await ElMessageBox.confirm(
      $t('processes.confirmEnd'),
      $t('common.warning'),
      {
        confirmButtonText: $t('common.confirm'),
        cancelButtonText: $t('common.cancel'),
        type: 'warning'
      }
    )
  }
  
  try {
    await invoke('end_process', { pid: process.pid })
    ElMessage.success(`进程 ${process.name} 已结束`)
    await loadProcesses()
  } catch (error) {
    ElMessage.error(`结束进程失败: ${error}`)
  }
}

onMounted(() => {
  loadProcesses()
  refreshInterval = window.setInterval(loadProcesses, 5000)
})

onUnmounted(() => {
  if (refreshInterval) {
    clearInterval(refreshInterval)
  }
})
</script>

<style scoped>
.processes {
  padding: 0;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-actions {
  display: flex;
  align-items: center;
}
</style>
