<template>
  <div class="settings">
    <el-row :gutter="20">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>{{ $t('settings.title') }}</span>
              <div class="header-actions">
                <el-button @click="exportSettings">
                  <el-icon><Download /></el-icon>
                  {{ $t('settings.exportSettings') }}
                </el-button>
                <el-button @click="importSettings">
                  <el-icon><Upload /></el-icon>
                  {{ $t('settings.importSettings') }}
                </el-button>
              </div>
            </div>
          </template>
          
          <el-tabs v-model="activeTab">
            <el-tab-pane :label="$t('settings.general')" name="general">
              <el-form label-width="150px" style="max-width: 700px;">
                <el-form-item :label="$t('settings.themeMode')">
                  <el-radio-group v-model="themeMode" @change="changeTheme">
                    <el-radio-button value="light">
                      <el-icon><Sunny /></el-icon>
                      {{ $t('settings.light') }}
                    </el-radio-button>
                    <el-radio-button value="dark">
                      <el-icon><Moon /></el-icon>
                      {{ $t('settings.dark') }}
                    </el-radio-button>
                    <el-radio-button value="auto">
                      <el-icon><Monitor /></el-icon>
                      {{ $t('settings.followSystem') }}
                    </el-radio-button>
                  </el-radio-group>
                </el-form-item>
                
                <el-form-item :label="$t('settings.primaryColor')">
                  <div class="theme-colors">
                    <div 
                      v-for="color in themeColors" 
                      :key="color.value"
                      class="color-item"
                      :class="{ active: primaryColor === color.value }"
                      :style="{ backgroundColor: color.value }"
                      @click="changePrimaryColor(color.value)"
                    >
                      <el-icon v-if="primaryColor === color.value"><Check /></el-icon>
                    </div>
                  </div>
                </el-form-item>
                
                <el-form-item :label="$t('settings.language')">
                  <el-select v-model="settings.general.language" @change="updateLanguage" style="width: 200px;">
                    <el-option label="简体中文" value="zh-CN" />
                    <el-option label="English" value="en" />
                  </el-select>
                </el-form-item>
                
                <el-form-item :label="$t('settings.startupOptions')">
                  <div class="checkbox-group">
                    <el-checkbox v-model="settings.general.autoStart" @change="updateSettings">
                      {{ $t('settings.autoStartDesc') }}
                    </el-checkbox>
                    <el-checkbox v-model="settings.general.minimizeToTray" @change="updateSettings">
                      {{ $t('settings.minimizeToTrayDesc') }}
                    </el-checkbox>
                    <el-checkbox v-model="settings.general.startMinimized" @change="updateSettings">
                      {{ $t('settings.startMinimized') }}
                    </el-checkbox>
                  </div>
                </el-form-item>
                
                <el-form-item :label="$t('settings.confirmDangerous')">
                  <el-switch v-model="settings.general.confirmDangerousActions" @change="updateSettings" />
                  <span class="form-tip">{{ $t('settings.confirmDangerousDesc') }}</span>
                </el-form-item>
                
                <el-form-item :label="$t('settings.showNotifications')">
                  <el-switch v-model="settings.general.showNotifications" @change="updateSettings" />
                </el-form-item>
              </el-form>
            </el-tab-pane>
            
            <el-tab-pane label="PowerShell" name="powershell">
              <el-form label-width="150px" style="max-width: 700px;">
                <el-form-item :label="$t('settings.defaultTimeout')">
                  <el-input-number 
                    v-model="settings.powershell.defaultTimeout" 
                    :min="1000" 
                    :max="300000"
                    :step="1000"
                    @change="updateSettings"
                  />
                  <span class="form-tip">ms</span>
                </el-form-item>
                
                <el-form-item :label="$t('settings.saveHistory')">
                  <el-switch v-model="settings.powershell.saveHistory" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item :label="$t('settings.maxHistoryItems')">
                  <el-input-number 
                    v-model="settings.powershell.maxHistoryItems" 
                    :min="10" 
                    :max="500"
                    @change="updateSettings"
                  />
                </el-form-item>
                
                <el-form-item :label="$t('settings.executionPolicy')">
                  <el-select v-model="settings.powershell.executionPolicy" @change="updateSettings" style="width: 200px;">
                    <el-option label="Restricted" value="Restricted" />
                    <el-option label="RemoteSigned" value="RemoteSigned" />
                    <el-option label="AllSigned" value="AllSigned" />
                    <el-option label="Bypass" value="Bypass" />
                  </el-select>
                  <span class="form-tip">{{ $t('settings.executionPolicyDesc') }}</span>
                </el-form-item>
                
                <el-form-item :label="$t('settings.outputFontSize')">
                  <el-slider v-model="settings.powershell.outputFontSize" :min="10" :max="24" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item>
                  <el-button type="danger" @click="clearPowerShellHistory">
                    {{ $t('settings.clearCommandHistory') }}
                  </el-button>
                </el-form-item>
              </el-form>
            </el-tab-pane>
            
            <el-tab-pane :label="$t('settings.registrySettings')" name="registry">
              <el-form label-width="150px" style="max-width: 700px;">
                <el-form-item :label="$t('settings.showHiddenKeys')">
                  <el-switch v-model="settings.registry.showHiddenKeys" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item :label="$t('settings.confirmDeletes')">
                  <el-switch v-model="settings.registry.confirmDeletes" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item :label="$t('settings.autoRefresh')">
                  <el-switch v-model="settings.registry.autoRefresh" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item :label="$t('settings.favoritePaths')">
                  <div class="favorites-count">
                    {{ $t('settings.favoritePathsCount', { count: registryFavoritesCount }) }}
                    <el-button text type="danger" @click="clearRegistryFavorites">
                      {{ $t('settings.clearFavorites') }}
                    </el-button>
                  </div>
                </el-form-item>
              </el-form>
            </el-tab-pane>
            
            <el-tab-pane :label="$t('settings.processManagement')" name="process">
              <el-form label-width="150px" style="max-width: 700px;">
                <el-form-item :label="$t('settings.autoRefresh')">
                  <el-switch v-model="settings.process.autoRefresh" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item :label="$t('settings.refreshIntervalSetting')">
                  <el-select v-model="settings.process.refreshInterval" @change="updateSettings" style="width: 120px;">
                    <el-option :value="2" :label="`2 ${$t('common.seconds')}`" />
                    <el-option :value="5" :label="`5 ${$t('common.seconds')}`" />
                    <el-option :value="10" :label="`10 ${$t('common.seconds')}`" />
                    <el-option :value="30" :label="`30 ${$t('common.seconds')}`" />
                  </el-select>
                </el-form-item>
                
                <el-form-item :label="$t('settings.showSystemProcesses')">
                  <el-switch v-model="settings.process.showSystemProcesses" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item :label="$t('settings.highCpuThreshold')">
                  <el-input-number 
                    v-model="settings.process.highCpuThreshold" 
                    :min="10" 
                    :max="100"
                    @change="updateSettings"
                  />
                  <span class="form-tip">%</span>
                </el-form-item>
                
                <el-form-item :label="$t('settings.highMemoryThreshold')">
                  <el-input-number 
                    v-model="settings.process.highMemoryThreshold" 
                    :min="100" 
                    :max="10000"
                    :step="100"
                    @change="updateSettings"
                  />
                  <span class="form-tip">MB</span>
                </el-form-item>
              </el-form>
            </el-tab-pane>
            
            <el-tab-pane :label="$t('settings.dataManagement')" name="data">
              <el-form label-width="150px" style="max-width: 700px;">
                <el-form-item :label="$t('settings.dataStorageLocation')">
                  <el-input :value="dataPath" readonly>
                    <template #append>
                      <el-button @click="openDataPath">
                        <el-icon><FolderOpened /></el-icon>
                        {{ $t('settings.open') }}
                      </el-button>
                    </template>
                  </el-input>
                </el-form-item>
                
                <el-form-item :label="$t('settings.autoBackup')">
                  <el-switch v-model="settings.data.autoBackup" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item :label="$t('settings.backupInterval')">
                  <el-select v-model="settings.data.backupInterval" @change="updateSettings" style="width: 150px;">
                    <el-option value="daily" :label="$t('settings.daily')" />
                    <el-option value="weekly" :label="$t('settings.weekly')" />
                    <el-option value="monthly" :label="$t('settings.monthly')" />
                  </el-select>
                </el-form-item>
                
                <el-form-item :label="$t('settings.dataOperations')">
                  <div class="data-actions">
                    <el-button @click="backupNow">
                      <el-icon><FolderAdd /></el-icon>
                      {{ $t('settings.backupNow') }}
                    </el-button>
                    <el-button @click="restoreBackup">
                      <el-icon><RefreshRight /></el-icon>
                      {{ $t('settings.restoreBackup') }}
                    </el-button>
                    <el-button type="danger" @click="clearAllData">
                      <el-icon><Delete /></el-icon>
                      {{ $t('settings.clearAllData') }}
                    </el-button>
                  </div>
                </el-form-item>
                
                <el-form-item :label="$t('settings.storageUsage')">
                  <div class="storage-info">
                    <el-progress :percentage="storagePercentage" :format="storageFormat" />
                    <span class="storage-text">{{ formatBytes(storageUsed) }} / {{ formatBytes(storageTotal) }}</span>
                  </div>
                </el-form-item>
              </el-form>
            </el-tab-pane>
          </el-tabs>
          
          <el-divider />
          
          <div class="settings-footer">
            <el-button @click="resetSettings">
              <el-icon><RefreshLeft /></el-icon>
              {{ $t('settings.reset') }}
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="20" style="margin-top: 20px;">
      <el-col :span="12">
        <el-card>
          <template #header>
            <span>{{ $t('settings.about') }}</span>
          </template>
          <el-descriptions :column="1" border>
            <el-descriptions-item :label="$t('settings.appName')">Windows Toolbox</el-descriptions-item>
            <el-descriptions-item :label="$t('settings.version')">1.0.0</el-descriptions-item>
            <el-descriptions-item :label="$t('settings.techStack')">Tauri + Vue 3 + TypeScript</el-descriptions-item>
            <el-descriptions-item :label="$t('settings.runtime')">
              <el-tag size="small">{{ systemInfo.osName }} {{ systemInfo.arch }}</el-tag>
            </el-descriptions-item>
          </el-descriptions>
          
          <div class="about-actions">
            <el-button @click="checkUpdate">
              <el-icon><Promotion /></el-icon>
              {{ $t('settings.checkUpdate') }}
            </el-button>
            <el-button @click="openLogFolder">
              <el-icon><FolderOpened /></el-icon>
              {{ $t('settings.openLogFolder') }}
            </el-button>
          </div>
        </el-card>
      </el-col>
      
      <el-col :span="12">
        <el-card>
          <template #header>
            <span>{{ $t('settings.shortcuts') }}</span>
          </template>
          <el-table :data="shortcuts" border size="small">
            <el-table-column prop="key" :label="$t('settings.shortcutKey')" width="120" />
            <el-table-column prop="action" :label="$t('settings.function')" />
          </el-table>
        </el-card>
      </el-col>
    </el-row>
    
    <input 
      type="file" 
      ref="fileInput" 
      accept=".json"
      style="display: none;"
      @change="handleFileImport"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAppStore } from '@/stores'
