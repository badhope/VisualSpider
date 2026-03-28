<template>
  <div class="quick-actions">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>{{ $t('quickActions.title') }}</span>
        </div>
      </template>
      
      <el-row :gutter="20">
        <el-col :span="6" v-for="action in quickActions" :key="action.id">
          <el-card class="action-card" shadow="hover" @click="executeAction(action)">
            <div class="action-icon">
              <el-icon size="32">
                <component :is="iconMap[action.icon]" />
              </el-icon>
            </div>
            <div class="action-name">{{ action.name }}</div>
            <div class="action-desc">{{ action.description }}</div>
            <el-tag v-if="action.requiresAdmin" type="warning" size="small">
              {{ $t('common.requiresAdmin') }}
            </el-tag>
          </el-card>
        </el-col>
      </el-row>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import { 
  Setting, Monitor, Cpu, Collection, Document, 
  Tickets, DataBoard, Grid, Tools
} from '@element-plus/icons-vue'
import { useI18n } from 'vue-i18n'
import type { QuickAction } from '@/types'

const { t } = useI18n()

const iconMap: Record<string, any> = {
  Setting, Monitor, Cpu, Collection, Document, 
  Tickets, DataBoard, Grid, Tools
}

const quickActions = ref<QuickAction[]>([
  { id: '1', name: t('quickActions.controlPanel'), description: t('quickActions.controlPanelDesc'), icon: 'Setting', command: 'control', category: 'system', requiresAdmin: false },
  { id: '2', name: t('quickActions.deviceManager'), description: t('quickActions.deviceManagerDesc'), icon: 'Monitor', command: 'devmgmt.msc', category: 'system', requiresAdmin: false },
  { id: '3', name: t('quickActions.taskManager'), description: t('quickActions.taskManagerDesc'), icon: 'Cpu', command: 'taskmgr', category: 'system', requiresAdmin: false },
  { id: '4', name: t('quickActions.registryEditor'), description: t('quickActions.registryEditorDesc'), icon: 'Collection', command: 'regedit', category: 'system', requiresAdmin: true },
  { id: '5', name: t('quickActions.groupPolicy'), description: t('quickActions.groupPolicyDesc'), icon: 'Document', command: 'gpedit.msc', category: 'system', requiresAdmin: true },
  { id: '6', name: t('quickActions.eventViewer'), description: t('quickActions.eventViewerDesc'), icon: 'Tickets', command: 'eventvwr.msc', category: 'system', requiresAdmin: false },
  { id: '7', name: t('quickActions.systemInfo'), description: t('quickActions.systemInfoDesc'), icon: 'DataBoard', command: 'msinfo32', category: 'system', requiresAdmin: false },
  { id: '8', name: t('quickActions.cmd'), description: t('quickActions.cmdDesc'), icon: 'Grid', command: 'cmd', category: 'system', requiresAdmin: false },
  { id: '9', name: t('quickActions.powershell'), description: t('quickActions.powershellDesc'), icon: 'Grid', command: 'powershell', category: 'system', requiresAdmin: false },
  { id: '10', name: t('quickActions.serviceManager'), description: t('quickActions.serviceManagerDesc'), icon: 'Tools', command: 'services.msc', category: 'system', requiresAdmin: true },
  { id: '11', name: t('quickActions.computerManagement'), description: t('quickActions.computerManagementDesc'), icon: 'Monitor', command: 'compmgmt.msc', category: 'system', requiresAdmin: true },
  { id: '12', name: t('quickActions.diskManagement'), description: t('quickActions.diskManagementDesc'), icon: 'Setting', command: 'diskmgmt.msc', category: 'system', requiresAdmin: true }
])

async function executeAction(action: QuickAction) {
  try {
    await invoke('open_system_tool', { command: action.command })
    ElMessage.success(t('quickActions.openSuccess', { name: action.name }))
  } catch (error) {
    ElMessage.error(t('quickActions.openFailed') + `: ${error}`)
  }
}
</script>

<style scoped>
.quick-actions {
  padding: 0;
}

.card-header {
  display: flex;
  align-items: center;
}

.action-card {
  cursor: pointer;
  text-align: center;
  padding: 20px;
  margin-bottom: 20px;
  transition: transform 0.2s;
}

.action-card:hover {
  transform: translateY(-4px);
}

.action-icon {
  margin-bottom: 12px;
  color: #409eff;
}

.action-name {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 8px;
  color: #303133;
}

.action-desc {
  font-size: 12px;
  color: #909399;
  margin-bottom: 8px;
}
</style>
