<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 添加工具状态
const toolsStatus = ref({
  uv: "未安装",
  bun: "未安装",
  git: "未安装"
});

const cmdOutput = ref("");
const cmdError = ref("");
const isLoading = ref(false);

// 添加安装工具的方法
async function installTool(tool: string) {
  try {
    isLoading.value = true;
    cmdError.value = ""; 
    cmdOutput.value = `正在安装 ${tool}...`;
    console.log(`开始安装 ${tool}...`);
    
    const result = await invoke("install_single_tool", { tool });
    console.log(`安装结果:`, result);
    
    toolsStatus.value[tool as keyof typeof toolsStatus.value] = "已安装";
    cmdOutput.value = `${tool} 安装成功！`;
  } catch (error) {
    console.error(`安装失败:`, error);
    cmdError.value = `安装失败: ${error}`;
    toolsStatus.value[tool as keyof typeof toolsStatus.value] = "安装失败";
  } finally {
    isLoading.value = false;
  }
}

// 检查工具状态
async function checkToolsStatus() {
  try {
    const status = await invoke("check_tools_status");
    toolsStatus.value = status as typeof toolsStatus.value;
  } catch (error) {
    console.error("检查工具状态失败:", error);
  }
}

// 初始化时检查工具状态
checkToolsStatus();
</script>

<template>
  <div class="tools-section">
    <h2>工具管理</h2>
    <div class="tools-grid">
      <div class="tool-card">
        <h3>UV</h3>
        <p>状态: {{ toolsStatus.uv }}</p>
        <button 
          @click="installTool('uv')"
          :disabled="isLoading || toolsStatus.uv === '已安装'"
        >
          {{ isLoading ? '安装中...' : (toolsStatus.uv === '已安装' ? '已安装' : '安装 UV') }}
        </button>
      </div>
      <div class="tool-card">
        <h3>Bun</h3>
        <p>状态: {{ toolsStatus.bun }}</p>
        <button 
          @click="installTool('bun')"
          :disabled="isLoading || toolsStatus.bun === '已安装'"
        >
          {{ isLoading ? '安装中...' : (toolsStatus.bun === '已安装' ? '已安装' : '安装 Bun') }}
        </button>
      </div>
      <div class="tool-card">
        <h3>Git</h3>
        <p>状态: {{ toolsStatus.git }}</p>
        <button 
          @click="installTool('git')"
          :disabled="isLoading || toolsStatus.git === '已安装'"
        >
          {{ isLoading ? '安装中...' : (toolsStatus.git === '已安装' ? '已安装' : '安装 Git') }}
        </button>
      </div>
    </div>
    
    <div class="output-container">
      <h3>执行结果</h3>
      <div v-if="isLoading" class="loading">命令执行中...</div>
      <pre v-if="cmdOutput" class="cmd-output">{{ cmdOutput }}</pre>
      <pre v-if="cmdError" class="cmd-error">{{ cmdError }}</pre>
    </div>
  </div>
</template>

<style scoped>
.tools-section {
  margin: 2rem auto;
  width: 100%;
  max-width: 800px;
}

.tools-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
  margin-top: 1rem;
}

.tool-card {
  padding: 1rem;
  border-radius: 8px;
  background-color: #f1f1f1;
  text-align: center;
}

.output-container {
  margin-top: 1rem;
  padding: 1rem;
  background-color: #f1f1f1;
  border-radius: 8px;
  text-align: left;
  max-height: 300px;
  overflow: auto;
  width: 100%;
}

.loading {
  color: #2196F3;
  font-style: italic;
}

.cmd-output {
  white-space: pre-wrap;
  word-break: break-all;
  margin: 0;
}

.cmd-error {
  white-space: pre-wrap;
  word-break: break-all;
  color: #e53935;
  margin: 0;
}

@media (prefers-color-scheme: dark) {
  .tool-card {
    background-color: #1a1a1a;
  }
  
  .output-container {
    background-color: #1a1a1a;
  }
}
</style>