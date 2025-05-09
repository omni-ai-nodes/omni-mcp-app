<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { confirm } from '@tauri-apps/plugin-dialog';

const activeMenu = ref('openai');
const customConfigs = ref([]);

const openaiConfig = ref({
  api_url: '',
  model: '',
  session_key: '',
  method: '/v1/chat/completions'  // 添加默认值
});

const ollamaConfig = ref({
  api_url: '',
  model: '',
  session_key: '',
  method: '/v1/chat/completions'  // 添加默认值
});

const newApiConfig = ref({
  name: '',
  api_url: '',
  model: '',
  session_key: '',
  method: '/v1/chat/completions'  // 添加默认值
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
        provider: 'openai',  // 添加这一行，确保 config 对象中也包含 provider
        api_url: formatapi_url(openaiConfig.value.api_url),
        model: openaiConfig.value.model,
        session_key: openaiConfig.value.session_key,
        endpoint: openaiConfig.value.endpoint || ''
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
        provider: 'ollama',  // 添加这一行，确保 config 对象中也包含 provider
        api_url: ollamaConfig.value.api_url || '',
        model: ollamaConfig.value.model,
        session_key: ollamaConfig.value.session_key || '',
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
    console.log('Loaded custom configs:', configs);
    customConfigs.value = Array.isArray(configs) ? configs : [];
  } catch (error) {
    console.error('加载自定义API配置失败:', error);
    customConfigs.value = [];
  }
}

async function saveNewApiConfig() {
  try {
    if (!newApiConfig.value.name) {
      alert('配置名称不能为空');
      return;
    }
    
    console.log('Saving new API config:', newApiConfig.value);
    
    // 保存配置名称以便后续使用
    const configName = newApiConfig.value.name;
    
    await invoke('save_model_config', { 
      provider: configName,
      config: {
        provider: configName,  // Add this line to include the provider field
        api_url: formatapi_url(newApiConfig.value.api_url),
        model: newApiConfig.value.model,
        session_key: newApiConfig.value.session_key,
        endpoint: ''
      }
    });
    
    // 保存成功后重新加载自定义配置
    await loadCustomConfigs();
    
    // 清空表单
    newApiConfig.value = { name: '', api_url: '', model: '', session_key: '' };
    
    // 切换到新添加的配置页面，使用保存的名称
    activeMenu.value = `api-${configName}`;
  } catch (error) {
    console.error('保存API配置失败:', error);
    alert(`保存失败: ${error}`);
  }
}

const formatapi_url = (url: string) => {
  if (!url) return '';
  if (url.endsWith('#')) {
    return url.slice(0, -1); // 移除#并保持原始地址
  }
  if (url.endsWith('/v')) {
    const baseUrl = url.slice(0, -1); // 移除末尾的/
    return `${baseUrl}v1/chat/completions`;
  }
  return url;
};

const handleapi_urlInput = (event: Event, config: any) => {
  const input = (event.target as HTMLInputElement).value;
  config.api_url = formatapi_url(input);
};

async function updateCustomConfig(config) {
  try {
    console.log('Updating custom config:', config);
    await invoke('save_model_config', { 
      provider: config.provider,
      config: {
        provider: config.provider,  // Add this line to include the provider field
        api_url: formatapi_url(config.api_url),
        model: config.model,
        session_key: config.session_key,
        endpoint: config.endpoint || ''
      }
    });
    await loadCustomConfigs();
    alert('配置已更新');
  } catch (error) {
    console.error('更新自定义配置失败:', error);
    alert(`更新失败: ${error}`);
  }
}

async function deleteCustomConfig(config) {
  console.log('确认对话框即将弹出');
  console.log('当前浏览器设置:', window.navigator.userAgent);
  console.log('confirm函数存在:', typeof confirm === 'function');
  const confirmed = await confirm(`确定要删除 ${config.provider} 配置吗？`, { title: '删除', kind: 'warning' });
  // const yes2 = await ask('This action cannot be reverted. Are you sure?', { title: 'Tauri', kind: 'warning' });

  if (!confirmed) {
    console.log('用户取消了删除操作');
    return;
  }
  
  try {
    console.log('调用Rust删除接口');
    await invoke('delete_model_config', { provider: config.provider });
    console.log('重新加载配置');
    await loadCustomConfigs();
    
    // 强制切换到默认菜单并触发视图更新
    activeMenu.value = 'openai';
    
    // 确保自定义配置列表更新
    customConfigs.value = customConfigs.value.filter(c => c.provider !== config.provider);
    
    // 如果当前活动菜单是被删除的配置，强制刷新视图
    if (activeMenu.value === `api-${config.provider}`) {
      activeMenu.value = 'openai';
    }
    
    // alert(`配置 ${config.provider} 已删除`);
  } catch (error) {
    console.error('删除自定义配置失败:', error);
    alert(`删除失败: ${error}`);
  }
}

