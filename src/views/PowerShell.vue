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
          
          <el-input
            v-model="searchCommand"
            :placeholder="$t('powershell.searchCommand')"
            clearable
            style="margin-bottom: 12px;"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
          
          <el-tabs v-model="activeTab" size="small">
            <el-tab-pane :label="$t('powershell.commandLibrary')" name="library">
              <el-collapse v-model="activeCollapse">
                <el-collapse-item :title="$t('powershell.systemInfo')" name="system">
                  <div class="command-list">
                    <div 
                      v-for="cmd in filteredSystemCommands" 
                      :key="cmd.name"
                      class="command-item"
                      @click="insertCommand(cmd.command)"
                    >
                      <span class="cmd-name">{{ cmd.name }}</span>
                      <el-button 
                        text 
                        size="small" 
                        @click.stop="toggleFavorite(cmd)"
                        :type="isFavorite(cmd) ? 'warning' : 'default'"
                      >
                        <el-icon><Star /></el-icon>
                      </el-button>
                    </div>
                  </div>
                </el-collapse-item>
                <el-collapse-item :title="$t('powershell.networkDiagnostics')" name="network">
                  <div class="command-list">
                    <div 
                      v-for="cmd in filteredNetworkCommands" 
                      :key="cmd.name"
                      class="command-item"
                      @click="insertCommand(cmd.command)"
                    >
                      <span class="cmd-name">{{ cmd.name }}</span>
                      <el-button 
                        text 
                        size="small" 
                        @click.stop="toggleFavorite(cmd)"
                        :type="isFavorite(cmd) ? 'warning' : 'default'"
                      >
                        <el-icon><Star /></el-icon>
                      </el-button>
                    </div>
                  </div>
                </el-collapse-item>
                <el-collapse-item :title="$t('powershell.processManagement')" name="process">
                  <div class="command-list">
                    <div 
                      v-for="cmd in filteredProcessCommands" 
                      :key="cmd.name"
                      class="command-item"
                      @click="insertCommand(cmd.command)"
                    >
                      <span class="cmd-name">{{ cmd.name }}</span>
                      <el-button 
                        text 
                        size="small" 
                        @click.stop="toggleFavorite(cmd)"
                        :type="isFavorite(cmd) ? 'warning' : 'default'"
                      >
                        <el-icon><Star /></el-icon>
                      </el-button>
                    </div>
                  </div>
                </el-collapse-item>
                <el-collapse-item :title="$t('powershell.diskManagement')" name="disk">
                  <div class="command-list">
                    <div 
                      v-for="cmd in filteredDiskCommands" 
                      :key="cmd.name"
                      class="command-item"
                      @click="insertCommand(cmd.command)"
                    >
                      <span class="cmd-name">{{ cmd.name }}</span>
                      <el-button 
                        text 
                        size="small" 
                        @click.stop="toggleFavorite(cmd)"
                        :type="isFavorite(cmd) ? 'warning' : 'default'"
                      >
                        <el-icon><Star /></el-icon>
                      </el-button>
                    </div>
                  </div>
                </el-collapse-item>
                <el-collapse-item :title="$t('powershell.serviceManagement')" name="service">
                  <div class="command-list">
                    <div 
                      v-for="cmd in filteredServiceCommands" 
                      :key="cmd.name"
                      class="command-item"
                      @click="insertCommand(cmd.command)"
                    >
                      <span class="cmd-name">{{ cmd.name }}</span>
                      <el-button 
                        text 
                        size="small" 
                        @click.stop="toggleFavorite(cmd)"
                        :type="isFavorite(cmd) ? 'warning' : 'default'"
                      >
                        <el-icon><Star /></el-icon>
                      </el-button>
                    </div>
                  </div>
                </el-collapse-item>
                <el-collapse-item :title="$t('powershell.fileOperations')" name="file">
                  <div class="command-list">
                    <div 
                      v-for="cmd in filteredFileCommands" 
                      :key="cmd.name"
                      class="command-item"
                      @click="insertCommand(cmd.command)"
                    >
                      <span class="cmd-name">{{ cmd.name }}</span>
                      <el-button 
                        text 
                        size="small" 
                        @click.stop="toggleFavorite(cmd)"
                        :type="isFavorite(cmd) ? 'warning' : 'default'"
                      >
                        <el-icon><Star /></el-icon>
                      </el-button>
                    </div>
                  </div>
                </el-collapse-item>
                <el-collapse-item :title="$t('powershell.security')" name="security">
                  <div class="command-list">
                    <div 
                      v-for="cmd in filteredSecurityCommands" 
                      :key="cmd.name"
                      class="command-item"
                      @click="insertCommand(cmd.command)"
                    >
                      <span class="cmd-name">{{ cmd.name }}</span>
                      <el-button 
                        text 
                        size="small" 
                        @click.stop="toggleFavorite(cmd)"
                        :type="isFavorite(cmd) ? 'warning' : 'default'"
                      >
                        <el-icon><Star /></el-icon>
                      </el-button>
                    </div>
                  </div>
                </el-collapse-item>
              </el-collapse>
            </el-tab-pane>
            
            <el-tab-pane :label="$t('powershell.favorites')" name="favorites">
              <div v-if="favorites.length === 0" class="empty-favorites">
                <el-icon size="32"><Star /></el-icon>
                <p>{{ $t('powershell.noFavorites') }}</p>
                <p class="tip">{{ $t('powershell.clickStarToAdd') }}</p>
              </div>
              <div v-else class="command-list">
                <div 
                  v-for="cmd in favorites" 
                  :key="cmd.command"
                  class="command-item"
                  @click="insertCommand(cmd.command)"
                >
                  <span class="cmd-name">{{ cmd.name }}</span>
                  <el-button 
                    text 
                    size="small" 
                    type="danger"
                    @click.stop="removeFavorite(cmd)"
                  >
                    <el-icon><Delete /></el-icon>
                  </el-button>
                </div>
              </div>
            </el-tab-pane>
          </el-tabs>
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
                  {{ $t('powershell.run') }} (Ctrl+Enter)
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
              :rows="4"
              :placeholder="$t('powershell.inputPlaceholder')"
              @keydown.ctrl.enter="executeCommand"
            />
          </div>
          
          <div class="output-area">
            <div class="output-header">
              <span>{{ $t('powershell.output') }}</span>
              <div class="output-actions">
                <el-button text size="small" @click="copyOutput" :disabled="!output">
                  <el-icon><CopyDocument /></el-icon>
                  {{ $t('common.copy') }}
                </el-button>
                <el-button text size="small" @click="exportOutput" :disabled="!output">
                  <el-icon><Download /></el-icon>
                  {{ $t('common.export') }}
                </el-button>
              </div>
            </div>
            <pre class="output-content" ref="outputRef">{{ output || $t('powershell.enterCommand') }}</pre>
          </div>
        </el-card>
        
        <el-card class="history-card" style="margin-top: 20px;">
          <template #header>
            <div class="card-header">
              <span>{{ $t('powershell.history') }} ({{ history.length }})</span>
              <el-button text type="danger" @click="clearHistory">
                <el-icon><Delete /></el-icon>
                {{ $t('powershell.clearHistory') }}
              </el-button>
            </div>
          </template>
          <el-table 
            :data="history" 
            max-height="200" 
            size="small"
            :empty-text="$t('common.noData')"
          >
            <el-table-column prop="command" :label="$t('powershell.commandInput')" show-overflow-tooltip>
              <template #default="{ row }">
                <code class="cmd-code">{{ row.command }}</code>
              </template>
            </el-table-column>
            <el-table-column prop="time" :label="$t('logs.time')" width="180" />
            <el-table-column :label="$t('services.status')" width="80">
              <template #default="{ row }">
                <el-tag :type="row.success ? 'success' : 'danger'" size="small">
                  {{ row.success ? $t('logs.success') : $t('logs.error') }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column :label="$t('common.edit')" width="120">
              <template #default="{ row }">
                <el-button text size="small" @click="insertCommand(row.command)">{{ $t('common.copy') }}</el-button>
                <el-button text size="small" type="warning" @click="addToFavoritesFromHistory(row)">
                  <el-icon><Star /></el-icon>
                </el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
    </el-row>
    
    <el-dialog v-model="showSaveDialog" :title="$t('powershell.addToFavorites')" width="400px">
      <el-form>
        <el-form-item :label="$t('registry.name')">
          <el-input v-model="newFavoriteName" :placeholder="$t('registry.valueName')" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showSaveDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="saveToFavorite">{{ $t('common.save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { VideoPlay, Delete, Star, Search, CopyDocument, Download } from '@element-plus/icons-vue'
import { useAppStore } from '@/stores'
import { ElMessage } from 'element-plus'
import type { CommandResult } from '@/types'

interface CommandItem {
  name: string
  command: string
  category?: string
}

const appStore = useAppStore()
const command = ref('')
const output = ref('')
const executing = ref(false)
const activeCollapse = ref(['system'])
const activeTab = ref('library')
const outputRef = ref<HTMLElement | null>(null)
const searchCommand = ref('')
const showSaveDialog = ref(false)
const newFavoriteName = ref('')
const pendingCommand = ref<CommandItem | null>(null)

const history = ref<Array<{ command: string; time: string; success: boolean }>>([])
const favorites = ref<CommandItem[]>([])

const systemCommands: CommandItem[] = [
  { name: '系统完整信息', command: 'Get-ComputerInfo', category: 'system' },
  { name: '操作系统版本', command: 'Get-CimInstance Win32_OperatingSystem | Select-Object Caption, Version, BuildNumber', category: 'system' },
  { name: 'CPU信息', command: 'Get-CimInstance Win32_Processor | Select-Object Name, NumberOfCores, NumberOfLogicalProcessors, MaxClockSpeed', category: 'system' },
  { name: '内存信息', command: 'Get-CimInstance Win32_PhysicalMemory | Select-Object Manufacturer, Capacity, Speed, PartNumber', category: 'system' },
  { name: '总内存大小', command: '$total = (Get-CimInstance Win32_PhysicalMemory | Measure-Object -Property Capacity -Sum).Sum; Write-Host "总内存: $([math]::Round($total/1GB, 2)) GB"', category: 'system' },
  { name: '磁盘信息', command: 'Get-CimInstance Win32_LogicalDisk | Select-Object DeviceID, VolumeName, @{N="Size(GB)";E={[math]::Round($_.Size/1GB,2)}}, @{N="Free(GB)";E={[math]::Round($_.FreeSpace/1GB,2)}}', category: 'system' },
  { name: '系统启动时间', command: 'Get-CimInstance Win32_OperatingSystem | Select-Object LastBootUpTime, @{N="运行时长";E={(Get-Date) - $_.ConvertToDateTime($_.LastBootUpTime)}}', category: 'system' },
  { name: '环境变量', command: 'Get-ChildItem Env: | Format-Table -AutoSize', category: 'system' },
  { name: '系统服务', command: 'Get-Service | Group-Object Status | Select-Object Name, Count', category: 'system' },
  { name: '用户列表', command: 'Get-LocalUser | Select-Object Name, Enabled, LastLogon', category: 'system' }
]

const networkCommands: CommandItem[] = [
  { name: '网络适配器', command: 'Get-NetAdapter | Select-Object Name, InterfaceDescription, Status, LinkSpeed, MacAddress', category: 'network' },
  { name: 'IP配置', command: 'Get-NetIPConfiguration | Select-Object InterfaceAlias, IPv4Address, IPv6Address', category: 'network' },
  { name: 'DNS服务器', command: 'Get-DnsClientServerAddress -AddressFamily IPv4 | Select-Object InterfaceAlias, ServerAddresses', category: 'network' },
  { name: '网络连接', command: 'Get-NetTCPConnection | Select-Object LocalAddress, LocalPort, RemoteAddress, RemotePort, State, OwningProcess | Format-Table -AutoSize', category: 'network' },
  { name: '监听端口', command: 'Get-NetTCPConnection -State Listen | Select-Object LocalAddress, LocalPort, OwningProcess | Sort-Object LocalPort', category: 'network' },
  { name: '测试网络', command: 'Test-NetConnection -ComputerName www.baidu.com -Port 443', category: 'network' },
  { name: '路由表', command: 'Get-NetRoute | Select-Object DestinationPrefix, NextHop, RouteMetric, ifIndex', category: 'network' },
  { name: 'ARP缓存', command: 'Get-NetNeighbor | Select-Object IPAddress, LinkLayerAddress, InterfaceAlias, State', category: 'network' },
  { name: '刷新DNS', command: 'Clear-DnsClientCache; Write-Host "DNS缓存已清除"', category: 'network' },
  { name: '网络统计', command: 'Get-NetAdapterStatistics | Select-Object Name, ReceivedBytes, SentBytes', category: 'network' }
]

const processCommands: CommandItem[] = [
  { name: '所有进程', command: 'Get-Process | Select-Object Id, Name, CPU, @{N="Memory(MB)";E={[math]::Round($_.WorkingSet64/1MB,2)}} | Format-Table -AutoSize', category: 'process' },
  { name: 'CPU占用前10', command: 'Get-Process | Sort-Object CPU -Descending | Select-Object -First 10 Id, Name, CPU, @{N="Memory(MB)";E={[math]::Round($_.WorkingSet64/1MB,2)}}', category: 'process' },
  { name: '内存占用前10', command: 'Get-Process | Sort-Object WorkingSet64 -Descending | Select-Object -First 10 Id, Name, @{N="Memory(MB)";E={[math]::Round($_.WorkingSet64/1MB,2)}}, CPU', category: 'process' },
  { name: '进程详情', command: 'Get-Process | Select-Object Id, Name, Path, StartTime, CPU, @{N="Memory(MB)";E={[math]::Round($_.WorkingSet64/1MB,2)}}', category: 'process' },
  { name: '查找进程', command: '$name = Read-Host "输入进程名"; Get-Process -Name $name -ErrorAction SilentlyContinue | Select-Object Id, Name, Path', category: 'process' },
  { name: '进程线程数', command: 'Get-Process | Select-Object Name, Id, Threads | Sort-Object {$_.Threads.Count} -Descending | Select-Object -First 20 Name, Id, @{N="Threads";E={$_.Threads.Count}}', category: 'process' },
  { name: '结束进程(需输入)', command: '$pid = Read-Host "输入进程ID"; Stop-Process -Id $pid -Force; Write-Host "进程 $pid 已结束"', category: 'process' }
]

const diskCommands: CommandItem[] = [
  { name: '磁盘分区', command: 'Get-Partition | Select-Object DiskNumber, PartitionNumber, DriveLetter, Size, Type', category: 'disk' },
  { name: '物理磁盘', command: 'Get-PhysicalDisk | Select-Object DeviceId, FriendlyName, MediaType, Size, HealthStatus', category: 'disk' },
  { name: '卷信息', command: 'Get-Volume | Select-Object DriveLetter, FileSystemLabel, FileSystem, @{N="Size(GB)";E={[math]::Round($_.Size/1GB,2)}}, @{N="Free(GB)";E={[math]::Round($_.SizeRemaining/1GB,2)}}', category: 'disk' },
  { name: '磁盘使用率', command: 'Get-CimInstance Win32_LogicalDisk | Select-Object DeviceID, VolumeName, @{N="Total(GB)";E={[math]::Round($_.Size/1GB,2)}}, @{N="Used(GB)";E={[math]::Round(($_.Size - $_.FreeSpace)/1GB,2)}}, @{N="Free(GB)";E={[math]::Round($_.FreeSpace/1GB,2)}}, @{N="Usage(%)";E={[math]::Round(($_.Size - $_.FreeSpace)/$_.Size*100,2)}}', category: 'disk' },
  { name: '磁盘健康', command: 'Get-PhysicalDisk | Select-Object FriendlyName, MediaType, OperationalStatus, HealthStatus', category: 'disk' },
  { name: '大文件查找', command: 'Get-ChildItem C:\\ -Recurse -File -ErrorAction SilentlyContinue | Sort-Object Length -Descending | Select-Object -First 20 FullName, @{N="Size(MB)";E={[math]::Round($_.Length/1MB,2)}}', category: 'disk' }
]

const serviceCommands: CommandItem[] = [
  { name: '所有服务', command: 'Get-Service | Select-Object Name, DisplayName, Status, StartType | Format-Table -AutoSize', category: 'service' },
  { name: '运行中的服务', command: 'Get-Service | Where-Object {$_.Status -eq "Running"} | Select-Object Name, DisplayName', category: 'service' },
  { name: '已停止的服务', command: 'Get-Service | Where-Object {$_.Status -eq "Stopped"} | Select-Object Name, DisplayName', category: 'service' },
  { name: '自动启动服务', command: 'Get-Service | Where-Object {$_.StartType -eq "Automatic"} | Select-Object Name, DisplayName, Status', category: 'service' },
  { name: '启动服务(需输入)', command: '$name = Read-Host "输入服务名"; Start-Service -Name $name; Write-Host "服务 $name 已启动"', category: 'service' },
  { name: '停止服务(需输入)', command: '$name = Read-Host "输入服务名"; Stop-Service -Name $name -Force; Write-Host "服务 $name 已停止"', category: 'service' },
  { name: '服务依赖', command: '$name = Read-Host "输入服务名"; Get-Service -Name $name | Select-Object Name, DisplayName, ServicesDependedOn, DependentServices', category: 'service' }
]

const fileCommands: CommandItem[] = [
  { name: '当前目录', command: 'Get-Location', category: 'file' },
  { name: '目录内容', command: 'Get-ChildItem | Select-Object Name, Length, LastWriteTime, Mode', category: 'file' },
  { name: '查找文件', command: '$pattern = Read-Host "输入文件名模式"; Get-ChildItem -Recurse -Filter $pattern -ErrorAction SilentlyContinue | Select-Object FullName, Length, LastWriteTime', category: 'file' },
  { name: '文件内容搜索', command: '$text = Read-Host "输入搜索内容"; $path = Read-Host "输入路径"; Select-String -Path "$path\\*" -Pattern $text -Recurse | Select-Object Path, LineNumber, Line', category: 'file' },
  { name: '创建目录', command: '$path = Read-Host "输入目录路径"; New-Item -ItemType Directory -Path $path', category: 'file' },
  { name: '文件哈希', command: '$file = Read-Host "输入文件路径"; Get-FileHash -Path $file -Algorithm SHA256', category: 'file' },
  { name: '压缩文件', command: '$source = Read-Host "输入源路径"; $dest = Read-Host "输入目标zip路径"; Compress-Archive -Path $source -DestinationPath $dest', category: 'file' },
  { name: '解压文件', command: '$source = Read-Host "输入zip路径"; $dest = Read-Host "输入目标目录"; Expand-Archive -Path $source -DestinationPath $dest', category: 'file' }
]

const securityCommands: CommandItem[] = [
  { name: '防火墙状态', command: 'Get-NetFirewallProfile | Select-Object Name, Enabled', category: 'security' },
  { name: '防火墙规则', command: 'Get-NetFirewallRule | Where-Object {$_.Enabled -eq $true} | Select-Object DisplayName, Direction, Action, Profile | Format-Table -AutoSize', category: 'security' },
  { name: '开放端口', command: 'Get-NetFirewallRule | Where-Object {$_.Action -eq "Allow" -and $_.Enabled -eq $true} | Get-NetFirewallPortFilter | Select-Object Protocol, LocalPort', category: 'security' },
  { name: '用户组', command: 'Get-LocalGroup | Select-Object Name, Description', category: 'security' },
  { name: '管理员组成员', command: 'Get-LocalGroupMember -Group "Administrators" | Select-Object Name, ObjectClass', category: 'security' },
  { name: '登录日志', command: 'Get-WinEvent -LogName Security -MaxEvents 20 | Where-Object {$_.Id -eq 4624} | Select-Object TimeCreated, @{N="User";E={$_.Properties[5].Value}}, @{N="LogonType";E={$_.Properties[8].Value}}', category: 'security' },
  { name: 'Windows更新', command: 'Get-HotFix | Select-Object HotFixID, Description, InstalledOn | Sort-Object InstalledOn -Descending | Select-Object -First 10', category: 'security' }
]

const filteredSystemCommands = computed(() => filterCommands(systemCommands))
const filteredNetworkCommands = computed(() => filterCommands(networkCommands))
const filteredProcessCommands = computed(() => filterCommands(processCommands))
const filteredDiskCommands = computed(() => filterCommands(diskCommands))
const filteredServiceCommands = computed(() => filterCommands(serviceCommands))
const filteredFileCommands = computed(() => filterCommands(fileCommands))
const filteredSecurityCommands = computed(() => filterCommands(securityCommands))

function filterCommands(commands: CommandItem[]) {
  if (!searchCommand.value) return commands
  const keyword = searchCommand.value.toLowerCase()
  return commands.filter(cmd => 
    cmd.name.toLowerCase().includes(keyword) || 
    cmd.command.toLowerCase().includes(keyword)
  )
}

function isFavorite(cmd: CommandItem) {
  return favorites.value.some(f => f.command === cmd.command)
}

function toggleFavorite(cmd: CommandItem) {
  if (isFavorite(cmd)) {
    favorites.value = favorites.value.filter(f => f.command !== cmd.command)
    ElMessage.success('已取消收藏')
  } else {
    pendingCommand.value = cmd
    newFavoriteName.value = cmd.name
    showSaveDialog.value = true
  }
}

function saveToFavorite() {
  if (!pendingCommand.value || !newFavoriteName.value.trim()) return
  
  const newFav: CommandItem = {
    name: newFavoriteName.value.trim(),
    command: pendingCommand.value.command,
    category: pendingCommand.value.category
  }
  
  if (!isFavorite(newFav)) {
    favorites.value.push(newFav)
    ElMessage.success('已添加到收藏')
  }
  
  showSaveDialog.value = false
  pendingCommand.value = null
  newFavoriteName.value = ''
  saveFavorites()
}

function removeFavorite(cmd: CommandItem) {
  favorites.value = favorites.value.filter(f => f.command !== cmd.command)
  saveFavorites()
  ElMessage.success('已从收藏中移除')
}

function addToFavoritesFromHistory(row: { command: string }) {
  pendingCommand.value = { name: '', command: row.command }
  newFavoriteName.value = ''
  showSaveDialog.value = true
}

function insertCommand(cmd: string) {
  command.value = cmd
}

async function executeCommand() {
  if (!command.value.trim()) {
    ElMessage.warning('请输入命令')
    return
  }
  
  executing.value = true
  output.value = '正在执行...\n'
  
  try {
    const result = await invoke<CommandResult>('execute_powershell', {
      command: command.value
    })
    
    if (result.success) {
      output.value = result.output || '命令执行成功，无输出。'
    } else {
      output.value = `错误: ${result.error}`
    }
    
    const historyItem = {
      command: command.value,
      time: new Date().toLocaleString('zh-CN'),
      success: result.success
    }
    
    history.value.unshift(historyItem)
    
    if (history.value.length > appStore.settings.powershell.maxHistoryItems) {
      history.value = history.value.slice(0, appStore.settings.powershell.maxHistoryItems)
    }
    
    saveHistory()
    
    appStore.addNotification(
      result.success ? 'success' : 'error',
      result.success ? '命令执行完成' : '命令执行失败',
      command.value.substring(0, 50)
    )
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

async function copyOutput() {
  if (!output.value) return
  
  try {
    await navigator.clipboard.writeText(output.value)
    ElMessage.success('已复制到剪贴板')
  } catch {
    ElMessage.error('复制失败')
  }
}

function exportOutput() {
  if (!output.value) return
  
  const blob = new Blob([output.value], { type: 'text/plain;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `powershell-output-${new Date().toISOString().slice(0, 10)}.txt`
  a.click()
  URL.revokeObjectURL(url)
  ElMessage.success('导出成功')
}

function clearHistory() {
  history.value = []
  localStorage.removeItem('powershell-history')
  ElMessage.success('历史记录已清空')
}

function saveHistory() {
  localStorage.setItem('powershell-history', JSON.stringify(history.value))
}

function saveFavorites() {
  localStorage.setItem('powershell-favorites', JSON.stringify(favorites.value))
}

function loadHistory() {
  const saved = localStorage.getItem('powershell-history')
  if (saved) {
    history.value = JSON.parse(saved)
  }
}

function loadFavorites() {
  const saved = localStorage.getItem('powershell-favorites')
  if (saved) {
    favorites.value = JSON.parse(saved)
  }
}

onMounted(() => {
  loadHistory()
  loadFavorites()
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
  gap: 2px;
}

.command-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  cursor: pointer;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.command-item:hover {
  background-color: #f0f2f5;
}

.cmd-name {
  font-size: 13px;
  color: #303133;
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
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: #f5f7fa;
  padding: 8px 12px;
  border-bottom: 1px solid #dcdfe6;
  font-weight: 600;
  font-size: 13px;
}

.output-actions {
  display: flex;
  gap: 4px;
}

.output-content {
  background: #1e1e1e;
  color: #d4d4d4;
  padding: 16px;
  margin: 0;
  min-height: 300px;
  max-height: 400px;
  overflow: auto;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-all;
}

.commands-card {
  height: calc(100vh - 160px);
  overflow-y: auto;
}

.empty-favorites {
  text-align: center;
  padding: 40px 20px;
  color: #909399;
}

.empty-favorites .tip {
  font-size: 12px;
  margin-top: 8px;
}

.cmd-code {
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  background: #f5f7fa;
  padding: 2px 6px;
  border-radius: 3px;
}

:deep(.el-collapse-item__header) {
  font-weight: 600;
  font-size: 13px;
}

:deep(.el-tabs__content) {
  max-height: calc(100vh - 350px);
  overflow-y: auto;
}
</style>