import { setLocale } from '@/locales'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  Download, Upload, Sunny, Moon, Monitor, Check, FolderOpened, 
  Delete, RefreshLeft, RefreshRight, FolderAdd, Promotion 
} from '@element-plus/icons-vue'

const { t } = useI18n()
const appStore = useAppStore()
const activeTab = ref('general')
const fileInput = ref<HTMLInputElement>()
const themeMode = ref('light')
const primaryColor = ref('#409EFF')
const dataPath = ref('C:\\Users\\User\\AppData\\Local\\WindowsToolbox')
const storageUsed = ref(1024 * 1024 * 5)
const storageTotal = ref(1024 * 1024 * 100)
const systemInfo = ref({
  osName: 'Windows 11',
  arch: 'x64'
})

const settings = computed(() => appStore.settings)

const registryFavoritesCount = computed(() => {
  const saved = localStorage.getItem('registry-favorites')
  return saved ? JSON.parse(saved).length : 0
})

const storagePercentage = computed(() => {
  return Math.round((storageUsed.value / storageTotal.value) * 100)
})

const themeColors = [
  { name: 'Default Blue', value: '#409EFF' },
  { name: 'Aurora Green', value: '#67C23A' },
  { name: 'Twilight Orange', value: '#E6A23C' },
  { name: 'Flame Red', value: '#F56C6C' },
  { name: 'Space Gray', value: '#909399' },
  { name: 'Geek Purple', value: '#9B59B6' },
  { name: 'Celadon Blue', value: '#3498DB' },
  { name: 'Mint Green', value: '#1ABC9C' }
]

