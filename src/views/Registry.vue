<template>
  <div class="registry">
    <el-row :gutter="20">
      <el-col :span="8">
        <el-card class="tree-card">
          <template #header>
            <div class="card-header">
              <span>{{ $t('registry.title') }}</span>
              <div class="header-actions">
                <el-button text @click="showFavoritesDialog = true" :title="$t('registry.addToFavorites')">
                  <el-icon><Star /></el-icon>
                </el-button>
                <el-button text @click="refreshTree" :title="$t('registry.refresh')">
                  <el-icon><Refresh /></el-icon>
                </el-button>
              </div>
            </div>
          </template>
          
          <el-input
            v-model="jumpPath"
            :placeholder="$t('registry.jumpPathPlaceholder')"
            clearable
            @keyup.enter="jumpToPath"
            style="margin-bottom: 12px;"
          >
            <template #append>
              <el-button @click="jumpToPath">{{ $t('registry.jumpToPath') }}</el-button>
            </template>
          </el-input>
          
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
          
          <el-tabs v-model="activeTab" size="small">
            <el-tab-pane :label="$t('registry.registryTree')" name="tree">
              <el-tree
                ref="treeRef"
                :data="filteredTree"
                :props="treeProps"
                node-key="path"
                highlight-current
                :expand-on-click-node="false"
                :load="loadNode"
                lazy
                @node-click="handleNodeClick"
                v-loading="loading"
              >
                <template #default="{ node, data }">
                  <span class="tree-node">
                    <el-icon v-if="data.isRoot" style="color: #409EFF;"><Folder /></el-icon>
                    <el-icon v-else style="color: #67C23A;"><FolderOpened /></el-icon>
                    <span class="node-label">{{ node.label }}</span>
                    <el-button 
                      v-if="!data.isRoot"
                      text 
                      size="small" 
                      class="fav-btn"
                      @click.stop="toggleFavorite(data)"
                    >
                      <el-icon :class="{ 'is-favorite': isFavorite(data.path) }"><Star /></el-icon>
                    </el-button>
                  </span>
                </template>
              </el-tree>
            </el-tab-pane>
            
            <el-tab-pane :label="$t('registry.favorites')" name="favorites">
              <div v-if="favoritePaths.length === 0" class="empty-favorites">
                <el-icon size="32"><Star /></el-icon>
                <p>{{ $t('registry.noFavoritesPath') }}</p>
                <p class="tip">{{ $t('registry.clickStarToAddPath') }}</p>
              </div>
              <div v-else class="favorites-list">
                <div 
                  v-for="fav in favoritePaths" 
                  :key="fav.path"
                  class="favorite-item"
                  @click="jumpToFavorite(fav.path)"
                >
                  <div class="fav-info">
                    <span class="fav-name">{{ fav.name }}</span>
                    <span class="fav-path">{{ fav.path }}</span>
                  </div>
                  <el-button 
                    text 
                    size="small" 
                    type="danger"
                    @click.stop="removeFavorite(fav.path)"
                  >
                    <el-icon><Delete /></el-icon>
                  </el-button>
                </div>
              </div>
            </el-tab-pane>
          </el-tabs>
        </el-card>
      </el-col>
      
      <el-col :span="16">
        <el-card class="values-card">
          <template #header>
            <div class="card-header">
              <div class="path-breadcrumb">
                <span v-if="currentPath" class="current-path">
                  <el-tag v-for="(part, index) in pathParts" :key="index" size="small" class="path-tag">
                    {{ part }}
                  </el-tag>
                </span>
                <span v-else class="placeholder">{{ $t('registry.selectKey') }}</span>
              </div>
              <div class="header-actions">
                <el-button type="primary" @click="createValue" :disabled="!currentPath">
                  <el-icon><Plus /></el-icon>
                  {{ $t('registry.newValue') }}
                </el-button>
                <el-button @click="exportKey" :disabled="!currentPath">
                  <el-icon><Download /></el-icon>
                  {{ $t('registry.export') }}
                </el-button>
                <el-button @click="copyPath" :disabled="!currentPath">
                  <el-icon><CopyDocument /></el-icon>
                  {{ $t('registry.copyPath') }}
                </el-button>
              </div>
            </div>
          </template>
          
          <el-empty v-if="!currentPath" :description="$t('registry.selectKey')" />
          
          <el-table 
            v-else
            :data="filteredValues" 
            v-loading="loadingValues" 
            border
            :default-sort="{ prop: 'name', order: 'ascending' }"
          >
            <el-table-column prop="name" :label="$t('registry.name')" width="200" sortable>
              <template #default="{ row }">
                <span :class="{ 'default-value': row.name === '(默认)' }">
                  {{ row.name }}
                </span>
              </template>
            </el-table-column>
            <el-table-column prop="value_type" :label="$t('registry.type')" width="150" sortable>
              <template #default="{ row }">
                <el-tag size="small" :type="getTypeTagType(row.value_type)">
                  {{ row.value_type }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="value" :label="$t('registry.value')">
              <template #default="{ row }">
                <div class="value-cell">
                  <span v-if="row.value_type === 'REG_DWORD' || row.value_type === 'REG_QWORD'" class="hex-value">
                    {{ row.value }}
                    <span class="hex-display">(0x{{ formatHex(row.value) }})</span>
                  </span>
                  <span v-else-if="row.value_type === 'REG_BINARY'" class="binary-value">
                    {{ formatBinary(row.value) }}
                  </span>
                  <span v-else>{{ row.value || $t('registry.emptyValue') }}</span>
                </div>
              </template>
            </el-table-column>
            <el-table-column :label="$t('common.edit')" width="150" fixed="right">
              <template #default="{ row }">
                <el-button text size="small" @click="editValue(row)">{{ $t('common.edit') }}</el-button>
                <el-button text size="small" type="danger" @click="deleteValue(row)">{{ $t('common.delete') }}</el-button>
              </template>
            </el-table-column>
          </el-table>
          
          <div v-if="registryValues.length > 0" class="table-footer">
            <span>{{ $t('registry.totalValues', { count: registryValues.length }) }}</span>
          </div>
        </el-card>
      </el-col>
    </el-row>
    
    <el-dialog v-model="editDialogVisible" :title="editingValue ? $t('registry.editValue') : $t('registry.createValue')" width="500px">
      <el-form :model="valueForm" label-width="100px">
        <el-form-item :label="$t('registry.name')">
          <el-input 
            v-model="valueForm.name" 
            :disabled="!!editingValue && editingValue.name !== '(默认)'"
            :placeholder="editingValue ? '' : $t('registry.defaultValue')"
          />
        </el-form-item>
        <el-form-item :label="$t('registry.type')">
          <el-select v-model="valueForm.type" style="width: 100%;" :disabled="!!editingValue">
            <el-option :label="$t('registry.stringValue') + ' (REG_SZ)'" value="REG_SZ" />
            <el-option :label="$t('registry.expandStringValue') + ' (REG_EXPAND_SZ)'" value="REG_EXPAND_SZ" />
            <el-option label="DWORD (32-bit)" value="REG_DWORD" />
            <el-option label="QWORD (64-bit)" value="REG_QWORD" />
            <el-option :label="$t('registry.binaryValue') + ' (REG_BINARY)'" value="REG_BINARY" />
            <el-option :label="$t('registry.multiStringValue') + ' (REG_MULTI_SZ)'" value="REG_MULTI_SZ" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('registry.value')">
          <el-input 
            v-if="valueForm.type === 'REG_SZ' || valueForm.type === 'REG_EXPAND_SZ'"
            v-model="valueForm.value" 
            :placeholder="$t('registry.stringValue')"
          />
          <el-input-number
            v-else-if="valueForm.type === 'REG_DWORD'"
            v-model="numericValue"
            :min="0"
            :max="4294967295"
            style="width: 100%;"
          />
          <el-input-number
            v-else-if="valueForm.type === 'REG_QWORD'"
            v-model="numericValue"
            :min="0"
            style="width: 100%;"
          />
          <el-input 
            v-else
            v-model="valueForm.value" 
            type="textarea" 
            :rows="3"
            :placeholder="$t('registry.value')"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="saveValue" :loading="saving">{{ $t('common.save') }}</el-button>
      </template>
    </el-dialog>
    
    <el-dialog v-model="showFavoritesDialog" :title="$t('registry.favorites')" width="500px">
      <div v-if="favoritePaths.length === 0" class="empty-favorites">
        <p>{{ $t('registry.noFavoritesPath') }}</p>
      </div>
      <div v-else class="favorites-manage">
        <div v-for="fav in favoritePaths" :key="fav.path" class="fav-manage-item">
          <div class="fav-info">
            <el-input v-model="fav.name" size="small" :placeholder="$t('registry.name')" style="width: 120px;" />
            <span class="fav-path">{{ fav.path }}</span>
          </div>
          <el-button text size="small" type="danger" @click="removeFavorite(fav.path)">
            <el-icon><Delete /></el-icon>
          </el-button>
        </div>
      </div>
      <template #footer>
        <el-button @click="showFavoritesDialog = false">{{ $t('common.close') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Refresh, Search, Folder, FolderOpened, Plus, Download, Star, Delete, CopyDocument } from '@element-plus/icons-vue'
import type { RegistryValue } from '@/types'

interface TreeNode {
  name: string
  path: string
  isRoot?: boolean
  children?: TreeNode[]
}

interface FavoritePath {
  name: string
  path: string
}

const treeRef = ref()
const loading = ref(false)
const loadingValues = ref(false)
const saving = ref(false)
const searchText = ref('')
const jumpPath = ref('')
const currentPath = ref('')
const registryValues = ref<RegistryValue[]>([])
const editDialogVisible = ref(false)
const showFavoritesDialog = ref(false)
const editingValue = ref<RegistryValue | null>(null)
const activeTab = ref('tree')

const favoritePaths = ref<FavoritePath[]>([])

const valueForm = ref({
  name: '',
  type: 'REG_SZ',
  value: ''
})

const treeProps = {
  children: 'children',
  label: 'name',
  isLeaf: (data: TreeNode) => !data.children || data.children.length === 0
}

const registryTree = ref<TreeNode[]>([
  { name: 'HKEY_CLASSES_ROOT', path: 'HKEY_CLASSES_ROOT', isRoot: true },
  { name: 'HKEY_CURRENT_USER', path: 'HKEY_CURRENT_USER', isRoot: true },
  { name: 'HKEY_LOCAL_MACHINE', path: 'HKEY_LOCAL_MACHINE', isRoot: true },
  { name: 'HKEY_USERS', path: 'HKEY_USERS', isRoot: true },
  { name: 'HKEY_CURRENT_CONFIG', path: 'HKEY_CURRENT_CONFIG', isRoot: true }
])

const pathParts = computed(() => {
  if (!currentPath.value) return []
  return currentPath.value.split('\\')
})

const filteredValues = computed(() => {
  if (!searchText.value) return registryValues.value
  const keyword = searchText.value.toLowerCase()
  return registryValues.value.filter(v => 
    v.name.toLowerCase().includes(keyword) || 
    String(v.value).toLowerCase().includes(keyword)
  )
})

const filteredTree = computed(() => registryTree.value)

const numericValue = computed({
  get: () => parseInt(valueForm.value.value) || 0,
  set: (val) => { valueForm.value.value = String(val) }
})

async function loadNode(node: { level: number; data: TreeNode }, resolve: (nodes: TreeNode[]) => void) {
  if (node.level === 0) {
    resolve(registryTree.value)
    return
  }
  
  try {
    const result = await invoke<TreeNode[]>('get_registry_subkeys', { 
      path: node.data.path 
    })
    resolve(result || [])
  } catch {
    resolve([])
  }
}

async function handleNodeClick(data: TreeNode) {
  currentPath.value = data.path
  await loadValues(data.path)
}

async function loadValues(path: string) {
  loadingValues.value = true
  try {
    const result = await invoke<RegistryValue[]>('get_registry_values', { path })
    registryValues.value = result || []
  } catch (error) {
    ElMessage.error(`加载注册表值失败: ${error}`)
    registryValues.value = []
  } finally {
    loadingValues.value = false
  }
}

async function refreshTree() {
  loading.value = true
  try {
    registryTree.value = [
      { name: 'HKEY_CLASSES_ROOT', path: 'HKEY_CLASSES_ROOT', isRoot: true },
      { name: 'HKEY_CURRENT_USER', path: 'HKEY_CURRENT_USER', isRoot: true },
      { name: 'HKEY_LOCAL_MACHINE', path: 'HKEY_LOCAL_MACHINE', isRoot: true },
      { name: 'HKEY_USERS', path: 'HKEY_USERS', isRoot: true },
      { name: 'HKEY_CURRENT_CONFIG', path: 'HKEY_CURRENT_CONFIG', isRoot: true }
    ]
    ElMessage.success('刷新成功')
  } catch (error) {
    ElMessage.error(`刷新失败: ${error}`)
  } finally {
    loading.value = false
  }
}

function jumpToPath() {
  if (!jumpPath.value.trim()) return
  
  const path = jumpPath.value.trim().toUpperCase()
  const validRoots = ['HKEY_CLASSES_ROOT', 'HKEY_CURRENT_USER', 'HKEY_LOCAL_MACHINE', 'HKEY_USERS', 'HKEY_CURRENT_CONFIG']
  
  const rootKey = validRoots.find(r => path.startsWith(r))
  if (!rootKey) {
    ElMessage.error('无效的注册表路径')
    return
  }
  
  currentPath.value = jumpPath.value.trim()
  loadValues(currentPath.value)
}

function jumpToFavorite(path: string) {
  currentPath.value = path
  loadValues(path)
  activeTab.value = 'tree'
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

function editValue(row: RegistryValue) {
  editingValue.value = row
  valueForm.value = {
    name: row.name === '(默认)' ? '' : row.name,
    type: row.value_type,
    value: String(row.value || '')
  }
  editDialogVisible.value = true
}

async function deleteValue(row: RegistryValue) {
  const name = row.name === '(默认)' ? '默认值' : row.name
  
  try {
    await ElMessageBox.confirm(
      `确定要删除值 "${name}" 吗？此操作不可恢复。`,
      '确认删除',
      {
        confirmButtonText: '删除',
        cancelButtonText: '取消',
        type: 'warning',
        confirmButtonClass: 'el-button--danger'
      }
    )
    
    await invoke('delete_registry_value', {
      path: currentPath.value,
      name: row.name
    })
    
    ElMessage.success('删除成功')
    await loadValues(currentPath.value)
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error(`删除失败: ${error}`)
    }
  }
}

async function saveValue() {
  if (!valueForm.value.name && !editingValue.value) {
    valueForm.value.name = ''
  }
  
  saving.value = true
  try {
    const name = valueForm.value.name || ''
    
    if (editingValue.value) {
      await invoke('set_registry_value', {
        path: currentPath.value,
        name: editingValue.value.name === '(默认)' ? '' : name,
        value_type: valueForm.value.type,
        value: valueForm.value.value
      })
    } else {
      await invoke('create_registry_value', {
        path: currentPath.value,
        name: name,
        value_type: valueForm.value.type,
        value: valueForm.value.value
      })
    }
    
    ElMessage.success('保存成功')
    editDialogVisible.value = false
    await loadValues(currentPath.value)
  } catch (error) {
    ElMessage.error(`保存失败: ${error}`)
  } finally {
    saving.value = false
  }
}

async function exportKey() {
  if (!currentPath.value) {
    ElMessage.warning('请先选择一个注册表项')
    return
  }
  
  try {
    const result = await invoke<string>('export_registry_key', { path: currentPath.value })
    
    const blob = new Blob([result], { type: 'text/plain;charset=utf-8' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `${currentPath.value.replace(/\\/g, '_')}.reg`
    a.click()
    URL.revokeObjectURL(url)
    
    ElMessage.success('导出成功')
  } catch (error) {
    ElMessage.error(`导出失败: ${error}`)
  }
}

async function copyPath() {
  if (!currentPath.value) return
  
  try {
    await navigator.clipboard.writeText(currentPath.value)
    ElMessage.success('路径已复制到剪贴板')
  } catch {
    ElMessage.error('复制失败')
  }
}

function isFavorite(path: string) {
  return favoritePaths.value.some(f => f.path === path)
}

function toggleFavorite(data: TreeNode) {
  if (isFavorite(data.path)) {
    removeFavorite(data.path)
  } else {
    favoritePaths.value.push({
      name: data.name,
      path: data.path
    })
    saveFavorites()
    ElMessage.success('已添加到收藏')
  }
}

function removeFavorite(path: string) {
  favoritePaths.value = favoritePaths.value.filter(f => f.path !== path)
  saveFavorites()
  ElMessage.success('已从收藏中移除')
}

function saveFavorites() {
  localStorage.setItem('registry-favorites', JSON.stringify(favoritePaths.value))
}

function loadFavorites() {
  const saved = localStorage.getItem('registry-favorites')
  if (saved) {
    favoritePaths.value = JSON.parse(saved)
  }
}

function formatHex(value: string): string {
  const num = parseInt(value)
  return isNaN(num) ? '0' : num.toString(16).toUpperCase().padStart(8, '0')
}

function formatBinary(value: string): string {
  if (!value) return ''
  return value.substring(0, 50) + (value.length > 50 ? '...' : '')
}

function getTypeTagType(type: string): string {
  const typeMap: Record<string, string> = {
    'REG_SZ': '',
    'REG_EXPAND_SZ': 'success',
    'REG_DWORD': 'warning',
    'REG_QWORD': 'warning',
    'REG_BINARY': 'info',
    'REG_MULTI_SZ': 'info'
  }
  return typeMap[type] || ''
}

onMounted(() => {
  loadFavorites()
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
  width: 100%;
}

.node-label {
  flex: 1;
}

.fav-btn {
  opacity: 0;
  transition: opacity 0.2s;
}

.tree-node:hover .fav-btn {
  opacity: 1;
}

.is-favorite {
  color: #E6A23C;
}

.values-card {
  min-height: calc(100vh - 160px);
}

.path-breadcrumb {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 4px;
}

.current-path {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.path-tag {
  margin: 0;
}

.placeholder {
  color: #909399;
}

.value-cell {
  word-break: break-all;
}

.default-value {
  font-style: italic;
  color: #909399;
}

.hex-value .hex-display {
  color: #909399;
  font-size: 12px;
  margin-left: 8px;
}

.binary-value {
  font-family: monospace;
  font-size: 12px;
}

.table-footer {
  padding: 12px 0;
  text-align: right;
  color: #909399;
  font-size: 13px;
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

.favorites-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.favorite-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.favorite-item:hover {
  background: #e6e8eb;
}

.fav-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.fav-name {
  font-weight: 600;
  font-size: 14px;
}

.fav-path {
  font-size: 12px;
  color: #909399;
  word-break: break-all;
}

.favorites-manage {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.fav-manage-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 4px;
}

.fav-manage-item .fav-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

:deep(.el-tabs__content) {
  max-height: calc(100vh - 400px);
  overflow-y: auto;
}
</style>
