<template>
  <div class="advanced">
    <el-row :gutter="20">
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>{{ $t('advanced.envVariables') }}</span>
              <div class="header-actions">
                <el-button type="primary" size="small" @click="showEnvDialog = true">
                  <el-icon><Plus /></el-icon>
                  添加
                </el-button>
                <el-button size="small" @click="loadEnvVariables">
                  <el-icon><Refresh /></el-icon>
                </el-button>
              </div>
            </div>
          </template>
          <el-table :data="envVariables" v-loading="loadingEnv" size="small" max-height="400">
            <el-table-column prop="name" label="变量名" width="200" show-overflow-tooltip />
            <el-table-column prop="value" label="值" show-overflow-tooltip />
            <el-table-column label="操作" width="120">
              <template #default="{ row }">
                <el-button text size="small" @click="editEnv(row)">编辑</el-button>
                <el-button text size="small" type="danger" @click="deleteEnv(row)">删除</el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
      
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>{{ $t('advanced.hostsEditor') }}</span>
              <div class="header-actions">
                <el-button type="primary" size="small" @click="showHostsDialog = true">
                  <el-icon><Plus /></el-icon>
                  添加
                </el-button>
                <el-button size="small" @click="loadHosts">
                  <el-icon><Refresh /></el-icon>
                </el-button>
              </div>
            </div>
          </template>
          <el-table :data="hostsEntries" v-loading="loadingHosts" size="small" max-height="400">
            <el-table-column prop="ip" label="IP地址" width="150" />
            <el-table-column prop="hostname" label="主机名" show-overflow-tooltip />
            <el-table-column label="操作" width="120">
              <template #default="{ row }">
                <el-button text size="small" @click="editHost(row)">编辑</el-button>
                <el-button text size="small" type="danger" @click="deleteHost(row)">删除</el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="20" style="margin-top: 20px;">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>系统工具</span>
            </div>
          </template>
          <div class="system-tools">
            <el-button type="primary" @click="runSfc">
              <el-icon><DocumentChecked /></el-icon>
              {{ $t('advanced.sfcScan') }}
            </el-button>
            <el-button type="success" @click="runDism">
              <el-icon><Tools /></el-icon>
              {{ $t('advanced.dism') }}
            </el-button>
            <el-button type="warning" @click="checkWindowsUpdate">
              <el-icon><Upload /></el-icon>
              检查更新
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>
    
    <el-dialog v-model="showEnvDialog" title="编辑环境变量" width="500px">
      <el-form :model="envForm" label-width="100px">
        <el-form-item label="变量名">
          <el-input v-model="envForm.name" />
        </el-form-item>
        <el-form-item label="变量值">
          <el-input v-model="envForm.value" type="textarea" :rows="3" />
        </el-form-item>
        <el-form-item label="作用域">
          <el-radio-group v-model="envForm.scope">
            <el-radio label="user">用户</el-radio>
            <el-radio label="system">系统</el-radio>
          </el-radio-group>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showEnvDialog = false">取消</el-button>
        <el-button type="primary" @click="saveEnv">保存</el-button>
      </template>
    </el-dialog>
    
    <el-dialog v-model="showHostsDialog" title="编辑Hosts条目" width="500px">
      <el-form :model="hostsForm" label-width="100px">
        <el-form-item label="IP地址">
          <el-input v-model="hostsForm.ip" placeholder="例如: 127.0.0.1" />
        </el-form-item>
        <el-form-item label="主机名">
          <el-input v-model="hostsForm.hostname" placeholder="例如: localhost" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showHostsDialog = false">取消</el-button>
        <el-button type="primary" @click="saveHost">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Refresh, DocumentChecked, Tools, Upload } from '@element-plus/icons-vue'
import { useAppStore } from '@/stores'

const appStore = useAppStore()
const loadingEnv = ref(false)
const loadingHosts = ref(false)
const showEnvDialog = ref(false)
const showHostsDialog = ref(false)

const envVariables = ref<Array<{ name: string; value: string; scope: string }>>([])
const hostsEntries = ref<Array<{ ip: string; hostname: string }>>([])

const envForm = ref({ name: '', value: '', scope: 'user' })
const hostsForm = ref({ ip: '', hostname: '' })

async function loadEnvVariables() {
  loadingEnv.value = true
  try {
    const result = await invoke('get_env_variables')
    envVariables.value = result
  } catch (error) {
    ElMessage.error(`加载环境变量失败: ${error}`)
  } finally {
    loadingEnv.value = false
  }
}

async function loadHosts() {
  loadingHosts.value = true
  try {
    const result = await invoke('get_hosts_entries')
    hostsEntries.value = result
  } catch (error) {
    ElMessage.error(`加载Hosts失败: ${error}`)
  } finally {
    loadingHosts.value = false
  }
}

function editEnv(row: any) {
  envForm.value = { name: row.name, value: row.value, scope: row.scope || 'user' }
  showEnvDialog.value = true
}

async function deleteEnv(row: any) {
  await ElMessageBox.confirm(`确定删除环境变量 ${row.name}？`, '确认', { type: 'warning' })
  try {
    await invoke('delete_env_variable', { name: row.name, scope: row.scope || 'user' })
    ElMessage.success('删除成功')
    await loadEnvVariables()
  } catch (error) {
    ElMessage.error(`删除失败: ${error}`)
  }
}

async function saveEnv() {
  try {
    await invoke('set_env_variable', envForm.value)
    ElMessage.success('保存成功')
    showEnvDialog.value = false
    await loadEnvVariables()
  } catch (error) {
    ElMessage.error(`保存失败: ${error}`)
  }
}

function editHost(row: any) {
  hostsForm.value = { ip: row.ip, hostname: row.hostname }
  showHostsDialog.value = true
}

async function deleteHost(row: any) {
  await ElMessageBox.confirm(`确定删除 ${row.ip} ${row.hostname}？`, '确认', { type: 'warning' })
  try {
    await invoke('delete_hosts_entry', { ip: row.ip, hostname: row.hostname })
    ElMessage.success('删除成功')
    await loadHosts()
  } catch (error) {
    ElMessage.error(`删除失败: ${error}`)
  }
}

async function saveHost() {
  try {
    await invoke('add_hosts_entry', hostsForm.value)
    ElMessage.success('保存成功')
    showHostsDialog.value = false
    await loadHosts()
  } catch (error) {
    ElMessage.error(`保存失败: ${error}`)
  }
}

async function runSfc() {
  try {
    ElMessage.info('正在运行系统文件检查，请稍候...')
    await invoke('run_sfc_scan')
    ElMessage.success('系统文件检查完成')
  } catch (error) {
    ElMessage.error(`运行失败: ${error}`)
  }
}

async function runDism() {
  try {
    ElMessage.info('正在运行DISM，请稍候...')
    await invoke('run_dism')
    ElMessage.success('DISM运行完成')
  } catch (error) {
    ElMessage.error(`运行失败: ${error}`)
  }
}

async function checkWindowsUpdate() {
  try {
    await invoke('check_windows_update')
    ElMessage.success('已打开Windows更新')
  } catch (error) {
    ElMessage.error(`操作失败: ${error}`)
  }
}

onMounted(() => {
  loadEnvVariables()
  loadHosts()
})
</script>

<style scoped>
.advanced {
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

.system-tools {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}
</style>