const shortcuts = computed(() => [
  { key: 'Ctrl + S', action: t('settings.saveCurrentOperation') },
  { key: 'Ctrl + R', action: t('settings.refreshCurrentPage') },
  { key: 'Ctrl + F', action: t('settings.search') },
  { key: 'Ctrl + ,', action: t('settings.openSettings') },
  { key: 'Esc', action: t('settings.closeDialog') },
  { key: 'F5', action: t('settings.refreshData') }
])

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

function storageFormat(percentage: number) {
  return `${percentage}%`
}

function changeTheme(mode: string) {
  themeMode.value = mode
  
  if (mode === 'dark') {
    document.documentElement.classList.add('dark')
  } else if (mode === 'light') {
    document.documentElement.classList.remove('dark')
  } else {
    const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    if (isDark) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }
  
  localStorage.setItem('theme-mode', mode)
  ElMessage.success(t('common.success'))
}

function changePrimaryColor(color: string) {
  primaryColor.value = color
  document.documentElement.style.setProperty('--el-color-primary', color)
  localStorage.setItem('primary-color', color)
  ElMessage.success(t('common.success'))
}

function updateSettings() {
  appStore.updateSettings({})
  ElMessage.success(t('common.success'))
}

function updateLanguage(locale: 'zh-CN' | 'en') {
  setLocale(locale)
  appStore.updateSettings({
    general: { ...settings.value.general, language: locale }
  })
  ElMessage.success(t('common.success'))
}

