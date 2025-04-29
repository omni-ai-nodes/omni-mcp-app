<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";  // 添加这行导入语句

const cmdInput = ref("");
const cmdArgs = ref("");
const cmdOutput = ref("");
const cmdError = ref("");
const isLoading = ref(false);
const commandHistory = ref<string[]>([]);
const containerWidth = ref("100%");

// 保存用户调整的宽度到本地存储
function saveContainerWidth() {
  if (typeof window !== 'undefined') {
    localStorage.setItem('commandExecutorWidth', containerWidth.value);
  }
}

// 从本地存储加载用户之前调整的宽度
onMounted(() => {
  if (typeof window !== 'undefined') {
    const savedWidth = localStorage.getItem('commandExecutorWidth');
    if (savedWidth) {
      containerWidth.value = savedWidth;
    }
  }
});

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
    cmdOutput.value = result as string;
  } catch (error) {
    console.error("命令执行失败:", error);
    cmdError.value = `执行失败: ${error}`;
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <div class="command-section" :style="{ width: containerWidth }">
    <h2>{{ $t('message.commandExecution') }}</h2>
    <form class="row" @submit.prevent="executeCommand">
      <input 
        id="cmd-input" 
        v-model="cmdInput" 
        :placeholder="$t('message.enterCommand')" 
        required
      />
      <input 
        id="args-input" 
        v-model="cmdArgs" 
        :placeholder="$t('message.parameters')" 
      />
      <button type="submit" :disabled="isLoading">
        {{ isLoading ? $t('message.executing') : $t('message.execute') }}
      </button>
    </form>
    
    <div v-if="commandHistory.length > 0" class="history-container">
      <h3>{{ $t('message.commandHistory') }}</h3>
      <ul>
        <li v-for="(cmd, index) in commandHistory" :key="index" @click="cmdInput = cmd">
          {{ cmd }}
        </li>
      </ul>
    </div>
    
    <div class="output-container">
      <h3>{{ $t('message.executionResult') }}</h3>
      <div v-if="isLoading" class="loading">{{ $t('message.executingCommand') }}</div>
      <pre v-if="cmdOutput" class="cmd-output">{{ cmdOutput }}</pre>
      <pre v-if="cmdError" class="cmd-error">{{ cmdError }}</pre>
    </div>
  </div>
</template>

<style scoped>
.command-section {
  margin: 2rem auto;
  max-width: 800px;
  transition: width 0.3s ease;
  position: relative;
}

.row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 1rem;
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
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.history-container {
  margin-top: 1rem;
  text-align: left;
}

.history-container ul {
  list-style: none;
  padding: 0;
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.history-container li {
  cursor: pointer;
  padding: 5px 10px;
  border-radius: 4px;
  background-color: #f5f5f5;
  border: 1px solid #e0e0e0;
  transition: background-color 0.2s;
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
  flex: 1;
  min-width: 200px;
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid #ccc;
}

button {
  padding: 8px 16px;
  border-radius: 4px;
  background-color: #42b983;
  color: white;
  border: none;
  cursor: pointer;
  transition: background-color 0.2s;
}

button:hover {
  background-color: #3aa876;
}

button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

@media (max-width: 768px) {
  .command-section {
    width: 95% !important;
  }
  
  .row {
    flex-direction: column;
  }
  
  #cmd-input, #args-input {
    width: 100%;
    margin-right: 0;
    margin-bottom: 8px;
  }
}

@media (prefers-color-scheme: dark) {
  .output-container {
    background-color: #1a1a1a;
  }
  
  .history-container li {
    background-color: #2a2a2a;
    border-color: #333;
  }
  
  .history-container li:hover {
    background-color: #333;
  }
  
  #cmd-input, #args-input {
    background-color: #2a2a2a;
    border-color: #444;
    color: #fff;
  }
}
</style>