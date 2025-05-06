<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";

const activeMenu = ref('openai');
const customConfigs = ref([]);

const openaiConfig = ref({
  api_url: '',
  model: '',
  session_key: '',
  endpoint: ''
});

const ollamaConfig = ref({
  api_url: '',
  endpoint: '',
  model: ''
});

const newApiConfig = ref({
  name: '',
  api_url: '',
  model: '',
  session_key: ''
});

onMounted(async () => {
  await loadCustomConfigs();
  await loadModelConfigs();
});

async function loadModelConfigs() {
  try {
    // 加载 OpenAI 配置
    const openaiResult = await invoke('get_model_config', { provider: 'openai' });
    if (openaiResult) {
      openaiConfig.value = openaiResult;
    }
    
    // 加载 Ollama 配置
    const ollamaResult = await invoke('get_model_config', { provider: 'ollama' });
    if (ollamaResult) {
      ollamaConfig.value = ollamaResult;
    }
  } catch (error) {
    console.error('加载模型配置失败:', error);
  }
}

async function saveOpenAIConfig() {
  try {
    console.log('Saving OpenAI config:', openaiConfig.value);
    await invoke('save_model_config', { 
      provider: 'openai',
      config: {
        api_url: formatapi_url(openaiConfig.value.api_url),
        model: openaiConfig.value.model,
        session_key: openaiConfig.value.session_key, // 添加这一行
        endpoint: openaiConfig.value.endpoint
      }
    });
    await loadModelConfigs(); // 保存后重新加载
  } catch (error) {
    console.error('保存OpenAI配置失败:', error);
  }
}

async function saveOllamaConfig() {
  try {
    console.log('Saving Ollama config:', ollamaConfig.value);
    await invoke('save_model_config', { 
      provider: 'ollama',
      config: {
        api_url: ollamaConfig.value.api_url,
        model: ollamaConfig.value.model,
        session_key: ollamaConfig.value.session_key || '', // 添加这一行
        endpoint: formatapi_url(ollamaConfig.value.endpoint)
      }
    });
    await loadModelConfigs(); // 保存后重新加载
  } catch (error) {
    console.error('保存Ollama配置失败:', error);
  }
}

async function loadCustomConfigs() {
  try {
    const configs = await invoke('get_custom_configs');
    console.log('Loaded configs:', configs); // Add this for debugging
    customConfigs.value = Array.isArray(configs) ? configs : [];
  } catch (error) {
    console.error('加载API配置失败:', error);
    customConfigs.value = [];
  }
}

async function saveNewApiConfig() {
  try {
    // 使用自定义名称作为provider
    await invoke('save_model_config', { 
      provider: newApiConfig.value.name,
      config: {
        api_url: formatapi_url(newApiConfig.value.api_url),
        model: newApiConfig.value.model,
        session_key: newApiConfig.value.session_key,
        endpoint: ''
      }
    });
    await loadCustomConfigs();
    newApiConfig.value = { name: '', api_url: '', model: '', session_key: '' };
  } catch (error) {
    console.error('保存API配置失败:', error);
  }
}

const formatapi_url = (url: string) => {
  if (!url) return '';
  if (url.endsWith('#')) {
    return url.slice(0, -1); // 移除#并保持原始地址
  }
  if (url.endsWith('/')) {
    const baseUrl = url.slice(0, -1); // 移除末尾的/
    return `${baseUrl}/v1/chat/completions`;
  }
  return url;
};

const handleapi_urlInput = (event: Event, config: any) => {
  const input = (event.target as HTMLInputElement).value;
  config.api_url = formatapi_url(input);
};
</script>