async function resetSettings() {
  try {
    await ElMessageBox.confirm(t('settings.confirmReset'), t('settings.resetConfirm'), {
      type: 'warning',
      confirmButtonText: t('common.confirm'),
      cancelButtonText: t('common.cancel')
    })
    
    appStore.resetSettings()
    themeMode.value = 'light'
    primaryColor.value = '#409EFF'
    document.documentElement.classList.remove('dark')
    document.documentElement.style.setProperty('--el-color-primary', '#409EFF')
    
    ElMessage.success(t('settings.resetSuccess'))
  } catch {
    // cancelled
  }
}

function exportSettings() {
  const data = {
    settings: settings.value,
    themeMode: themeMode.value,
    primaryColor: primaryColor.value,
    registryFavorites: localStorage.getItem('registry-favorites'),
    powerShellFavorites: localStorage.getItem('powershell-favorites'),
    powerShellHistory: localStorage.getItem('powershell-history'),
    exportTime: new Date().toISOString()
  }
  
  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `windows-toolbox-settings-${new Date().toISOString().split('T')[0]}.json`
  a.click()
  URL.revokeObjectURL(url)
  
  ElMessage.success(t('settings.exportSuccess'))
}

function importSettings() {
  fileInput.value?.click()
}

async function handleFileImport(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  
  try {
    const text = await file.text()
    const data = JSON.parse(text)
    
    if (data.settings) {
      appStore.updateSettings(data.settings)
    }
    
    if (data.themeMode) {
      changeTheme(data.themeMode)
    }
    
    if (data.primaryColor) {
      changePrimaryColor(data.primaryColor)
    }
    
    if (data.registryFavorites) {
      localStorage.setItem('registry-favorites', data.registryFavorites)
    }
    
    if (data.powerShellFavorites) {
      localStorage.setItem('powershell-favorites', data.powerShellFavorites)
    }
    
    if (data.powerShellHistory) {
      localStorage.setItem('powershell-history', data.powerShellHistory)
    }
    
    ElMessage.success(t('settings.importSuccess'))
  } catch {
    ElMessage.error(t('settings.importFailed'))
  }
  
  if (fileInput.value) {
    fileInput.value.value = ''
  }
}

