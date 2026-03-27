<template>
  <div class="powershell">
    <el-row :gutter="20">
      <el-col :span="6">
        <el-card class="commands-card">
          <template #header>
            <div class="card-header">
              <span>{{ $t('powershell.commonCommands') }}</span>
            </div>
          </template>
          <el-collapse v-model="activeCollapse">
            <el-collapse-item :title="$t('powershell.systemInfo')" name="system">
              <div class="command-list">
                <el-button 
                  v-for="cmd in systemCommands" 
                  :key="cmd.name"
                  text 
                  @click="insertCommand(cmd.command)"
                >
                  {{ cmd.name }}
                </el-button>
              </div>
            </el-collapse-item>
            <el-collapse-item :title="$t('powershell.networkInfo')" name="network">
              <div class="command-list">
                <el-button 
                  v-for="cmd in networkCommands" 
                  :key="cmd.name"
                  text 
                  @click="insertCommand(cmd.command)"
                >
                  {{ cmd.name }}
                </el-button>
              </div>
            </el-collapse-item>
            <el-collapse-item :title="$t('powershell.processInfo')" name="process">
              <div class="command-list">
                <el-button 
                  v-for="cmd in processCommands" 
                  :key="cmd.name"
                  text 
                  @click="insertCommand(cmd.command)"
                >
                  {{ cmd.name }}
                </el-button>
              </div>
            </el-collapse-item>
            <el-collapse-item :title="$t('powershell.diskInfo')" name="disk">
              <div class="command-list">
                <el-button 
                  v-for="cmd in diskCommands" 
                  :key="cmd.name"
                  text 
                  @click="insertCommand(cmd.command)"
                >
                  {{ cmd.name }}
                </el-button>
              </div>
            </el-collapse-item>
            <el-collapse-item :title="$t('powershell.serviceInfo')" name="service">
              <div class="command-list">
                <el-button 
                  v-for="cmd in serviceCommands" 
                  :key="cmd.name"
                  text 
                  @click="insertCommand(cmd.command)"
                >
                  {{ cmd.name }}
                </el-button>
              </div>
            </el-collapse-item>
          </el-collapse>
        </el-card>
      </el-col>
      
      <el-col :span="18">
        <el-card class="terminal-card">
          <template #header>
            <div class="card-header">
              <span>{{ $t('powershell.commandInput') }}</span>
              <div class="header-actions">
                <el-button type="primary" @click="executeCommand" :loading="executing">
                  <el-icon><VideoPlay /></el-icon>
                  {{ $t('powershell.run') }}
                </el-button>
                <el-button @click="clearOutput">
                  <el-icon><Delete /></el-icon>
                  {{ $t('powershell.clear') }}
                </el-button>
              </div>
            </div>
          </template>
          
          <div class="command-input">
            <el-input
              v-model="command"
              type="textarea"
              :rows="3"
              placeholder="输入 PowerShell 命令..."
              @keydown.ctrl.enter="executeCommand"
            />
          </div>
          
          <div class="output-area">
            <div class="output-header">
              <span>{{ $t('powershell.output') }}</span>
            </div>
            <pre class="output-content" ref="outputRef">{{ output || '等待执行命令...' }}</pre>
          </div>
        </el-card>
        
        <el-card class="history-card" style="margin-top: 20px;">
          <template #header>
            <div class="card-header">
              <span>{{ $t('powershell.history') }}</span>
              <el-button text @click="clearHistory">清空历史</el-button>
            </div>
          </template>
          <el-table :data="history" max-height="200" size="small">
            <el-table-column prop="command" label="命令" show-overflow-tooltip />
            <el-table-column prop="time" label="时间" width="180" />
            <el-table-column label="操作" width="100">
              <template #default="{ row }">
                <el-button text size="small" @click="insertCommand(row.command)">重用</el-button>
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
import { VideoPlay, Delete } from '@element-plus/icons-vue'
import { useAppStore } from '@/stores'
import type { CommandResult } from '@/types'

const appStore = useAppStore()
const command = ref('')
const output = ref('')
const executing = ref(false)
const activeCollapse = ref(['system'])
const outputRef = ref<HTMLElement | null>(null)
const history = ref<Array<{ command: string; time: string }>>([])

