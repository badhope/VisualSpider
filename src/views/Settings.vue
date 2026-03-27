<template>
  <div class="settings">
    <el-row :gutter="20">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>{{ $t('settings.title') }}</span>
            </div>
          </template>
          
          <el-tabs v-model="activeTab">
            <el-tab-pane :label="$t('settings.general')" name="general">
              <el-form label-width="150px" style="max-width: 600px;">
                <el-form-item :label="$t('settings.theme')">
                  <el-select v-model="settings.general.theme" @change="updateSettings">
                    <el-option label="浅色" value="light" />
                    <el-option label="深色" value="dark" />
                    <el-option label="跟随系统" value="auto" />
                  </el-select>
                </el-form-item>
                
                <el-form-item :label="$t('settings.language')">
                  <el-select v-model="settings.general.language" @change="updateLanguage">
                    <el-option label="中文" value="zh-CN" />
                    <el-option label="English" value="en" />
                  </el-select>
                </el-form-item>
                
                <el-form-item :label="$t('settings.autoStart')">
                  <el-switch v-model="settings.general.autoStart" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item :label="$t('settings.minimizeToTray')">
                  <el-switch v-model="settings.general.minimizeToTray" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item :label="$t('settings.confirmDangerous')">
                  <el-switch v-model="settings.general.confirmDangerousActions" @change="updateSettings" />
                </el-form-item>
              </el-form>
            </el-tab-pane>
            
            <el-tab-pane :label="$t('settings.powershell')" name="powershell">
              <el-form label-width="150px" style="max-width: 600px;">
                <el-form-item :label="$t('settings.defaultTimeout')">
                  <el-input-number 
                    v-model="settings.powershell.defaultTimeout" 
                    :min="1000" 
                    :max="300000"
                    :step="1000"
                    @change="updateSettings"
                  />
                  <span style="margin-left: 8px; color: #909399;">毫秒</span>
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
              </el-form>
            </el-tab-pane>
            
            <el-tab-pane :label="$t('settings.registry')" name="registry">
              <el-form label-width="150px" style="max-width: 600px;">
                <el-form-item :label="$t('settings.showHiddenKeys')">
                  <el-switch v-model="settings.registry.showHiddenKeys" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item :label="$t('settings.confirmDeletes')">
                  <el-switch v-model="settings.registry.confirmDeletes" @change="updateSettings" />
                </el-form-item>
                
                <el-form-item :label="$t('settings.autoRefresh')">
                  <el-switch v-model="settings.registry.autoRefresh" @change="updateSettings" />
                </el-form-item>
              </el-form>
            </el-tab-pane>
          </el-tabs>
          
          <el-divider />
          
          <div class="settings-footer">
            <el-button type="danger" @click="resetSettings">
              {{ $t('settings.reset') }}
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="20" style="margin-top: 20px;">
      <el-col :span="24">
        <el-card>
          <template #header>
            <span>关于</span>
          </template>
          <el-descriptions :column="2" border>
            <el-descriptions-item label="应用名称">Windows工具箱</el-descriptions-item>
            <el-descriptions-item label="版本">1.0.0</el-descriptions-item>
            <el-descriptions-item label="技术栈">Tauri + Vue 3 + TypeScript</el-descriptions-item>
            <el-descriptions-item label="作者">WindowsToolbox Team</el-descriptions-item>
          </el-descriptions>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAppStore } from '@/stores'
import { setLocale } from '@/locales'
import { ElMessage, ElMessageBox } from 'element-plus'

const appStore = useAppStore()
const activeTab = ref('general')

const settings = computed(() => appStore.settings)

function updateSettings() {
  appStore.updateSettings({})
}

function updateLanguage(locale: 'zh-CN' | 'en') {
  setLocale(locale)
  appStore.updateSettings({
    general: { ...settings.value.general, language: locale }
  })
  ElMessage.success('语言已更改')
}

async function resetSettings() {
  await ElMessageBox.confirm('确定要恢复默认设置吗？', '确认', { type: 'warning' })
  appStore.resetSettings()
  ElMessage.success('设置已恢复默认')
}
</script>

<style scoped>
.settings {
  padding: 0;
}

.card-header {
  display: flex;
  align-items: center;
}

.settings-footer {
  text-align: right;
}
</style>