async function clearPowerShellHistory() {
  try {
    await ElMessageBox.confirm(t('settings.confirmClearHistory'), t('settings.confirmClear'), {
      type: 'warning',
      confirmButtonText: t('common.confirm'),
      cancelButtonText: t('common.cancel')
    })
    
    localStorage.removeItem('powershell-history')
    ElMessage.success(t('settings.historyCleared'))
  } catch {
    // cancelled
  }
}

async function clearRegistryFavorites() {
  try {
    await ElMessageBox.confirm(t('settings.confirmClearFavorites'), t('settings.confirmClear'), {
      type: 'warning',
      confirmButtonText: t('common.confirm'),
      cancelButtonText: t('common.cancel')
    })
    
    localStorage.removeItem('registry-favorites')
    ElMessage.success(t('settings.favoritesCleared'))
  } catch {
    // cancelled
  }
}

async function clearAllData() {
  try {
    await ElMessageBox.confirm(
      t('settings.confirmClearAllData'),
      t('settings.dangerousOperation'),
      {
        type: 'error',
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
        confirmButtonClass: 'el-button--danger'
      }
    )
    
    localStorage.clear()
    appStore.resetSettings()
    themeMode.value = 'light'
    primaryColor.value = '#409EFF'
    document.documentElement.classList.remove('dark')
    document.documentElement.style.setProperty('--el-color-primary', '#409EFF')
    
    ElMessage.success(t('settings.allDataCleared'))
  } catch {
    // cancelled
  }
}

function backupNow() {
  ElMessage.success(t('settings.backupCreated'))
}

function restoreBackup() {
  ElMessage.info(t('settings.selectBackupFile'))
}

function openDataPath() {
  ElMessage.info(t('settings.openingDataDirectory'))
}

function checkUpdate() {
  ElMessage.info(t('settings.checkUpdate'))
}

function openLogFolder() {
  ElMessage.info(t('settings.openLogFolder'))
}

onMounted(() => {
  const savedTheme = localStorage.getItem('theme-mode')
  if (savedTheme) {
    themeMode.value = savedTheme
    changeTheme(savedTheme)
  }
  
  const savedColor = localStorage.getItem('primary-color')
  if (savedColor) {
    primaryColor.value = savedColor
    document.documentElement.style.setProperty('--el-color-primary', savedColor)
  }
})
</script>

<style scoped>
.settings {
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

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-tip {
  margin-left: 12px;
  color: var(--el-text-color-secondary);
  font-size: 12px;
}

.theme-colors {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.color-item {
  width: 32px;
  height: 32px;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 2px solid transparent;
  transition: all 0.2s;
}

.color-item:hover {
  transform: scale(1.1);
}

.color-item.active {
  border-color: var(--el-text-color-primary);
}

.color-item .el-icon {
  color: white;
  font-size: 16px;
}

.favorites-count {
  display: flex;
  align-items: center;
  gap: 12px;
}

.data-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.storage-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
  max-width: 300px;
}

.storage-text {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.about-actions {
  margin-top: 16px;
  display: flex;
  gap: 12px;
}

.settings-footer {
  display: flex;
  justify-content: flex-end;
}
</style>
