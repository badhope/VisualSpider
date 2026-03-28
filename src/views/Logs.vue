<template>
  <div class="logs-viewer">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>{{ $t('logs.title') }}</span>
          <div class="header-actions">
            <el-select v-model="filterModule" :placeholder="$t('logs.filterModule')" clearable style="width: 120px;">
              <el-option :label="$t('logs.all')" value="" />
              <el-option :label="$t('logs.system')" value="system" />
              <el-option label="PowerShell" value="powershell" />
              <el-option :label="$t('logs.registry')" value="registry" />
              <el-option :label="$t('logs.process')" value="process" />
              <el-option :label="$t('logs.service')" value="service" />
              <el-option :label="$t('logs.network')" value="network" />
              <el-option :label="$t('logs.disk')" value="disk" />
            </el-select>
            
            <el-select v-model="filterStatus" :placeholder="$t('logs.filterStatus')" clearable style="width: 100px;">
              <el-option :label="$t('logs.all')" value="" />
              <el-option :label="$t('logs.success')" value="success" />
              <el-option :label="$t('logs.error')" value="error" />
              <el-option :label="$t('logs.warning')" value="warning" />
            </el-select>
            
            <el-button @click="refreshLogs">
              <el-icon><Refresh /></el-icon>
              {{ $t('common.refresh') }}
            </el-button>
            
            <el-button @click="exportLogs">
              <el-icon><Download /></el-icon>
              {{ $t('logs.export') }}
            </el-button>
            
            <el-button type="danger" @click="confirmClearLogs">
              <el-icon><Delete /></el-icon>
              {{ $t('logs.clear') }}
            </el-button>
          </div>
        </div>
      </template>
      
      <div class="stats-bar">
        <div class="stat-item">
          <span class="stat-label">{{ $t('logs.totalRecords') }}</span>
          <span class="stat-value">{{ logs.length }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">{{ $t('logs.success') }}</span>
          <span class="stat-value success">{{ successCount }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">{{ $t('logs.error') }}</span>
          <span class="stat-value error">{{ errorCount }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">{{ $t('logs.warning') }}</span>
          <span class="stat-value warning">{{ warningCount }}</span>
        </div>
      </div>
      
      <el-table :data="filteredLogs" border stripe max-height="500">
        <el-table-column prop="timestamp" :label="$t('logs.time')" width="180">
          <template #default="{ row }">
            {{ formatTime(row.timestamp) }}
          </template>
        </el-table-column>
        <el-table-column prop="module" :label="$t('logs.module')" width="100">
          <template #default="{ row }">
            <el-tag size="small">{{ getModuleLabel(row.module) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="action" :label="$t('logs.action')" width="120" />
        <el-table-column prop="status" :label="$t('services.status')" width="80">
          <template #default="{ row }">
            <el-tag size="small" :type="getStatusType(row.status)">
              {{ getStatusLabel(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="message" :label="$t('logs.message')" show-overflow-tooltip />
        <el-table-column :label="$t('logs.details')" width="80">
          <template #default="{ row }">
            <el-button 
              v-if="row.details" 
              text 
              size="small" 
              @click="showDetails(row)"
            >
              {{ $t('logs.view') }}
            </el-button>
            <span v-else class="no-details">-</span>
          </template>
        </el-table-column>
      </el-table>
      
      <div class="table-footer">
        <span>{{ $t('logs.showingCount', { shown: filteredLogs.length, total: logs.length }) }}</span>
      </div>
    </el-card>
    
    <el-dialog v-model="detailsDialogVisible" :title="$t('logs.logDetails')" width="600px">
      <div v-if="selectedLog" class="log-details">
        <el-descriptions :column="1" border>
          <el-descriptions-item :label="$t('logs.time')">{{ formatTime(selectedLog.timestamp) }}</el-descriptions-item>
          <el-descriptions-item :label="$t('logs.module')">{{ getModuleLabel(selectedLog.module) }}</el-descriptions-item>
          <el-descriptions-item :label="$t('logs.action')">{{ selectedLog.action }}</el-descriptions-item>
          <el-descriptions-item :label="$t('services.status')">
            <el-tag :type="getStatusType(selectedLog.status)">
              {{ getStatusLabel(selectedLog.status) }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item :label="$t('logs.message')">{{ selectedLog.message }}</el-descriptions-item>
          <el-descriptions-item :label="$t('logs.detailedInfo')">
            <pre class="details-content">{{ selectedLog.details }}</pre>
          </el-descriptions-item>
        </el-descriptions>
      </div>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAppStore, type OperationLog } from '@/stores/appStore'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Refresh, Download, Delete } from '@element-plus/icons-vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const appStore = useAppStore()
const filterModule = ref('')
const filterStatus = ref('')
const detailsDialogVisible = ref(false)
const selectedLog = ref<OperationLog | null>(null)

const logs = computed(() => appStore.operationLogs)

const filteredLogs = computed(() => {
  let result = logs.value
  
  if (filterModule.value) {
    result = result.filter(log => log.module === filterModule.value)
  }
  
  if (filterStatus.value) {
    result = result.filter(log => log.status === filterStatus.value)
  }
  
  return result.slice().reverse()
})

const successCount = computed(() => logs.value.filter(l => l.status === 'success').length)
const errorCount = computed(() => logs.value.filter(l => l.status === 'error').length)
const warningCount = computed(() => logs.value.filter(l => l.status === 'warning').length)

function formatTime(timestamp: string): string {
  return new Date(timestamp).toLocaleString('zh-CN')
}

function getModuleLabel(module: string): string {
  const labels: Record<string, string> = {
    system: t('logs.system'),
    powershell: 'PowerShell',
    registry: t('logs.registry'),
    process: t('logs.process'),
    service: t('logs.service'),
    network: t('logs.network'),
    disk: t('logs.disk')
  }
  return labels[module] || module
}

function getStatusType(status: string): string {
  const types: Record<string, string> = {
    success: 'success',
    error: 'danger',
    warning: 'warning'
  }
  return types[status] || ''
}

function getStatusLabel(status: string): string {
  const labels: Record<string, string> = {
    success: t('logs.success'),
    error: t('logs.error'),
    warning: t('logs.warning')
  }
  return labels[status] || status
}

function refreshLogs() {
  ElMessage.success(t('logs.refreshSuccess'))
}

function showDetails(log: OperationLog) {
  selectedLog.value = log
  detailsDialogVisible.value = true
}

function exportLogs() {
  const data = filteredLogs.value.map(log => ({
    [t('logs.time')]: formatTime(log.timestamp),
    [t('logs.module')]: getModuleLabel(log.module),
    [t('logs.action')]: log.action,
    [t('services.status')]: getStatusLabel(log.status),
    [t('logs.message')]: log.message,
    [t('logs.details')]: log.details || ''
  }))
  
  const csv = [
    Object.keys(data[0]).join(','),
    ...data.map(row => Object.values(row).map(v => `"${v}"`).join(','))
  ].join('\n')
  
  const blob = new Blob(['\ufeff' + csv], { type: 'text/csv;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `operation-logs-${new Date().toISOString().split('T')[0]}.csv`
  a.click()
  URL.revokeObjectURL(url)
  
  ElMessage.success(t('logs.exportSuccess'))
}

async function confirmClearLogs() {
  try {
    await ElMessageBox.confirm(
      t('logs.confirmClear'),
      t('logs.confirmClearTitle'),
      {
        type: 'warning',
        confirmButtonText: t('logs.clear'),
        cancelButtonText: t('common.cancel')
      }
    )
    
    appStore.clearLogs()
    ElMessage.success(t('logs.clearSuccess'))
  } catch {
    // cancelled
  }
}
</script>

<style scoped>
.logs-viewer {
  padding: 0;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.stats-bar {
  display: flex;
  align-items: center;
  gap: 24px;
  padding: 12px 16px;
  background: #f5f7fa;
  border-radius: 4px;
  margin-bottom: 16px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.stat-label {
  color: #909399;
  font-size: 13px;
}

.stat-value {
  font-weight: 600;
  font-size: 14px;
}

.stat-value.success {
  color: #67C23A;
}

.stat-value.error {
  color: #F56C6C;
}

.stat-value.warning {
  color: #E6A23C;
}

.table-footer {
  padding: 12px 0;
  text-align: right;
  color: #909399;
  font-size: 13px;
}

.no-details {
  color: #c0c4cc;
}

.log-details {
  padding: 16px 0;
}

.details-content {
  background: #f5f7fa;
  padding: 12px;
  border-radius: 4px;
  font-size: 12px;
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 300px;
  overflow-y: auto;
}
</style>
