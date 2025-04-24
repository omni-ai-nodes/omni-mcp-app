<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

// 命令执行相关
const cmdInput = ref("");
const cmdArgs = ref("");
const cmdOutput = ref("");
const cmdError = ref("");
const isLoading = ref(false);
const commandHistory = ref<string[]>([]); // 添加命令历史记录数组

// 添加工具状态
const toolsStatus = ref({
  uv: "未安装",
  bun: "未安装"
});

// 添加安装工具的方法
async function installTool(tool: string) {
  try {
    isLoading.value = true;
    cmdError.value = ""; // 清除之前的错误信息
    cmdOutput.value = `正在安装 ${tool}...`; // 显示安装进度
    console.log(`开始安装 ${tool}...`); // 添加日志
    
    const result = await invoke("install_single_tool", { tool });
    console.log(`安装结果:`, result); // 添加日志
    
    toolsStatus.value[tool as keyof typeof toolsStatus.value] = "已安装";
    cmdOutput.value = `${tool} 安装成功！`;
  } catch (error) {
    console.error(`安装失败:`, error); // 添加错误日志
    cmdError.value = `安装失败: ${error}`;
    toolsStatus.value[tool as keyof typeof toolsStatus.value] = "安装失败";
  } finally {
    isLoading.value = false;
  }
}

onMounted(async () => {
  console.log("应用已加载");
  
  // 检查工具状态
  try {
    const status = await invoke("check_tools_status");
    toolsStatus.value = status as typeof toolsStatus.value;
  } catch (error) {
    console.error("检查工具状态失败:", error);
  }
});
</script>

<template>
  <main class="container">
    <h1>命令执行工具</h1>
    
    <!-- 工具状态和安装按钮 -->
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
            {{ isLoading ? '安装中...' : '安装 UV' }}
          </button>
        </div>
        <div class="tool-card">
          <h3>Bun</h3>
          <p>状态: {{ toolsStatus.bun }}</p>
          <button 
            @click="installTool('bun')"
            :disabled="isLoading || toolsStatus.bun === '已安装'"
          >
            {{ isLoading ? '安装中...' : '安装 Bun' }}
          </button>
        </div>
      </div>
    </div>

    <!-- 命令执行部分 -->
    <div class="command-section">
      <h2>命令执行</h2>
      <form class="row" @submit.prevent="executeCommand">
        <input 
          id="cmd-input" 
          v-model="cmdInput" 
          placeholder="输入命令..." 
          required
        />
        <input 
          id="args-input" 
          v-model="cmdArgs" 
          placeholder="参数（可选）" 
        />
        <button type="submit" :disabled="isLoading">
          {{ isLoading ? '执行中...' : '执行' }}
        </button>
      </form>
      
      <div v-if="commandHistory.length > 0" class="history-container">
        <h3>命令历史</h3>
        <ul>
          <li v-for="(cmd, index) in commandHistory" :key="index" @click="cmdInput = cmd">
            {{ cmd }}
          </li>
        </ul>
      </div>
      
      <!-- 修改输出容器的显示条件 -->
      <div class="output-container">
        <h3>执行结果</h3>
        <div v-if="isLoading" class="loading">命令执行中...</div>
        <pre v-if="cmdOutput" class="cmd-output">{{ cmdOutput }}</pre>
        <pre v-if="cmdError" class="cmd-error">{{ cmdError }}</pre>
      </div>
    </div>
    
    <!-- 原有的问候部分 -->
    <div class="greeting-section">
      <h2>问候功能</h2>
      <form class="row" @submit.prevent="greet">
        <input id="greet-input" v-model="name" placeholder="输入名字..." />
        <button type="submit">问候</button>
      </form>
      <p>{{ greetMsg }}</p>
    </div>
  </main>
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

@media (prefers-color-scheme: dark) {
  .tool-card {
    background-color: #1a1a1a;
  }
}

.command-section {
  margin-top: 2rem;
  width: 100%;
  max-width: 800px;
}

.greeting-section {
  margin-top: 2rem;
  border-top: 1px solid #ccc;
  padding-top: 1rem;
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

.history-container {
  margin-top: 1rem;
  text-align: left;
}

.history-container ul {
  list-style: none;
  padding: 0;
}

.history-container li {
  cursor: pointer;
  padding: 5px;
  border-radius: 4px;
}

.history-container li:hover {
  background-color: #f1f1f1;
}

.cmd-output {
  white-space: pre-wrap;
  word-break: break-all;
}

.cmd-error {
  white-space: pre-wrap;
  word-break: break-all;
  color: #e53935;
}

#cmd-input {
  flex: 2;
  margin-right: 5px;
}

#args-input {
  flex: 2;
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  .output-container {
    background-color: #1a1a1a;
  }
  
  .history-container li:hover {
    background-color: #333;
  }
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>