<template>
  <div class="optimization">
    <el-row :gutter="20">
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>{{ $t('optimization.startupItems') }}</span>
              <el-button @click="loadStartupItems">
                <el-icon><Refresh /></el-icon>
              </el-button>
            </div>
          </template>
          <el-table :data="startupItems" v-loading="loadingStartup" size="small">
            <el-table-column prop="name" label="名称" show-overflow-tooltip />
            <el-table-column prop="command" label="命令" show-overflow-tooltip />
            <el-table-column prop="location" label="位置" show-overflow-tooltip />
            <el-table-column label="操作" width="100">
              <template #default="{ row }">
                <el-switch v-model="row.enabled" @change="toggleStartupItem(row)" />
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
      
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>{{ $t('optimization.performance') }}</span>
            </div>
          </template>
          <div class="optimization-actions">
            <el-button type="primary" @click="cleanTemp">
              <el-icon><Delete /></el-icon>
              {{ $t('optimization.cleanTemp') }}
            </el-button>
            <el-button type="success" @click="cleanCache">
              <el-icon><Brush /></el-icon>
              {{ $t('optimization.cleanCache') }}
            </el-button>
            <el-button type="warning" @click="optimizePerformance">
              <el-icon><TrendCharts /></el-icon>
              性能优化
            </el-button>
          </div>
          
          <el-divider />
          
          <div class="performance-tips">
            <h4>优化建议</h4>
            <el-alert
              v-for="(tip, index) in performanceTips"
              :key="index"
              :title="tip"
              type="info"
              :closable="false"
              style="margin-bottom: 8px;"
            />
          </div>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="20" style="margin-top: 20px;">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>{{ $t('optimization.scheduledTasks') }}</span>
              <el-button @click="loadScheduledTasks">
                <el-icon><Refresh /></el-icon>
              </el-button>
            </div>
          </template>
          <el-table :data="scheduledTasks" v-loading="loadingTasks" size="small">
            <el-table-column prop="name" label="任务名称" show-overflow-tooltip />
            <el-table-column prop="status" label="状态" width="100">
              <template #default="{ row }">
                <el-tag :type="row.status === 'Ready' ? 'success' : 'info'">{{ row.status }}</el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="lastRun" label="上次运行" width="180" />
            <el-table-column prop="nextRun" label="下次运行" width="180" />
            <el-table-column label="操作" width="150">
              <template #default="{ row }">
                <el-button text size="small" @click="runTask(row.name)">运行</el-button>
                <el-button text size="small" @click="disableTask(row.name)">禁用</el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import { Refresh, Delete, Brush, TrendCharts } from '@element-plus/icons-vue'

const loadingStartup = ref(false)
const loadingTasks = ref(false)
const startupItems = ref<Array<{ name: string; command: string; location: string; enabled: boolean }>>([])
const scheduledTasks = ref<Array<{ name: string; status: string; lastRun: string; nextRun: string }>>([])
const performanceTips = ref([
  '禁用不必要的启动项可以加快系统启动速度',
  '定期清理临时文件可以释放磁盘空间',
  '关闭不必要的后台服务可以节省系统资源'
])

async function loadStartupItems() {
  loadingStartup.value = true
  try {
    const result = await invoke('get_startup_items')
    startupItems.value = result
  } catch (error) {
    ElMessage.error(`加载启动项失败: ${error}`)
  } finally {
    loadingStartup.value = false
  }
}

async function toggleStartupItem(item: any) {
  try {
    await invoke('toggle_startup_item', { name: item.name, enabled: item.enabled })
    ElMessage.success(item.enabled ? '已启用' : '已禁用')
  } catch (error) {
    ElMessage.error(`操作失败: ${error}`)
    item.enabled = !item.enabled
  }
}

async function loadScheduledTasks() {
  loadingTasks.value = true
  try {
    const result = await invoke('get_scheduled_tasks')
    scheduledTasks.value = result
  } catch (error) {
    ElMessage.error(`加载计划任务失败: ${error}`)
  } finally {
    loadingTasks.value = false
  }
}

async function runTask(name: string) {
  try {
    await invoke('run_scheduled_task', { name })
    ElMessage.success(`任务 ${name} 已启动`)
  } catch (error) {
    ElMessage.error(`运行任务失败: ${error}`)
  }
}

async function disableTask(name: string) {
  try {
    await invoke('disable_scheduled_task', { name })
    ElMessage.success(`任务 ${name} 已禁用`)
    await loadScheduledTasks()
  } catch (error) {
    ElMessage.error(`禁用任务失败: ${error}`)
  }
}

async function cleanTemp() {
  try {
    const result = await invoke<{ deleted: number; freed: number }>('clean_temp_files')
    ElMessage.success(`已清理 ${result.deleted} 个文件，释放 ${formatBytes(result.freed)} 空间`)
  } catch (error) {
    ElMessage.error(`清理失败: ${error}`)
  }
}

async function cleanCache() {
  try {
    const result = await invoke<{ deleted: number; freed: number }>('clean_cache_files')
    ElMessage.success(`已清理 ${result.deleted} 个文件，释放 ${formatBytes(result.freed)} 空间`)
  } catch (error) {
    ElMessage.error(`清理失败: ${error}`)
  }
}

async function optimizePerformance() {
  try {
    await invoke('optimize_performance')
    ElMessage.success('性能优化完成')
  } catch (error) {
    ElMessage.error(`优化失败: ${error}`)
  }
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

onMounted(() => {
  loadStartupItems()
  loadScheduledTasks()
})
</script>

<style scoped>
.optimization {
  padding: 0;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.optimization-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.performance-tips h4 {
  margin-bottom: 12px;
  color: #303133;
}
</style>
