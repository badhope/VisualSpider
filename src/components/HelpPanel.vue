<template>
  <el-drawer
    v-model="visible"
    :title="$t('help.title')"
    direction="rtl"
    size="400px"
    :append-to-body="true"
  >
    <div class="help-content">
      <el-input
        v-model="searchText"
        :placeholder="$t('help.searchPlaceholder')"
        clearable
        style="margin-bottom: 16px;"
      >
        <template #prefix>
          <el-icon><Search /></el-icon>
        </template>
      </el-input>
      
      <el-collapse v-model="activeCollapse">
        <el-collapse-item :title="$t('help.gettingStarted')" name="getting-started">
          <div class="help-section">
            <h4>{{ $t('help.quickStart') }}</h4>
            <ol>
              <li>{{ $t('help.step1') }}</li>
              <li>{{ $t('help.step2') }}</li>
              <li>{{ $t('help.step3') }}</li>
              <li>{{ $t('help.step4') }}</li>
            </ol>
          </div>
        </el-collapse-item>
        
        <el-collapse-item :title="$t('help.modules')" name="modules">
          <div class="help-section">
            <div v-for="module in modules" :key="module.name" class="module-item">
              <h4>{{ module.name }}</h4>
              <p>{{ module.description }}</p>
            </div>
          </div>
        </el-collapse-item>
        
        <el-collapse-item :title="$t('help.shortcuts')" name="shortcuts">
          <div class="help-section">
            <div v-for="shortcut in shortcuts" :key="shortcut.key" class="shortcut-item">
              <el-tag size="small" type="info">{{ shortcut.key }}</el-tag>
              <span>{{ shortcut.description }}</span>
            </div>
          </div>
        </el-collapse-item>
        
        <el-collapse-item :title="$t('help.faq')" name="faq">
          <div class="help-section">
            <el-collapse accordion>
              <el-collapse-item
                v-for="(item, index) in faqItems"
                :key="index"
                :title="item.question"
                :name="index"
              >
                <p>{{ item.answer }}</p>
              </el-collapse-item>
            </el-collapse>
          </div>
        </el-collapse-item>
        
        <el-collapse-item :title="$t('help.tips')" name="tips">
          <div class="help-section">
            <el-alert
              v-for="(tip, index) in tips"
              :key="index"
              :title="tip"
              type="info"
              :closable="false"
              style="margin-bottom: 8px;"
            />
          </div>
        </el-collapse-item>
      </el-collapse>
    </div>
    
    <template #footer>
      <div class="help-footer">
        <el-button @click="visible = false">{{ $t('common.close') }}</el-button>
        <el-button type="primary" @click="openExternalHelp">
          {{ $t('help.viewOnline') }}
        </el-button>
      </div>
    </template>
  </el-drawer>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Search } from '@element-plus/icons-vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const visible = defineModel<boolean>({ default: false })
const searchText = ref('')
const activeCollapse = ref(['getting-started'])

const modules = computed(() => [
  { name: t('nav.dashboard'), description: t('help.dashboardDesc') },
  { name: t('nav.powershell'), description: t('help.powershellDesc') },
  { name: t('nav.registry'), description: t('help.registryDesc') },
  { name: t('nav.services'), description: t('help.servicesDesc') },
  { name: t('nav.processes'), description: t('help.processesDesc') },
  { name: t('nav.network'), description: t('help.networkDesc') },
  { name: t('nav.disk'), description: t('help.diskDesc') }
])

const shortcuts = computed(() => [
  { key: 'Ctrl + F', description: t('help.shortcutSearch') },
  { key: 'Ctrl + R', description: t('help.shortcutRefresh') },
  { key: 'Ctrl + S', description: t('help.shortcutSave') },
  { key: 'Esc', description: t('help.shortcutClose') },
  { key: 'F5', description: t('help.shortcutRefreshPage') }
])

const faqItems = computed(() => [
  { question: t('help.faq1Q'), answer: t('help.faq1A') },
  { question: t('help.faq2Q'), answer: t('help.faq2A') },
  { question: t('help.faq3Q'), answer: t('help.faq3A') },
  { question: t('help.faq4Q'), answer: t('help.faq4A') }
])

const tips = computed(() => [
  t('help.tip1'),
  t('help.tip2'),
  t('help.tip3'),
  t('help.tip4')
])

function openExternalHelp() {
  window.open('https://github.com/your-repo/visual-spider/wiki', '_blank')
}
</script>

<style scoped>
.help-content {
  padding: 0 4px;
}

.help-section {
  padding: 8px 0;
}

.help-section h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  color: #303133;
}

.help-section ol {
  margin: 0;
  padding-left: 20px;
}

.help-section ol li {
  margin-bottom: 8px;
  color: #606266;
}

.module-item {
  margin-bottom: 16px;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 4px;
}

.module-item h4 {
  margin: 0 0 8px 0;
  font-size: 14px;
}

.module-item p {
  margin: 0;
  font-size: 13px;
  color: #909399;
}

.shortcut-item {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.shortcut-item span {
  font-size: 13px;
  color: #606266;
}

.help-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style>