<template>
  <div class="model-settings">
    <div class="settings-container">
      <div class="settings-menu">
        <h3>模型配置</h3>
        <ul>
          <li :class="{ active: activeMenu === 'openai' }" @click="activeMenu = 'openai'">
            OpenAI配置
          </li>
          <li :class="{ active: activeMenu === 'ollama' }" @click="activeMenu = 'ollama'">
            Ollama配置
          </li>
          <li :class="{ active: activeMenu === 'newApi' }" @click="activeMenu = 'newApi'">
            添加API配置
          </li>
          <div class="submenu" v-if="customConfigs.length > 0">
            <li v-for="config in customConfigs" 
                :key="config?.name || ''"
                :class="{ active: activeMenu === `api-${config?.name}` }"
                @click="activeMenu = `api-${config?.name}`">
              {{ config?.name }}
            </li>
          </div>
        </ul>
      </div>

      <div class="settings-content">
        <div v-if="activeMenu === 'openai'" class="config-section">
          <h3>OpenAI配置</h3>
          <form @submit.prevent="saveOpenAIConfig">
            <div class="form-group">
                <label>API 地址:</label>
                <input 
                  type="text" 
                  :value="openaiConfig.api_url"
                  @input="(e) => handleapi_urlInput(e, openaiConfig)"
                  placeholder="https://api.siliconflow.cn" 
                />
                <div class="input-tip">
                  提示：/ 结尾自动补全 /v1/chat/completions，# 结尾使用原始地址
                </div>
              </div>
            <div class="form-group">
              <label>模型名称:</label>
              <input type="text" v-model="openaiConfig.model" />
            </div>
            <div class="form-group">
              <label>Session Key:</label>
              <input type="text" v-model="openaiConfig.session_key" />
            </div>
            <button type="submit">保存配置</button>
          </form>
        </div>

        <div v-if="activeMenu === 'ollama'" class="config-section">
          <h3>Ollama配置</h3>
          <form @submit.prevent="saveOllamaConfig">
            <div class="form-group">
                <label>API 地址:</label>
                <input 
                  type="text" 
                  :value="ollamaConfig.endpoint"
                  @input="(e) => handleapi_urlInput(e, ollamaConfig)"
                  placeholder="https://api.siliconflow.cn" 
                />
                <div class="input-tip">
                  提示：/ 结尾自动补全 /v1/chat/completions，# 结尾使用原始地址
                </div>
              </div>
            <div class="form-group">
              <label>模型名称:</label>
              <input type="text" v-model="ollamaConfig.model" />
            </div>
            <button type="submit">保存配置</button>
          </form>
        </div>

        <div v-if="activeMenu === 'newApi'" class="config-section">
          <h3>添加API配置</h3>
          <form @submit.prevent="saveNewApiConfig">
            <div class="form-group">
              <label>配置名称:</label>
              <input type="text" v-model="newApiConfig.name" required />
            </div>
            <div class="form-group">
              <label>API 地址:</label>
              <input 
                type="text" 
                :value="newApiConfig.api_url"
                @input="(e) => handleapi_urlInput(e, newApiConfig)"
                placeholder="https://api.siliconflow.cn" 
              />
              <div class="input-tip">
                提示：/ 结尾自动补全 /v1/chat/completions，# 结尾使用原始地址
              </div>
            </div>
            <div class="form-group">
              <label>模型名称:</label>
              <input type="text" v-model="newApiConfig.model" />
            </div>
            <div class="form-group">
              <label>Session Key:</label>
              <input type="text" v-model="newApiConfig.session_key" />
            </div>
            <button type="submit">保存配置</button>
          </form>
        </div>

        <div v-for="config in customConfigs" 
             :key="config?.name || ''"
             v-if="activeMenu === `api-${config?.name}`"
             class="config-section">
          <h3>{{ config?.name }}</h3>
          <div class="form-group">
            <label>API Key:</label>
            <input type="password" :value="config?.api_url" readonly />
          </div>
          <div class="form-group">
            <label>模型名称:</label>
            <input type="text" :value="config?.model" readonly />
          </div>
          <div class="form-group">
            <label>Session Key:</label>
            <input type="text" :value="config?.session_key" readonly />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.model-settings {
  padding: 20px;
  height: 100%;
}

.settings-container {
  display: flex;
  gap: 20px;
  height: 100%;
  max-width: 1200px;
  margin: 0 auto;
}

.settings-menu {
  width: 200px;
  padding: 20px;
  background-color: #f5f5f5;
  border-radius: 8px;
  color: #333333;
}

.settings-menu h3 {
  margin-bottom: 15px;
  font-size: 1.2em;
  font-weight: 500;
}

.settings-menu ul {
  list-style: none;
  padding: 0;
}

.settings-menu li {
  padding: 12px 15px;
  margin-bottom: 5px;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.3s ease;
}

.settings-menu li:hover {
  background-color: #e0e0e0;
}

.settings-menu li.active {
  background-color: #42b983;
  color: white;
}

.submenu {
  margin-left: 15px;
  margin-top: 5px;
}

.submenu li {
  font-size: 0.9em;
  padding: 8px 12px;
}

.settings-content {
  flex: 1;
}

.config-section {
  padding: 20px;
  border: 1px solid #ddd;
  border-radius: 8px;
  background-color: #f9f9f9;
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
  background-color: #fff;
  color: #333;
  font-family: monospace;
}

.form-group input:focus {
  outline: none;
  border-color: #42b983;
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

.input-tip {
  font-size: 0.8em;
  color: #666;
  margin-top: 4px;
}

@media (prefers-color-scheme: dark) {
  .settings-menu {
    background-color: #1a1a1a;
    color: #ffffff;
  }
  
  .settings-menu li:hover {
    background-color: #333333;
  }
  
  .config-section {
    border: 1px solid rgba(255, 255, 255, 0.1);
    background-color: rgba(255, 255, 255, 0.05);
    color: #fff;
  }
  
  .form-group input {
    border: 1px solid rgba(255, 255, 255, 0.1);
    background-color: rgba(255, 255, 255, 0.05);
    color: #fff;
  }
  
  .input-tip {
    color: rgba(255, 255, 255, 0.5);
  }
}
</style>
