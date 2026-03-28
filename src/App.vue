<template>
  <div class="app-container" :class="{ 'is-collapsed': isCollapsed }">
    <aside class="sidebar">
      <div class="sidebar-header">
        <el-icon v-if="!isCollapsed" size="28"><Tools /></el-icon>
        <span v-if="!isCollapsed" class="logo-text">{{ $t('app.title') }}</span>
        <el-icon v-else size="24"><Tools /></el-icon>
      </div>
      
      <el-menu
        :default-active="currentRoute"
        class="sidebar-menu"
        :collapse="isCollapsed"
        router
      >
        <el-menu-item index="/">
          <el-icon><HomeFilled /></el-icon>
          <template #title>{{ $t('nav.dashboard') }}</template>
        </el-menu-item>
        
        <el-menu-item index="/powershell">
          <el-icon><SetUp /></el-icon>
          <template #title>{{ $t('nav.powershell') }}</template>
        </el-menu-item>
        
        <el-menu-item index="/registry">
          <el-icon><Collection /></el-icon>
          <template #title>{{ $t('nav.registry') }}</template>
        </el-menu-item>
        
        <el-menu-item index="/services">
          <el-icon><Service /></el-icon>
          <template #title>{{ $t('nav.services') }}</template>
        </el-menu-item>
        
        <el-menu-item index="/processes">
          <el-icon><DataLine /></el-icon>
          <template #title>{{ $t('nav.processes') }}</template>
        </el-menu-item>
        
        <el-menu-item index="/network">
          <el-icon><Connection /></el-icon>
          <template #title>{{ $t('nav.network') }}</template>
        </el-menu-item>
        
        <el-menu-item index="/disk">
          <el-icon><FolderOpened /></el-icon>
          <template #title>{{ $t('nav.disk') }}</template>
        </el-menu-item>
        
        <el-menu-item index="/quick-actions">
          <el-icon><Position /></el-icon>
          <template #title>{{ $t('nav.quickActions') }}</template>
        </el-menu-item>
        
        <el-menu-item index="/optimization">
          <el-icon><TrendCharts /></el-icon>
          <template #title>{{ $t('nav.optimization') }}</template>
        </el-menu-item>
        
        <el-menu-item index="/advanced">
          <el-icon><Setting /></el-icon>
          <template #title>{{ $t('nav.advanced') }}</template>
        </el-menu-item>
      </el-menu>
      
      <div class="sidebar-footer">
        <el-menu
          :default-active="currentRoute"
          class="sidebar-menu-bottom"
          :collapse="isCollapsed"
          router
        >
          <el-menu-item index="/settings">
            <el-icon><Operation /></el-icon>
            <template #title>{{ $t('nav.settings') }}</template>
          </el-menu-item>
          
          <el-menu-item index="/logs">
            <el-icon><Document /></el-icon>
            <template #title>{{ $t('nav.logs') }}</template>
          </el-menu-item>
        </el-menu>
        <el-button 
          class="collapse-btn" 
          :icon="isCollapsed ? 'Expand' : 'Fold'" 
          @click="toggleSidebar"
          text
        />
      </div>
    </aside>
    
    <main class="main-content">
      <header class="main-header">
        <div class="header-left">
          <h1 class="page-title">{{ currentPageTitle }}</h1>
        </div>
        <div class="header-right">
          <el-button circle @click="showHelp = true" :title="$t('help.title')">
            <el-icon><QuestionFilled /></el-icon>
          </el-button>
          <LanguageSwitcher />
          <el-badge :value="unreadCount" :hidden="unreadCount === 0" class="notification-badge">
            <el-button circle @click="showNotifications = true">
              <el-icon><Bell /></el-icon>
            </el-button>
          </el-badge>
        </div>
      </header>
      
      <div class="content-wrapper">
        <router-view />
      </div>
    </main>
    
    <el-drawer v-model="showNotifications" :title="$t('notifications.title')" size="400px">
      <div class="notification-list">
        <div 
          v-for="item in notifications" 
          :key="item.id" 
          class="notification-item"
          :class="{ unread: !item.read }"
          @click="markRead(item.id)"
        >
          <el-icon :class="item.type">
            <SuccessFilled v-if="item.type === 'success'" />
            <WarningFilled v-else-if="item.type === 'warning'" />
            <CircleCloseFilled v-else-if="item.type === 'error'" />
            <InfoFilled v-else />
          </el-icon>
          <div class="notification-content">
            <div class="notification-title">{{ item.title }}</div>
            <div class="notification-message">{{ item.message }}</div>
            <div class="notification-time">{{ formatTime(item.timestamp) }}</div>
          </div>
        </div>
        <el-empty v-if="notifications.length === 0" :description="$t('notifications.empty')" />
      </div>
    </el-drawer>
    
    <HelpPanel v-model="showHelp" />
    <UserGuide v-if="showGuide" @finish="handleGuideFinish" @close="handleGuideClose" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAppStore } from '@/stores'