const systemCommands = [
  { name: '系统信息', command: 'Get-ComputerInfo' },
  { name: '操作系统版本', command: 'Get-CimInstance Win32_OperatingSystem | Select-Object Caption, Version' },
  { name: 'CPU信息', command: 'Get-CimInstance Win32_Processor | Select-Object Name, NumberOfCores' },
  { name: '内存信息', command: 'Get-CimInstance Win32_PhysicalMemory | Measure-Object -Property Capacity -Sum' },
  { name: '磁盘信息', command: 'Get-CimInstance Win32_LogicalDisk | Select-Object DeviceID, Size, FreeSpace' },
  { name: '系统启动时间', command: 'Get-CimInstance Win32_OperatingSystem | Select-Object LastBootUpTime' }
]

const networkCommands = [
  { name: '网络适配器', command: 'Get-NetAdapter' },
  { name: 'IP配置', command: 'Get-NetIPConfiguration' },
  { name: 'DNS缓存', command: 'Get-DnsClientCache' },
  { name: '网络连接', command: 'Get-NetTCPConnection' },
  { name: '测试网络连接', command: 'Test-NetConnection google.com' },
  { name: '路由表', command: 'Get-NetRoute' }
]

const processCommands = [
  { name: '所有进程', command: 'Get-Process | Select-Object Id, Name, CPU, WorkingSet' },
  { name: 'CPU占用前10', command: 'Get-Process | Sort-Object CPU -Descending | Select-Object -First 10' },
  { name: '内存占用前10', command: 'Get-Process | Sort-Object WorkingSet -Descending | Select-Object -First 10' },
  { name: '进程详情', command: 'Get-Process | Select-Object Id, Name, Path, StartTime' }
]

const diskCommands = [
  { name: '磁盘分区', command: 'Get-Partition' },
  { name: '物理磁盘', command: 'Get-PhysicalDisk' },
  { name: '卷信息', command: 'Get-Volume' },
  { name: '磁盘使用率', command: 'Get-CimInstance Win32_LogicalDisk | Select-Object DeviceID, @{N="Used(GB)";E={[math]::Round(($_.Size - $_.FreeSpace)/1GB,2)}}, @{N="Free(GB)";E={[math]::Round($_.FreeSpace/1GB,2)}}' }
]

const serviceCommands = [
  { name: '所有服务', command: 'Get-Service | Select-Object Name, Status, StartType' },
  { name: '运行中的服务', command: 'Get-Service | Where-Object {$_.Status -eq "Running"}' },
  { name: '已停止的服务', command: 'Get-Service | Where-Object {$_.Status -eq "Stopped"}' }
]

function insertCommand(cmd: string) {
  command.value = cmd
}

async function executeCommand() {
  if (!command.value.trim()) return
  
  executing.value = true
  output.value = `${$t('common.executing')}\n`
  
  try {
    const result = await invoke<CommandResult>('execute_powershell', {
      command: command.value
    })
    
    if (result.success) {
      output.value = result.output || '命令执行成功，无输出。'
    } else {
      output.value = `错误: ${result.error}`
    }
    
    if (appStore.settings.powershell.saveHistory) {
      history.value.unshift({
        command: command.value,
        time: new Date().toLocaleString('zh-CN')
      })
      
      if (history.value.length > appStore.settings.powershell.maxHistoryItems) {
        history.value = history.value.slice(0, appStore.settings.powershell.maxHistoryItems)
      }
      saveHistory()
    }
    
    appStore.addNotification('success', '命令执行完成', command.value.substring(0, 50))
  } catch (error) {
    output.value = `执行失败: ${error}`
    appStore.addNotification('error', '命令执行失败', String(error))
  } finally {
    executing.value = false
  }
}

function clearOutput() {
  output.value = ''
}

function clearHistory() {
  history.value = []
  localStorage.removeItem('powershell-history')
}

function saveHistory() {
  localStorage.setItem('powershell-history', JSON.stringify(history.value))
}

function loadHistory() {
  const saved = localStorage.getItem('powershell-history')
  if (saved) {
    history.value = JSON.parse(saved)
  }
}

onMounted(() => {
  loadHistory()
})
</script>

<style scoped>
.powershell {
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

.command-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.command-list .el-button {
  justify-content: flex-start;
  text-align: left;
}

.command-input {
  margin-bottom: 16px;
}

.output-area {
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  overflow: hidden;
}

.output-header {
  background: #f5f7fa;
  padding: 8px 12px;
  border-bottom: 1px solid #dcdfe6;
  font-weight: 600;
  font-size: 13px;
}

.output-content {
  background: #1e1e1e;
  color: #d4d4d4;
  padding: 16px;
  margin: 0;
  min-height: 300px;
  max-height: 400px;
  overflow: auto;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 13px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-all;
}

.commands-card {
  height: calc(100vh - 160px);
  overflow-y: auto;
}
</style>