async function handleCustomConfigClick(config) {
  activeMenu.value = `api-${config.provider}`;
  // 加载对应的配置数据
  try {
    const configData = await invoke('get_model_config', { provider: config.provider });
    console.log('获取到的配置数据:', configData);
    
    if (configData) {
      // 使用Vue的响应式API来更新数组
      const index = customConfigs.value.findIndex(c => c.provider === config.provider);
      if (index !== -1) {
        // 创建一个新的配置对象
        const updatedConfig = {
          provider: config.provider,
          api_url: configData.api_url || '',
          model: configData.model || '',
          session_key: configData.session_key || '',
          endpoint: configData.endpoint || '',
          method: configData.method || '/v1/chat/completions'  // 添加 method 字段
        };
        
        // 使用数组方法触发响应式更新
        customConfigs.value.splice(index, 1, updatedConfig);
      }
    }
  } catch (error) {
    console.error('加载配置数据失败:', error);
  }
}

// 获取模型列表
async function fetchModels(apiUrl: string, sessionKey?: string) {
  try {
    if (!apiUrl) {
      alert('请先输入API地址');
      return;
    }
    
    // 确保API地址不以/结尾
    const baseUrl = apiUrl.endsWith('/') ? apiUrl.slice(0, -1) : apiUrl;
    const modelsUrl = `${baseUrl}/v1/models`;
    
    console.log('请求模型列表URL:', modelsUrl);
    
    const headers: Record<string, string> = {
      'Content-Type': 'application/json'
    };

    // 如果提供了 session_key，添加到请求头
    if (sessionKey) {
      headers['Authorization'] = `Bearer ${sessionKey}`;
    }
    
    const response = await fetch(modelsUrl, {
      headers
    });
    
    if (!response.ok) {
      throw new Error(`请求失败: ${response.status} ${response.statusText}`);
    }
    
    const data = await response.json();
    console.log('获取到的模型列表数据:', data);
    
    if (data && data.object === 'list' && Array.isArray(data.data)) {
      // 提取模型ID
      const modelIds = data.data.map(model => model.id).join(',');
      console.log('提取的模型ID列表:', modelIds);
      return modelIds;
    } else {
      throw new Error('无效的模型数据格式');
    }
  } catch (error) {
    console.error('获取模型列表失败:', error);
    alert(`获取模型列表失败: ${error}`);
    return null;
  }
}

// 为OpenAI配置获取模型
async function fetchOpenAIModels() {
  console.log('开始获取OpenAI模型列表...');
  console.log('当前API地址:', openaiConfig.value.api_url);
  
  if (!openaiConfig.value.api_url) {
    console.error('API地址为空，无法获取模型列表');
    alert('请先输入API地址');
    return;
  }
  
  try {
    const models = await fetchModels(openaiConfig.value.api_url, openaiConfig.value.session_key);
    console.log('获取到的OpenAI模型列表:', models);
    
    if (models) {
      openaiConfig.value.model = models;
    } else {
      console.error('未获取到有效的模型列表');
    }
  } catch (error) {
    console.error('获取OpenAI模型列表出错:', error);
    alert(`获取模型列表失败: ${error}`);
  }
}

// 为Ollama配置获取模型
async function fetchOllamaModels() {
  const models = await fetchModels(ollamaConfig.value.api_url, ollamaConfig.value.session_key);
  if (models) {
    ollamaConfig.value.model = models;
  } else {
    console.error('未获取到有效的模型列表');
  }
}

// 为自定义配置获取模型
async function fetchCustomModels(config) {
  const models = await fetchModels(config.api_url, config.session_key);
  if (models) {
    config.model = models;
  }
}
</script>

