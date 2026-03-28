import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import type { AppSettings, NotificationItem } from '@/types'

interface OperationLog {
  id: string
  module: string
  action: string
  status: 'success' | 'error' | 'warning'
  message: string
  details?: string
  timestamp: string
}

const defaultSettings: AppSettings = {
  general: {
    theme: 'light',
    language: 'zh-CN',
    autoStart: false,
    minimizeToTray: false,
    confirmDangerousActions: true,
    showNotifications: true,
    startMinimized: false
  },
  powershell: {
    defaultTimeout: 30000,
    saveHistory: true,
    maxHistoryItems: 100,
    executionPolicy: 'RemoteSigned',
    outputFontSize: 14
  },
  registry: {
    showHiddenKeys: false,
    confirmDeletes: true,
    autoRefresh: true
  },
  process: {
    autoRefresh: true,
    refreshInterval: 5,
    showSystemProcesses: true,
    highCpuThreshold: 50,
    highMemoryThreshold: 500
  },
  data: {
    autoBackup: false,
    backupInterval: 'weekly'
  }
}

const STORAGE_KEY = 'windows-toolbox-settings'
const LOGS_KEY = 'windows-toolbox-logs'

export const useAppStore = defineStore('app', () => {
  const settings = ref<AppSettings>(loadSettings())
  const notifications = ref<NotificationItem[]>([])
  const operationLogs = ref<OperationLog[]>(loadLogs())
  const isInitialized = ref(false)
  const isLoading = ref(false)

  function loadSettings(): AppSettings {
    try {
      const saved = localStorage.getItem(STORAGE_KEY)
      if (saved) {
        const parsed = JSON.parse(saved)
        return deepMerge(defaultSettings, parsed)
      }
    } catch (e) {
      console.error('Failed to load settings:', e)
    }
    return { ...defaultSettings }
  }

  function loadLogs(): OperationLog[] {
    try {
      const saved = localStorage.getItem(LOGS_KEY)
      if (saved) {
        return JSON.parse(saved)
      }
    } catch (e) {
      console.error('Failed to load logs:', e)
    }
    return []
  }

  function deepMerge<T extends Record<string, any>>(target: T, source: Partial<T>): T {
    const result = { ...target }
    for (const key in source) {
      if (source[key] && typeof source[key] === 'object' && !Array.isArray(source[key])) {
        result[key] = deepMerge(result[key] as any, source[key] as any)
      } else {
        result[key] = source[key] as any
      }
    }
    return result
  }

  function saveSettings(): void {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(settings.value))
    } catch (e) {
      console.error('Failed to save settings:', e)
    }
  }

  function saveLogs(): void {
    try {
      const logsToSave = operationLogs.value.slice(-500)
      localStorage.setItem(LOGS_KEY, JSON.stringify(logsToSave))
    } catch (e) {
      console.error('Failed to save logs:', e)
    }
  }

  function updateSettings(updates: Partial<AppSettings>): void {
    settings.value = deepMerge(settings.value, updates)
    saveSettings()
  }

  function resetSettings(): void {
    settings.value = { ...defaultSettings }
    saveSettings()
  }

  function addNotification(
    type: NotificationItem['type'],
    title: string,
    message: string
  ): void {
    const notification: NotificationItem = {
      id: Date.now().toString(36) + Math.random().toString(36).substr(2, 9),
      type,
      title,
      message,
      timestamp: new Date().toISOString(),
      read: false
    }

    notifications.value.push(notification)

    if (notifications.value.length > 100) {
      notifications.value = notifications.value.slice(-100)
    }
  }

  function markNotificationRead(id: string): void {
    const notification = notifications.value.find(n => n.id === id)
    if (notification) {
      notification.read = true
    }
  }

  function markAllNotificationsRead(): void {
    notifications.value.forEach(n => {
      n.read = true
    })
  }

  function clearNotifications(): void {
    notifications.value = []
  }

  function getUnreadCount(): number {
    return notifications.value.filter(n => !n.read).length
  }

  function addLog(
    module: string,
    action: string,
    status: 'success' | 'error' | 'warning',
    message: string,
    details?: string
  ): void {
    const log: OperationLog = {
      id: Date.now().toString(36) + Math.random().toString(36).substr(2, 9),
      module,
      action,
      status,
      message,
      details,
      timestamp: new Date().toISOString()
    }

    operationLogs.value.push(log)

    if (operationLogs.value.length > 500) {
      operationLogs.value = operationLogs.value.slice(-500)
    }

    saveLogs()
  }

  function clearLogs(): void {
    operationLogs.value = []
    saveLogs()
  }

  function getLogsByModule(module: string): OperationLog[] {
    return operationLogs.value.filter(log => log.module === module)
  }

  function getLogsByStatus(status: 'success' | 'error' | 'warning'): OperationLog[] {
    return operationLogs.value.filter(log => log.status === status)
  }

  function getRecentLogs(count: number = 50): OperationLog[] {
    return operationLogs.value.slice(-count)
  }

  function initialize(): void {
    if (isInitialized.value) return
    settings.value = loadSettings()
    operationLogs.value = loadLogs()
    addLog('system', '启动', 'success', 'Windows工具箱已成功启动')
    isInitialized.value = true
  }

  watch(settings, () => {
    if (isInitialized.value) {
      saveSettings()
    }
  }, { deep: true })

  return {
    settings,
    notifications,
    operationLogs,
    isInitialized,
    isLoading,
    updateSettings,
    resetSettings,
    addNotification,
    markNotificationRead,
    markAllNotificationsRead,
    clearNotifications,
    getUnreadCount,
    addLog,
    clearLogs,
    getLogsByModule,
    getLogsByStatus,
    getRecentLogs,
    initialize
  }
})

export type { OperationLog }
