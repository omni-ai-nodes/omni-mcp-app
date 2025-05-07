<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const message = ref('');
const response = ref('');
const isLoading = ref(false);

async function handleSubmit() {
  if (!message.value.trim() || isLoading.value) return;
  
  try {
    isLoading.value = true;
    const result = await invoke('parse_mcp_config', { config: message.value });
    response.value = result as string;
    message.value = ''; // 清空输入
  } catch (error) {
    console.error('提交失败:', error);
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <div class="greeter-container">
    <h2>请输入MCP配置信息</h2>
    <div class="input-area">
      <form @submit.prevent="handleSubmit">
        <textarea 
          v-model="message" 
          placeholder="请输入JSON格式的MCP配置信息"
          required
          rows="20"
        ></textarea>
        <button type="submit" :disabled="isLoading">
          {{ isLoading ? '提交中...' : '提交' }}
        </button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.greeter-container {
  margin: 2rem auto;
  max-width: 800px;
  padding: 1rem;
}

.input-area {
  margin: 1rem 0;
}

form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid #ccc;
  border-radius: 4px;
  resize: vertical;
  min-height: 400px;
  font-family: monospace;
}

button {
  padding: 12px 24px;
  background-color: #42b983;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
  align-self: flex-end;
  width: 120px;
}

button:hover {
  background-color: #3aa876;
}

button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

.response-container {
  margin-top: 1rem;
  padding: 1rem;
  background-color: #f1f1f1;
  border-radius: 8px;
}

.response-content {
  white-space: pre-wrap;
  word-break: break-all;
}

@media (prefers-color-scheme: dark) {
  .response-container {
    background-color: #1a1a1a;
  }
  
  textarea {
    background-color: #2a2a2a;
    border-color: #444;
    color: #fff;
  }
}

@media (max-width: 768px) {
  .greeter-container {
    margin: 1rem;
    padding: 0;
  }
  
  button {
    width: 100%;
  }
}
</style>