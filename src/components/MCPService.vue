<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const isLoading = ref(false);
const serviceStatus = ref("未安装");
const installOutput = ref("");
const installError = ref("");

// 检查 MCP 服务状态
async function checkServiceStatus() {
  try {
    const status = await invoke("check_mcp_service_status");
    serviceStatus.value = status as string;
  } catch (error) {
    console.error("检查服务状态失败:", error);
    serviceStatus.value = "检查失败";
  }
}

// 安装 MCP 服务
async function installMCPService() {
  try {
    isLoading.value = true;
    installError.value = "";
    installOutput.value = $t('message.installing');
    
    const result = await invoke("install_mcp_service");
    console.log($t('message.installResult'), result);
    
    serviceStatus.value = $t('message.installed');
    installOutput.value = $t('message.installSuccess');
    
    // 刷新状态
    await checkServiceStatus();
  } catch (error) {
    console.error($t('message.installFailed'), error);
    installError.value = `${$t('message.installFailed')}: ${error}`;
  } finally {
    isLoading.value = false;
  }
}

// 启动 MCP 服务
async function startMCPService() {
  try {
    isLoading.value = true;
    installError.value = "";
    installOutput.value = "正在启动服务...";
    
    await invoke("start_mcp_service");
    
    // 刷新状态
    await checkServiceStatus();
  } catch (error) {
    console.error("启动服务失败:", error);
    installError.value = `启动失败: ${error}`;
  } finally {
    isLoading.value = false;
  }
}

// 停止 MCP 服务
async function stopMCPService() {
  try {
    isLoading.value = true;
    installError.value = "";
    installOutput.value = "正在停止服务...";
    
    await invoke("stop_mcp_service");
    
    // 刷新状态
    await checkServiceStatus();
  } catch (error) {
    console.error("停止服务失败:", error);
    installError.value = `停止失败: ${error}`;
  } finally {
    isLoading.value = false;
  }
}

onMounted(async () => {
  await checkServiceStatus();
});
</script>

<template>
  <div class="mcp-service">
    <h2>{{ $t('message.mcpService') }}</h2>
    
    <div class="status-card">
      <h3>服务状态</h3>
      <div class="status-badge" :class="serviceStatus">
        {{ serviceStatus }}
      </div>
      
      <div class="action-buttons">
        <button @click="installMCPService" :disabled="isLoading">
          安装
        </button>
        <button @click="startMCPService" :disabled="isLoading || serviceStatus !== '已安装'">
          启动服务
        </button>
        <button @click="stopMCPService" :disabled="isLoading || serviceStatus !== '运行中'">
          停止服务
        </button>
      </div>
    </div>
    
    <div v-if="installOutput || installError" class="output-section">
      <div v-if="installOutput" class="output">
        {{ installOutput }}
      </div>
      <div v-if="installError" class="error">
        {{ installError }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.mcp-service {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.status-card {
  background-color: #f9f9f9;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.status-badge {
  display: inline-block;
  padding: 6px 12px;
  border-radius: 4px;
  font-weight: bold;
  margin: 10px 0;
}

.status-badge.未安装 {
  background-color: #f8d7da;
  color: #721c24;
}

.status-badge.已安装 {
  background-color: #d4edda;
  color: #155724;
}

.status-badge.运行中 {
  background-color: #cce5ff;
  color: #004085;
}

.action-buttons {
  margin-top: 15px;
  display: flex;
  gap: 10px;
}

button {
  padding: 8px 16px;
  border-radius: 4px;
  background-color: #42b983;
  color: white;
  border: none;
  cursor: pointer;
}

button:hover {
  background-color: #3aa876;
}

button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

.output-section {
  margin-top: 20px;
  padding: 15px;
  border-radius: 8px;
  background-color: #f5f5f5;
}

.output {
  white-space: pre-wrap;
  margin-bottom: 10px;
}

.error {
  color: #721c24;
  background-color: #f8d7da;
  padding: 10px;
  border-radius: 4px;
}

@media (prefers-color-scheme: dark) {
  .status-card {
    background-color: #2a2a2a;
  }
  
  .output-section {
    background-color: #333;
  }
  
  .error {
    background-color: #3f2229;
    color: #f8d7da;
  }
}
</style>