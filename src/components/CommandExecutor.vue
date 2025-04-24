<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";  // 添加这行导入语句

const cmdInput = ref("");
const cmdArgs = ref("");
const cmdOutput = ref("");
const cmdError = ref("");
const isLoading = ref(false);
const commandHistory = ref<string[]>([]);

async function executeCommand() {
  try {
    isLoading.value = true;
    cmdError.value = "";
    
    // 添加命令到历史记录
    if (!commandHistory.value.includes(cmdInput.value)) {
      commandHistory.value.unshift(cmdInput.value);
    }
    
    const result = await invoke("execute_command", {
        cmd: cmdInput.value,
        args: cmdArgs.value ? cmdArgs.value.split(" ") : []
    });
    cmdOutput.value = result;
  } catch (error) {
    console.error("命令执行失败:", error);
    cmdError.value = `执行失败: ${error}`;
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
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
    
    <div class="output-container">
      <h3>执行结果</h3>
      <div v-if="isLoading" class="loading">命令执行中...</div>
      <pre v-if="cmdOutput" class="cmd-output">{{ cmdOutput }}</pre>
      <pre v-if="cmdError" class="cmd-error">{{ cmdError }}</pre>
    </div>
  </div>
</template>

<style scoped>
.command-section {
  margin-top: 2rem;
  width: 100%;
  max-width: 800px;
}

.row {
  display: flex;
  justify-content: center;
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

#cmd-input, #args-input {
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