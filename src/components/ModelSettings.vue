<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";

const openaiConfig = ref({
  apiKey: '',
  model: '',
  sessionKey: ''
});

const ollamaConfig = ref({
  endpoint: '',
  model: ''
});

async function saveOpenAIConfig() {
  try {
    await invoke('save_model_config', { 
      provider: 'openai',
      config: openaiConfig.value 
    });
  } catch (error) {
    console.error('保存OpenAI配置失败:', error);
  }
}

async function saveOllamaConfig() {
  try {
    await invoke('save_model_config', { 
      provider: 'ollama',
      config: ollamaConfig.value 
    });
  } catch (error) {
    console.error('保存Ollama配置失败:', error);
  }
}
</script>

<template>
  <div class="model-settings">
    <h2>{{ $t('message.modelSettings') }}</h2>
    
    <div class="config-section">
      <h3>OpenAI配置</h3>
      <form @submit.prevent="saveOpenAIConfig">
        <div class="form-group">
          <label>API Key:</label>
          <input type="password" v-model="openaiConfig.apiKey" />
        </div>
        <div class="form-group">
          <label>模型名称:</label>
          <input type="text" v-model="openaiConfig.model" />
        </div>
        <div class="form-group">
          <label>Session Key:</label>
          <input type="text" v-model="openaiConfig.sessionKey" />
        </div>
        <button type="submit">保存OpenAI配置</button>
      </form>
    </div>

    <div class="config-section">
      <h3>Ollama配置</h3>
      <form @submit.prevent="saveOllamaConfig">
        <div class="form-group">
          <label>端点地址:</label>
          <input type="text" v-model="ollamaConfig.endpoint" />
        </div>
        <div class="form-group">
          <label>模型名称:</label>
          <input type="text" v-model="ollamaConfig.model" />
        </div>
        <button type="submit">保存Ollama配置</button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.model-settings {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.config-section {
  margin-bottom: 30px;
  padding: 20px;
  border: 1px solid #ddd;
  border-radius: 8px;
}

.form-group {
  margin-bottom: 15px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
}

.form-group input {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

button {
  padding: 8px 16px;
  background-color: #42b983;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background-color: #3aa876;
}
</style>