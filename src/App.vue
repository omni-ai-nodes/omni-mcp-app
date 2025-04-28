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
  <div class="app-container">
    <nav class="side-nav">
      <router-link to="/greeter">Chat</router-link>
      <router-link to="/">MCP服务安装</router-link>
      <router-link to="/tools">工具管理</router-link>
    </nav>
    <main class="content-container">
      <router-view></router-view>
    </main>
  </div>
</template>

<style>
.app-container {
  display: flex;
  min-height: 100vh;
}

.side-nav {
  width: 200px;
  background-color: #f1f1f1;
  padding: 20px 0;
  display: flex;
  flex-direction: column;
}

.side-nav a {
  color: #2c3e50;
  text-decoration: none;
  padding: 10px 20px;
  margin: 5px 0;
  border-left: 3px solid transparent;
}

.side-nav a.router-link-active {
  color: #42b983;
  border-left: 3px solid #42b983;
  background-color: rgba(66, 185, 131, 0.1);
}

.content-container {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  /* 隐藏滚动条但保留功能 */
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
}

/* 为 Webkit 浏览器隐藏滚动条 */
.content-container::-webkit-scrollbar {
  display: none;
}

/* 全局隐藏所有滚动条 */
::-webkit-scrollbar {
  width: 0px;
  background: transparent;
}

@media (prefers-color-scheme: dark) {
  .side-nav {
    background-color: #1a1a1a;
  }
  
  .side-nav a {
    color: #ffffff;
  }
  
  .side-nav a.router-link-active {
    color: #42b983;
  }
}
</style>

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

/* 删除重复的 cmd-output 和 cmd-error 样式定义 */
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

/* 响应式布局 */
@media (max-width: 768px) {
  .app-container {
    flex-direction: column;
  }
  
  .side-nav {
    width: 100%;
    padding: 10px 0;
    flex-direction: row;
    justify-content: center;
    flex-wrap: wrap;
  }
  
  .side-nav a {
    padding: 8px 15px;
    margin: 3px;
    border-left: none;
    border-bottom: 3px solid transparent;
  }
  
  .side-nav a.router-link-active {
    border-left: none;
    border-bottom: 3px solid #42b983;
  }
  
  .content-container {
    padding: 10px;
  }
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

<style scoped>
.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.features-toggle {
  margin: 2rem auto;
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.features-toggle button {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
}
</style>