import HelpPanel from '@/components/HelpPanel.vue'
import UserGuide from '@/components/UserGuide.vue'
import LanguageSwitcher from '@/components/LanguageSwitcher.vue'
import {
  Tools, HomeFilled, SetUp, Collection, Service, DataLine,
  Connection, FolderOpened, Position, TrendCharts, Setting,
  Operation, Bell, SuccessFilled, WarningFilled, CircleCloseFilled, InfoFilled, Document, QuestionFilled
} from '@element-plus/icons-vue'

const { t } = useI18n()
const route = useRoute()
const appStore = useAppStore()

const isCollapsed = ref(false)
const showNotifications = ref(false)
const showHelp = ref(false)
const showGuide = ref(false)

function handleGuideFinish() {
  showGuide.value = false
}

function handleGuideClose() {
  showGuide.value = false
}

onMounted(() => {
  const guideCompleted = localStorage.getItem('visual-spider-guide-completed')
  if (guideCompleted !== 'true') {
    showGuide.value = true
  }
})

const currentRoute = computed(() => route.path)
const notifications = computed(() => appStore.notifications)
const unreadCount = computed(() => appStore.getUnreadCount())

const currentPageTitle = computed(() => {
  const titles: Record<string, string> = {
    '/': t('nav.dashboard'),
    '/powershell': t('nav.powershell'),
    '/registry': t('nav.registry'),
    '/services': t('nav.services'),
    '/processes': t('nav.processes'),
    '/network': t('nav.network'),
    '/disk': t('nav.disk'),
    '/quick-actions': t('nav.quickActions'),
    '/optimization': t('nav.optimization'),
    '/advanced': t('nav.advanced'),
    '/settings': t('nav.settings')
  }
  return titles[route.path] || t('nav.dashboard')
})

function toggleSidebar() {
  isCollapsed.value = !isCollapsed.value
}

function markRead(id: string) {
  appStore.markNotificationRead(id)
}

function formatTime(timestamp: string) {
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN')
}

onMounted(() => {
  appStore.initialize()
})
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  height: 100%;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}

.app-container {
  display: flex;
  height: 100%;
  background: #f0f2f5;
}

.sidebar {
  width: 220px;
  background: linear-gradient(180deg, #1e3a5f 0%, #0d1b2a 100%);
  display: flex;
  flex-direction: column;
  transition: width 0.3s ease;
  position: relative;
}

.is-collapsed .sidebar {
  width: 64px;
}

.sidebar-header {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  color: #fff;
  font-weight: 600;
  font-size: 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.logo-text {
  white-space: nowrap;
}

.sidebar-menu {
  flex: 1;
  border-right: none;
  background: transparent;
  overflow-y: auto;
}

.sidebar-menu .el-menu-item {
  color: rgba(255, 255, 255, 0.7);
  height: 48px;
  line-height: 48px;
  margin: 4px 8px;
  border-radius: 8px;
}

.sidebar-menu .el-menu-item:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
}

.sidebar-menu .el-menu-item.is-active {
  background: linear-gradient(90deg, #409eff 0%, #53a8ff 100%);
  color: #fff;
}

.sidebar-menu .el-menu-item .el-icon {
  font-size: 18px;
}

.sidebar-footer {
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  padding: 8px;
}

.sidebar-menu-bottom {
  background: transparent;
  border-right: none;
}

.sidebar-menu-bottom .el-menu-item {
  color: rgba(255, 255, 255, 0.7);
  height: 48px;
  line-height: 48px;
  border-radius: 8px;
}

.sidebar-menu-bottom .el-menu-item:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
}

.collapse-btn {
  width: 100%;
  color: rgba(255, 255, 255, 0.7);
  margin-top: 8px;
}

.collapse-btn:hover {
  color: #fff;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.main-header {
  height: 60px;
  background: #fff;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
}

.page-title {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.notification-badge {
  margin-left: 8px;
}

.content-wrapper {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

.notification-list {
  padding: 0;
}

.notification-item {
  display: flex;
  gap: 12px;
  padding: 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.notification-item:hover {
  background: #f5f7fa;
}

.notification-item.unread {
  background: #ecf5ff;
}

.notification-item .el-icon {
  font-size: 20px;
  margin-top: 2px;
}

.notification-item .el-icon.success { color: #67c23a; }
.notification-item .el-icon.warning { color: #e6a23c; }
.notification-item .el-icon.error { color: #f56c6c; }
.notification-item .el-icon.info { color: #409eff; }

.notification-content {
  flex: 1;
}

.notification-title {
  font-weight: 600;
  color: #303133;
  margin-bottom: 4px;
}

.notification-message {
  font-size: 13px;
  color: #606266;
  margin-bottom: 4px;
}

.notification-time {
  font-size: 12px;
  color: #909399;
}
</style>
