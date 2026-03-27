import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import type { AppSettings, NotificationItem } from '@/types'

const defaultSettings: AppSettings = {
  general: {
    theme: 'light',
    language: 'zh-CN',
    autoStart: false,
    minimizeToTray: false,
    confirmDangerousActions: true
  },
  powershell: {
    defaultTimeout: 30000,
    saveHistory: true,
    maxHistoryItems: 100
  },
  registry: {
    showHiddenKeys: false,
    confirmDeletes: true,
    autoRefresh: true
  }
}

const STORAGE_KEY = 'windows-toolbox-settings'

export const useAppStore = defineStore('app', () => {
  const settings = ref<AppSettings>(loadSettings())
  const notifications = ref<NotificationItem[]>([])
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

  function initialize(): void {
    if (isInitialized.value) return
    settings.value = loadSettings()
    addNotification('info', '系统启动', 'Windows工具箱已成功加载')
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
    isInitialized,
    isLoading,
    updateSettings,
    resetSettings,
    addNotification,
    markNotificationRead,
    markAllNotificationsRead,
    clearNotifications,
    getUnreadCount,
    initialize
  }
})
