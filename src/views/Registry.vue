<template>
  <div class="registry">
    <el-row :gutter="20">
      <el-col :span="8">
        <el-card class="tree-card">
          <template #header>
            <div class="card-header">
              <span>{{ $t('registry.title') }}</span>
              <div class="header-actions">
                <el-button text @click="refreshTree">
                  <el-icon><Refresh /></el-icon>
                </el-button>
              </div>
            </div>
          </template>
          <el-input
            v-model="searchText"
            :placeholder="$t('registry.searchPlaceholder')"
            clearable
            style="margin-bottom: 12px;"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
          <el-tree
            ref="treeRef"
            :data="registryTree"
            :props="treeProps"
            node-key="path"
            highlight-current
            :expand-on-click-node="false"
            @node-click="handleNodeClick"
            v-loading="loading"
          >
            <template #default="{ node, data }">
              <span class="tree-node">
                <el-icon v-if="data.isRoot"><Folder /></el-icon>
                <el-icon v-else><FolderOpened /></el-icon>
                <span>{{ node.label }}</span>
              </span>
            </template>
          </el-tree>
        </el-card>
      </el-col>
      
      <el-col :span="16">
        <el-card class="values-card">
          <template #header>
            <div class="card-header">
              <span>{{ currentPath || '请选择注册表项' }}</span>
              <div class="header-actions">
                <el-button type="primary" @click="createValue">
                  <el-icon><Plus /></el-icon>
                  {{ $t('registry.newValue') }}
                </el-button>
                <el-button @click="exportKey">
                  <el-icon><Download /></el-icon>
                  {{ $t('registry.export') }}
                </el-button>
              </div>
            </div>
          </template>
          
          <el-table :data="registryValues" v-loading="loadingValues" border>
            <el-table-column prop="name" :label="$t('registry.name')" width="200" />
            <el-table-column prop="type" :label="$t('registry.type')" width="150" />
            <el-table-column prop="value" :label="$t('registry.value')">
              <template #default="{ row }">
                <span v-if="row.type === 'REG_DWORD' || row.type === 'REG_QWORD'">
                  {{ row.value }} (0x{{ Number(row.value).toString(16).toUpperCase() }})
                </span>
                <span v-else>{{ row.value }}</span>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="150">
              <template #default="{ row }">
                <el-button text size="small" @click="editValue(row)">{{ $t('common.edit') }}</el-button>
                <el-button text size="small" type="danger" @click="deleteValue(row)">{{ $t('common.delete') }}</el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
    </el-row>
    
    <el-dialog v-model="editDialogVisible" :title="editingValue ? '编辑值' : '新建值'" width="500px">
      <el-form :model="valueForm" label-width="100px">
        <el-form-item label="名称">
          <el-input v-model="valueForm.name" :disabled="!!editingValue" />
        </el-form-item>
        <el-form-item label="类型">
          <el-select v-model="valueForm.type" style="width: 100%;">
            <el-option label="字符串值 (REG_SZ)" value="REG_SZ" />
            <el-option label="可扩展字符串 (REG_EXPAND_SZ)" value="REG_EXPAND_SZ" />
            <el-option label="多字符串 (REG_MULTI_SZ)" value="REG_MULTI_SZ" />
            <el-option label="DWORD (32位)" value="REG_DWORD" />
            <el-option label="QWORD (64位)" value="REG_QWORD" />
            <el-option label="二进制值 (REG_BINARY)" value="REG_BINARY" />
          </el-select>
        </el-form-item>
        <el-form-item label="值">
          <el-input v-model="valueForm.value" type="textarea" :rows="3" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="saveValue">{{ $t('common.save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Refresh, Search, Folder, FolderOpened, Plus, Download } from '@element-plus/icons-vue'
import { useAppStore } from '@/stores'
import type { RegistryKey } from '@/types'

const appStore = useAppStore()
const treeRef = ref()
const loading = ref(false)
const loadingValues = ref(false)
const searchText = ref('')
const currentPath = ref('')
const registryValues = ref<RegistryKey[]>([])
const editDialogVisible = ref(false)
const editingValue = ref<RegistryKey | null>(null)

const valueForm = ref({
  name: '',
  type: 'REG_SZ',
  value: ''
})

const treeProps = {
  children: 'children',
  label: 'name'
}

const registryTree = ref([
  { name: 'HKEY_CLASSES_ROOT', path: 'HKEY_CLASSES_ROOT', isRoot: true, children: [] },
  { name: 'HKEY_CURRENT_USER', path: 'HKEY_CURRENT_USER', isRoot: true, children: [] },
  { name: 'HKEY_LOCAL_MACHINE', path: 'HKEY_LOCAL_MACHINE', isRoot: true, children: [] },
  { name: 'HKEY_USERS', path: 'HKEY_USERS', isRoot: true, children: [] },
  { name: 'HKEY_CURRENT_CONFIG', path: 'HKEY_CURRENT_CONFIG', isRoot: true, children: [] }
])

async function handleNodeClick(data: any) {
  currentPath.value = data.path
  await loadValues(data.path)
}

async function loadValues(path: string) {
  loadingValues.value = true
  try {
    const result = await invoke<RegistryKey[]>('get_registry_values', { path })
    registryValues.value = result
  } catch (error) {
    ElMessage.error(`加载注册表值失败: ${error}`)
  } finally {
    loadingValues.value = false
  }
}

async function refreshTree() {
  loading.value = true
  try {
    const result = await invoke('get_registry_tree')
    registryTree.value = result
    ElMessage.success('刷新成功')
  } catch (error) {
    ElMessage.error(`刷新失败: ${error}`)
  } finally {
    loading.value = false
  }
}

function createValue() {
  if (!currentPath.value) {
    ElMessage.warning('请先选择一个注册表项')
    return
  }
  editingValue.value = null
  valueForm.value = { name: '', type: 'REG_SZ', value: '' }
  editDialogVisible.value = true
}

function editValue(row: RegistryKey) {
  editingValue.value = row
  valueForm.value = {
    name: row.name,
    type: row.type,
    value: String(row.value || '')
  }
  editDialogVisible.value = true
}

async function deleteValue(row: RegistryKey) {
  if (appStore.settings.registry.confirmDeletes) {
    await ElMessageBox.confirm($t('registry.confirmDelete'), $t('common.warning'), {
      confirmButtonText: $t('common.confirm'),
      cancelButtonText: $t('common.cancel'),
      type: 'warning'
    })
  }
  
  try {
    await invoke('delete_registry_value', {
      path: currentPath.value,
      name: row.name
    })
    ElMessage.success('删除成功')
    await loadValues(currentPath.value)
  } catch (error) {
    ElMessage.error(`删除失败: ${error}`)
  }
}

async function saveValue() {
  try {
    if (editingValue.value) {
      await invoke('set_registry_value', {
        path: currentPath.value,
        name: valueForm.value.name,
        valueType: valueForm.value.type,
        value: valueForm.value.value
      })
    } else {
      await invoke('create_registry_value', {
        path: currentPath.value,
        name: valueForm.value.name,
        valueType: valueForm.value.type,
        value: valueForm.value.value
      })
    }
    ElMessage.success('保存成功')
    editDialogVisible.value = false
    await loadValues(currentPath.value)
  } catch (error) {
    ElMessage.error(`保存失败: ${error}`)
  }
}

async function exportKey() {
  if (!currentPath.value) {
    ElMessage.warning('请先选择一个注册表项')
    return
  }
  try {
    await invoke('export_registry_key', { path: currentPath.value })
    ElMessage.success('导出成功')
  } catch (error) {
    ElMessage.error(`导出失败: ${error}`)
  }
}

onMounted(() => {
  refreshTree()
})
</script>

<style scoped>
.registry {
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

.tree-card {
  height: calc(100vh - 160px);
  overflow-y: auto;
}

.tree-node {
  display: flex;
  align-items: center;
  gap: 6px;
}

.values-card {
  min-height: calc(100vh - 160px);
}
</style>