<template>
  <div class="model-settings">
    <div class="settings-container">
      <div class="settings-menu">
        <h3>模型配置</h3>
        <ul>
          <li :class="{ active: activeMenu === 'newApi' }" @click="activeMenu = 'newApi'">
            添加API配置
          </li>
          <li :class="{ active: activeMenu === 'openai' }" @click="activeMenu = 'openai'">
            OpenAI配置
          </li>
          <li :class="{ active: activeMenu === 'ollama' }" @click="activeMenu = 'ollama'">
            Ollama配置
          </li>
          <li v-for="config in customConfigs" 
              :key="config?.provider || ''"
              :class="{ active: activeMenu === `api-${config?.provider}` }"
              @click="() => handleCustomConfigClick(config)">
            {{ config?.provider }}
          </li>
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
                  v-model="openaiConfig.api_url"
                  placeholder="https://api.siliconflow.cn" 
                />
            </div>
            <div class="form-group">
              <label>Session Key:</label>
              <input type="text" v-model="openaiConfig.session_key" />
            </div>
            <div class="form-group">
              <label>API 方法:</label>
              <input 
                type="text" 
                v-model="openaiConfig.method"
                placeholder="/v1/chat/completions" 
              />
              <div class="input-tip">
                  提示: API 结尾自动补全 /v1/chat/completions
                </div>
                <button type="button" class="fetch-btn" @click="fetchOpenAIModels">获取模型列表</button>
            </div>
            <div class="form-group">
              <label>模型名称:</label>
              <input type="text" v-model="openaiConfig.model" />
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
                  v-model="ollamaConfig.api_url"
                  placeholder="https://api.siliconflow.cn" 
                />
            </div>
            <div class="form-group">
              <label>API 方法:</label>
              <input 
                type="text" 
                v-model="ollamaConfig.method"
                placeholder="/v1/chat/completions" 
              />
              <div class="input-tip">
                提示: API 结尾自动补全 /v1/chat/completions
                </div>
                <button type="button" class="fetch-btn" @click="fetchOllamaModels">获取模型列表</button>
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
                v-model="newApiConfig.api_url"
                placeholder="https://api.siliconflow.cn" 
              />
            </div>
            <div class="form-group">
              <label>Session Key:</label>
              <input type="text" v-model="newApiConfig.session_key" />
            </div>
            <div class="form-group">
              <label>API 方法:</label>
              <input 
                type="text" 
                v-model="newApiConfig.method"
                placeholder="/v1/chat/completions" 
              />
              <div class="input-tip">
                提示: API 结尾自动补全 /v1/chat/completions
              </div>
              <button type="button" class="fetch-btn" @click="async () => {
                const models = await fetchModels(newApiConfig.api_url, newApiConfig.session_key);
                if (models) newApiConfig.model = models;
              }">获取模型列表</button>
            </div>
            <div class="form-group">
              <label>模型名称:</label>
              <input type="text" v-model="newApiConfig.model" />
             
            </div>
            <button type="submit">保存配置</button>
          </form>
        </div>

        <!-- 修改自定义配置部分 -->
        <template v-for="config in customConfigs" :key="config?.provider || ''">
          <div v-if="activeMenu === `api-${config?.provider}`"
               class="config-section">
            <h3>{{ config?.provider }}</h3>
            <form @submit.prevent="updateCustomConfig(config)">
              <div class="form-group">
                <label>API 地址:</label>
                <input 
                  type="text" 
                  v-model="config.api_url"
                  placeholder="https://api.siliconflow.cn" 
                />
                <div class="form-group">
                  <label>Session Key:</label>
                  <input type="text" v-model="config.session_key" />
                </div>
                <div class="input-tip">
                  提示：/ 结尾自动补全 /v1/chat/completions，# 结尾使用原始地址
                </div>
                <button type="button" class="fetch-btn" @click="() => fetchCustomModels(config)">获取模型列表</button>
              </div>
              <div class="form-group">
                <label>模型名称:</label>
                <input type="text" v-model="config.model" />
              </div>
              <div class="button-group">
                <button type="submit">更新配置</button>
                <button type="button" class="delete-btn" @click="() => deleteCustomConfig(config)">删除</button>
              </div>
            </form>
          </div>
        </template>
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

.button-group {
  display: flex;
  gap: 10px;
}

.delete-btn {
  background-color: #e74c3c;
}

.delete-btn:hover {
  background-color: #c0392b;
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

.fetch-btn {
  margin-top: 8px;
  background-color: #4a6bdf;
  padding: 6px 12px;
  font-size: 0.9em;
}

.fetch-btn:hover {
  background-color: #3a5bcf;
}
</style>
