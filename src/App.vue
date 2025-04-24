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

async function executeCommand() {
  if (!cmdInput.value) return;
  
  isLoading.value = true;
  cmdOutput.value = "";
  cmdError.value = "";
  
  try {
    // 将参数字符串拆分为数组
    const args = cmdArgs.value ? cmdArgs.value.split(" ") : [];
    
    console.log(`执行命令: ${cmdInput.value} ${args.join(' ')}`);
    
    // 添加到历史记录
    const fullCommand = cmdInput.value + (cmdArgs.value ? ' ' + cmdArgs.value : '');
    commandHistory.value.unshift(fullCommand);
    if (commandHistory.value.length > 10) {
      commandHistory.value.pop();
    }
    
    const result = await invoke("execute_command", { 
      cmd: cmdInput.value,
      args: args
    });
    
    // 确保结果显示在界面上
    cmdOutput.value = result as string;
    console.log(`命令执行结果: ${cmdOutput.value}`);
  } catch (error) {
    cmdError.value = error as string;
    console.error(`命令执行失败: ${error}`);
  } finally {
    isLoading.value = false;
  }
}

onMounted(() => {
  console.log("应用已加载");
});
</script>

<template>
  <main class="container">
    <h1>命令执行工具</h1>

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
      
      <div class="output-container" v-if="isLoading || cmdOutput || cmdError">
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