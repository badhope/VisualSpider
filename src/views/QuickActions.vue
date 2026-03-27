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
                <component :is="action.icon" />
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
  Tickets, DataBoard, Terminal, Grid, Tools
} from '@element-plus/icons-vue'
import type { QuickAction } from '@/types'

const quickActions = ref<QuickAction[]>([
  { id: '1', name: '控制面板', description: '打开Windows控制面板', icon: 'Setting', command: 'control', category: 'system', requiresAdmin: false },
  { id: '2', name: '设备管理器', description: '管理计算机硬件设备', icon: 'Monitor', command: 'devmgmt.msc', category: 'system', requiresAdmin: false },
  { id: '3', name: '任务管理器', description: '查看和管理运行中的程序', icon: 'Cpu', command: 'taskmgr', category: 'system', requiresAdmin: false },
  { id: '4', name: '注册表编辑器', description: '编辑Windows注册表', icon: 'Collection', command: 'regedit', category: 'system', requiresAdmin: true },
  { id: '5', name: '组策略', description: '配置系统和用户策略', icon: 'Document', command: 'gpedit.msc', category: 'system', requiresAdmin: true },
  { id: '6', name: '事件查看器', description: '查看系统和应用程序日志', icon: 'Tickets', command: 'eventvwr.msc', category: 'system', requiresAdmin: false },
  { id: '7', name: '系统信息', description: '查看详细的系统信息', icon: 'DataBoard', command: 'msinfo32', category: 'system', requiresAdmin: false },
  { id: '8', name: '命令提示符', description: '打开CMD命令行', icon: 'Terminal', command: 'cmd', category: 'system', requiresAdmin: false },
  { id: '9', name: 'PowerShell', description: '打开PowerShell', icon: 'Grid', command: 'powershell', category: 'system', requiresAdmin: false },
  { id: '10', name: '服务管理', description: '管理Windows服务', icon: 'Tools', command: 'services.msc', category: 'system', requiresAdmin: true },
  { id: '11', name: '计算机管理', description: '打开计算机管理控制台', icon: 'Monitor', command: 'compmgmt.msc', category: 'system', requiresAdmin: true },
  { id: '12', name: '磁盘管理', description: '管理磁盘分区', icon: 'Setting', command: 'diskmgmt.msc', category: 'system', requiresAdmin: true }
])

async function executeAction(action: QuickAction) {
  try {
    await invoke('open_system_tool', { command: action.command })
    ElMessage.success(`已打开 ${action.name}`)
  } catch (error) {
    ElMessage.error(`打开失败: ${error}`)
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
