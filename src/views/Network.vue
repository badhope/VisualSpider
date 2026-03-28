<template>
  <div class="network">
    <el-row :gutter="20">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>{{ $t('network.title') }}</span>
            </div>
          </template>
          
          <div class="quick-actions">
            <el-button type="primary" @click="flushDns">
              <el-icon><Refresh /></el-icon>
              {{ $t('network.flushDns') }}
            </el-button>
            <el-button @click="releaseIp">
              <el-icon><SwitchButton /></el-icon>
              {{ $t('network.releaseIp') }}
            </el-button>
            <el-button @click="renewIp">
              <el-icon><Connection /></el-icon>
              {{ $t('network.renewIp') }}
            </el-button>
            <el-button type="warning" @click="resetNetwork">
              <el-icon><Setting /></el-icon>
              {{ $t('network.resetNetwork') }}
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="20" style="margin-top: 20px;">
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>{{ $t('network.connections') }}</span>
              <el-button text @click="loadConnections">
                <el-icon><Refresh /></el-icon>
              </el-button>
            </div>
          </template>
          <el-table :data="connections" v-loading="loadingConnections" max-height="400" size="small">
            <el-table-column prop="protocol" :label="$t('network.protocol')" width="80" />
            <el-table-column prop="localAddress" :label="$t('network.localAddress')" width="150" />
            <el-table-column prop="localPort" :label="$t('network.localPort')" width="80" />
            <el-table-column prop="remoteAddress" :label="$t('network.remoteAddress')" width="150" />
            <el-table-column prop="state" :label="$t('network.state')" show-overflow-tooltip />
          </el-table>
        </el-card>
      </el-col>
      
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>{{ $t('network.ports') }}</span>
              <el-button text @click="loadPorts">
                <el-icon><Refresh /></el-icon>
              </el-button>
            </div>
          </template>
          <el-table :data="ports" v-loading="loadingPorts" max-height="400" size="small">
            <el-table-column prop="port" :label="$t('network.port')" width="80" />
            <el-table-column prop="processName" :label="$t('network.processName')" width="150" />
            <el-table-column prop="pid" label="PID" width="80" />
            <el-table-column prop="protocol" :label="$t('network.protocol')" />
          </el-table>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="20" style="margin-top: 20px;">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>{{ $t('network.dns') }}</span>
            </div>
          </template>
          <div class="dns-info">
            <el-descriptions :column="2" border>
              <el-descriptions-item :label="$t('network.dnsServer')">
                <div v-for="(dns, index) in dnsServers" :key="index">{{ dns }}</div>
              </el-descriptions-item>
            </el-descriptions>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import { Refresh, SwitchButton, Connection, Setting } from '@element-plus/icons-vue'
import { useI18n } from 'vue-i18n'
import type { NetworkConnection } from '@/types'

const { t } = useI18n()
const loadingConnections = ref(false)
const loadingPorts = ref(false)
const connections = ref<NetworkConnection[]>([])
const ports = ref<Array<{ port: number; processName: string; pid: number; protocol: string }>>([])
const dnsServers = ref<string[]>([])

async function loadConnections() {
  loadingConnections.value = true
  try {
    const result = await invoke<NetworkConnection[]>('get_network_connections')
    connections.value = result
  } catch (error) {
    ElMessage.error(t('network.loadConnectionsFailed') + `: ${error}`)
  } finally {
    loadingConnections.value = false
  }
}

async function loadPorts() {
  loadingPorts.value = true
  try {
    const result = await invoke('get_port_usage') as Array<{ port: number; processName: string; pid: number; protocol: string }>
    ports.value = result
  } catch (error) {
    ElMessage.error(t('network.loadPortsFailed') + `: ${error}`)
  } finally {
    loadingPorts.value = false
  }
}

async function loadDnsServers() {
  try {
    const result = await invoke<string[]>('get_dns_servers')
    dnsServers.value = result
  } catch (error) {
    console.error(t('network.loadDnsFailed') + ':', error)
  }
}

async function flushDns() {
  try {
    await invoke('flush_dns')
    ElMessage.success(t('network.flushDnsSuccess'))
  } catch (error) {
    ElMessage.error(t('network.flushDnsFailed') + `: ${error}`)
  }
}

async function releaseIp() {
  try {
    await invoke('release_ip')
    ElMessage.success(t('network.releaseIpSuccess'))
  } catch (error) {
    ElMessage.error(t('network.releaseIpFailed') + `: ${error}`)
  }
}

async function renewIp() {
  try {
    await invoke('renew_ip')
    ElMessage.success(t('network.renewIpSuccess'))
  } catch (error) {
    ElMessage.error(t('network.renewIpFailed') + `: ${error}`)
  }
}

async function resetNetwork() {
  try {
    await invoke('reset_network')
    ElMessage.success(t('network.resetNetworkSuccess'))
  } catch (error) {
    ElMessage.error(t('network.resetNetworkFailed') + `: ${error}`)
  }
}

onMounted(() => {
  loadConnections()
  loadPorts()
  loadDnsServers()
})
</script>

<style scoped>
.network {
  padding: 0;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.quick-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.dns-info {
  max-height: 200px;
  overflow-y: auto;
}
</style>
