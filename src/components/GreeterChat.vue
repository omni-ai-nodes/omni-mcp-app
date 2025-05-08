<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface Message {
  id: string;
  content: string;
  role: 'user' | 'assistant';
  timestamp: number;
}

interface Conversation {
  id: string;
  title: string;
  messages: Message[];
  timestamp: number;
  model: string; // 添加模型类型字段
}

interface ModelConfig {
  provider: string;
  api_url: string;
  model: string;
  method: string;
  session_key: string;
  endpoint?: string;
  modelOptions?: string[]; // 添加模型选项数组
}

const conversations = ref<Conversation[]>([]);
const currentConversation = ref<Conversation | null>(null);
const newMessage = ref('');
const loading = ref(false);
const availableModels = ref(['openai', 'ollama']); // 改为响应式数组
const currentModel = ref('openai');
const modelConfigs = ref<Record<string, ModelConfig>>({});
const streamingContent = ref('');
const eventSource = ref<EventSource | null>(null);
const modelOptions = ref<Record<string, string[]>>({});
const selectedModelOption = ref<string>('');

// 加载自定义模型配置
async function loadCustomConfigs() {
  try {
    const configs = await invoke('get_custom_configs', { filterType: 'ALL' }) as ModelConfig[];
    const customModels = configs.map(config => config.provider);
    availableModels.value = [...customModels];
    
    // 将配置保存到 modelConfigs 对象中
    configs.forEach(config => {
      modelConfigs.value[config.provider] = config;
      
      // 解析模型字符串，支持多个模型选择
      if (config.model && config.model.includes(',')) {
        const options = config.model.split(',').map(option => option.trim());
        modelOptions.value[config.provider] = options;
      } else {
        // 单个模型的情况，也创建选项数组
        modelOptions.value[config.provider] = [config.model];
      }
      
      // 设置默认选中的模型选项
      if (currentModel.value === config.provider) {
        selectedModelOption.value = modelOptions.value[config.provider][0];
      }
    });
    
    console.log('加载的模型配置:', modelConfigs.value);
    console.log('模型选项:', modelOptions.value);
  } catch (error) {
    console.error('加载自定义模型配置失败:', error);
  }
}

// 获取当前模型的配置
const currentModelConfig = computed(() => {
  return modelConfigs.value[currentModel.value] || null;
});

// 从本地存储加载对话记录
onMounted(async () => {
  await loadCustomConfigs(); // 加载自定义模型配置
  
  const savedConversations = localStorage.getItem('chatConversations');
  if (savedConversations) {
    conversations.value = JSON.parse(savedConversations);
    // 为旧的对话添加模型字段
    conversations.value = conversations.value.map(conv => ({
      ...conv,
      model: conv.model || 'openai'
    }));
    if (conversations.value.length > 0) {
      currentConversation.value = conversations.value[0];
      currentModel.value = conversations.value[0].model;
    }
  }
});

// 保存对话记录到本地存储
function saveConversations() {
  localStorage.setItem('chatConversations', JSON.stringify(conversations.value));
}

// 创建新对话
function createNewConversation() {
  const newConv: Conversation = {
    id: Date.now().toString(),
    title: '新对话',  // 先设置一个默认标题
    messages: [],
    timestamp: Date.now(),
    model: currentModel.value
  };
  conversations.value.unshift(newConv);
  currentConversation.value = newConv;
  saveConversations();
}

// 删除对话
function deleteConversation(conv: Conversation) {
  const index = conversations.value.findIndex(c => c.id === conv.id);
  if (index > -1) {
    conversations.value.splice(index, 1);
    if (currentConversation.value?.id === conv.id) {
      currentConversation.value = conversations.value[0] || null;
      if (currentConversation.value) {
        currentModel.value = currentConversation.value.model;
      }
    }
    saveConversations();
  }
}

// 切换模型
function changeModel(model: string) {
  if (currentConversation.value) {
    currentConversation.value.model = model;
    currentModel.value = model;
    
    // 设置默认的模型选项
    if (modelOptions.value[model] && modelOptions.value[model].length > 0) {
      selectedModelOption.value = modelOptions.value[model][0];
    } else {
      selectedModelOption.value = '';
    }
    
    saveConversations();
  }
}

// 选择具体的模型选项
function selectModelOption(option: string) {
  selectedModelOption.value = option;
  // 可以在这里添加其他逻辑，例如保存用户的选择
}

// 关闭 SSE 连接
function closeEventSource() {
  if (eventSource.value) {
    eventSource.value.close();
    eventSource.value = null;
  }
}

// 使用 SSE 发送消息
// 添加一个新的方法来处理滚动
function scrollToBottom() {
  const chatMessages = document.querySelector('.chat-messages');
  if (chatMessages) {
    chatMessages.scrollTop = chatMessages.scrollHeight;
  }
}

// 在 sendMessage 函数中的适当位置添加滚动调用
async function sendMessage() {
  if (!newMessage.value.trim() || !currentConversation.value || loading.value || !currentModelConfig.value) {
    return;
  }
  
  const userMessage: Message = {
    id: Date.now().toString(),
    content: newMessage.value,
    role: 'user',
    timestamp: Date.now()
  };
  
  // 如果是对话的第一条消息，则更新对话标题
  if (currentConversation.value.messages.length === 0) {
    // 获取用户消息的前20个字符作为标题，如果超过17个字符则添加省略号
    let title = userMessage.content.trim();
    if (title.length > 17) {
      title = title.substring(0, 17) + '...';
    }
    currentConversation.value.title = title;
  }
  
  currentConversation.value.messages.push(userMessage);
  newMessage.value = '';
  loading.value = true;
  streamingContent.value = '';
  
  try {
    // 关闭之前的连接
    closeEventSource();
    
    const config = currentModelConfig.value;
    // 使用选定的模型选项，如果没有则使用默认模型
    const modelName = selectedModelOption.value || config.model;
    const apiUrl = config.api_url;
    const method = config.method;
    const sessionKey = config.session_key;
    
    // 创建一个临时的消息对象用于流式显示
    const assistantMessage: Message = {
      id: (Date.now() + 1).toString(),
      content: '',
      role: 'assistant',
      timestamp: Date.now()
    };
    currentConversation.value.messages.push(assistantMessage);
    
    // 根据模型类型选择不同的 API 端点
    let sseUrl = '';
    let headers = new Headers();
    let body = null;
    
    if (currentModel.value === 'openai') {
      sseUrl = `${apiUrl}${method}`;
      headers.append('Content-Type', 'application/json');
      headers.append('Authorization', `Bearer ${sessionKey}`);
      
      body = JSON.stringify({
        model: modelName,
        messages: [
          { role: 'user', content: userMessage.content }
        ],
        stream: true
      });
    } else if (currentModel.value === 'ollama') {
      sseUrl = `${apiUrl}${method}`;
      headers.append('Content-Type', 'application/json');
      
      body = JSON.stringify({
        model: modelName,
        messages: [
          { role: 'user', content: userMessage.content }
        ],
        stream: true
      });
    } else {
      // 自定义模型，使用通用格式
      sseUrl = `${apiUrl}${method}`;
      headers.append('Content-Type', 'application/json');
      if (sessionKey) {
        headers.append('Authorization', `Bearer ${sessionKey}`);
      }
      
      body = JSON.stringify({
        model: modelName,
        messages: [
          { role: 'user', content: userMessage.content }
        ],
        stream: true
      });
    }
    
    // 使用 fetch 创建 SSE 连接
    const response = await fetch(sseUrl, {
      method: 'POST',
      headers: headers,
      body: body
    });
    
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    
    const reader = response.body?.getReader();
    if (!reader) {
      throw new Error('无法获取响应流');
    }
    
    // 处理流式响应
    const decoder = new TextDecoder();
    let buffer = '';
    
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      
      buffer += decoder.decode(value, { stream: true });
      
      // 处理接收到的数据
      const lines = buffer.split('\n');
      buffer = lines.pop() || '';
      
      for (const line of lines) {
        // 检查是否是结束标记
        if (line === 'data: [DONE]') {
          console.log('收到 SSE 结束标记');
          break; // 结束处理循环
        }
        
        if (line.startsWith('data: ') && line !== 'data: [DONE]') {
          try {
            console.log('收到 SSE 数据:', line);
            const data = JSON.parse(line.substring(6));
            let content = '';
            
            // 记录解析后的数据结构
            console.log('解析后的数据:', JSON.stringify(data));
            if (currentModel.value === 'openai') {
              // OpenAI 格式
              if (data.choices && data.choices[0].delta && data.choices[0].delta.content) {
                content = data.choices[0].delta.content;
              }
              // 检查是否结束
              if (data.choices && data.choices[0].finish_reason === 'stop') {
                console.log('OpenAI 响应结束');
              }
            } else if (currentModel.value === 'ollama') {
              // Ollama 格式 - 实际上与 OpenAI 格式相同
              if (data.choices && data.choices[0].delta && data.choices[0].delta.content) {
                content = data.choices[0].delta.content;
              }
              // 检查是否结束
              if (data.choices && data.choices[0].finish_reason === 'stop') {
                console.log('Ollama 响应结束');
              }
            } else {
              // 通用格式，尝试提取内容
              content = data.content || data.text || 
                       (data.choices && data.choices[0] && 
                        (data.choices[0].delta?.content || data.choices[0].text)) || '';
            }
            
            if (content) {
              assistantMessage.content += content;
              // 更新消息内容
              const index = currentConversation.value.messages.findIndex(m => m.id === assistantMessage.id);
              if (index !== -1) {
                currentConversation.value.messages[index] = { ...assistantMessage };
                // 添加延时滚动，确保内容更新后再滚动
                setTimeout(scrollToBottom, 0);
              }
            }
          } catch (e) {
            console.error('解析 SSE 数据失败:', e, line);
          }
        }
      }
    }
  } catch (error) {
    console.error('发送消息失败:', error);
    // 如果 SSE 失败，回退到普通的 API 调用
    try {
      const response = await invoke('chat_with_model', { 
        message: userMessage.content,
        model: currentConversation.value.model
      });
      
      // 更新最后一条消息的内容
      if (currentConversation.value.messages.length > 0) {
        const lastMessage = currentConversation.value.messages[currentConversation.value.messages.length - 1];
        if (lastMessage.role === 'assistant') {
          lastMessage.content = response as string;
        } else {
          // 如果最后一条不是助手消息，则添加新消息
          const assistantMessage: Message = {
            id: (Date.now() + 1).toString(),
            content: response as string,
            role: 'assistant',
            timestamp: Date.now()
          };
          currentConversation.value.messages.push(assistantMessage);
        }
      }
    } catch (fallbackError) {
      console.error('回退到普通 API 调用也失败:', fallbackError);
    }
  } finally {
    loading.value = false;
    saveConversations();
    // 消息发送完成后再次滚动到底部
    setTimeout(scrollToBottom, 100);
  }
}

// 用于跟踪哪些思考内容被展开
const expandedThinks = ref<Set<string>>(new Set());

// 检查消息内容是否包含 <think> 标签
function hasThinkTag(content: string): boolean {
  return content.includes('<think>') && content.includes('</think>');
}

// 格式化包含 <think> 标签的内容
function formatThinkContent(content: string): string {
  // 将消息内容分为 <think> 标签内外两部分
  const thinkRegex = /<think>([\s\S]*?)<\/think>/g;
  let formattedContent = content;
  
  // 替换所有 <think> 标签为可折叠的 HTML
  formattedContent = formattedContent.replace(thinkRegex, '<div class="think-tag-content">$1</div>');
  
  return formattedContent;
}

// 切换思考内容的展开/折叠状态
function toggleThink(messageId: string): void {
  if (expandedThinks.value.has(messageId)) {
    expandedThinks.value.delete(messageId);
  } else {
    expandedThinks.value.add(messageId);
  }
}

// 检查特定消息的思考内容是否展开
function isThinkExpanded(messageId: string): boolean {
  return expandedThinks.value.has(messageId);
}

// 处理消息显示，将 <think> 标签内容替换为折叠区域
// 在 processMessageContent 函数中添加格式化处理
function processMessageContent(msg: Message): { normalContent: string, thinkContent: string | null } {
  let content = msg.content;
  
  // 处理数学公式块
  content = content.replace(/\$\$(.*?)\$\$/g, (match, formula) => {
    return `\n${formula.trim()}\n`;
  });
  
  // 处理行内数学公式
  content = content.replace(/\$(.*?)\$/g, (match, formula) => {
    return `『${formula.trim()}』`;
  });
  
  // 处理对齐块 - 优化间距
  content = content.replace(/\\begin\{align\*\}([\s\S]*?)\\end\{align\*\}/g, (match, block) => {
    const lines = block.split('\n')
      .map(line => line.trim())
      .filter(line => line)
      .map(line => line.replace(/\\\\$/, ''))
      .map(line => line.replace(/\\/, ''));
    return lines.join('\n');  // 移除额外的换行
  });
  
  // 处理水平线
  content = content.replace(/\\hline/g, '──────');
  
  // 处理盒子
  content = content.replace(/\\boxed\{(.*?)\}/g, '[$1]');
  
  // 处理加粗文本，保持紧凑
  content = content.replace(/\*\*(.*?)\*\*/g, '[$1]');
  
  // 处理步骤编号，优化格式
  content = content.replace(/(\d+)\.\s*[(.*?)]/g, '$1：$2');
  
  // 移除多余的空行，只保留单个空行
  content = content.replace(/\n{2,}/g, '\n');
  
  // 处理空的『』对
  content = content.replace(/『\s*』/g, '').replace(/『\s*/g, '').replace(/』\s*/g, '');
  
  // 确保段落之间只有一个空行
  content = content.split('\n')
    .reduce((lines, line) => {
      const currentLine = line.trim();
      const lastLine = lines[lines.length - 1] || '';
      
      // 如果当前行不为空，直接添加
      if (currentLine) {
        lines.push(currentLine);
      }
      // 如果当前行为空且上一行不为空，添加一个空行
      else if (lastLine) {
        lines.push('');
      }
      return lines;
    }, [] as string[])
    .join('\n')
    .replace(/\n{3,}/g, '\n\n'); // 确保没有连续的多个空行
  
  if (!hasThinkTag(content)) {
    return { normalContent: content, thinkContent: null };
  }
  
  const thinkRegex = /<think>([\s\S]*?)<\/think>/;
  const match = content.match(thinkRegex);
  
  if (!match) {
    return { normalContent: content, thinkContent: null };
  }
  
  const thinkContent = match[1].trim();
  const normalContent = content.replace(thinkRegex, '').trim();
  
  return { normalContent, thinkContent };
}
</script>

<template>
  <div class="chat-container">
    <!-- 对话列表侧边栏 -->
    <div class="conversations-sidebar">
      <button class="new-chat-btn" @click="createNewConversation">
        新建对话
      </button>
      <div class="conversations-list">
        <div
          v-for="conv in conversations"
          :key="conv.id"
          class="conversation-item"
          :class="{ active: currentConversation?.id === conv.id }"
          @click="currentConversation = conv; currentModel = conv.model"
        >
          <span class="conversation-title">{{ conv.title }}</span>
          <button class="delete-btn" @click.stop="deleteConversation(conv)">
            删除
          </button>
        </div>
      </div>
    </div>

    <!-- 聊天主区域 -->
    <div class="chat-main">
      <!-- 模型选择器 -->
      <div class="model-selector" v-if="currentConversation">
        <label>选择模型：</label>
        <select 
          v-model="currentModel"
          @change="changeModel(currentModel)"
        >
          <option v-for="model in availableModels" :key="model" :value="model">
            {{ model }}
          </option>
        </select>
        
        <!-- 模型选项选择器 - 对所有模型都显示 -->
        <div class="model-options" v-if="modelOptions[currentModel]">
          <label>模型版本：</label>
          <div class="model-options-list">
            <div 
              v-for="option in modelOptions[currentModel]" 
              :key="option"
              class="model-option-item"
              :class="{ active: selectedModelOption === option }"
              @click="selectModelOption(option)"
            >
              <input 
                type="radio" 
                :id="option" 
                :value="option" 
                v-model="selectedModelOption"
              >
              <label :for="option">{{ option }}</label>
            </div>
          </div>
        </div>
      </div>

      <div v-if="currentConversation" class="chat-messages">
        <div
          v-for="msg in currentConversation.messages"
          :key="msg.id"
          class="message"
          :class="msg.role"
        >
          <div class="message-content" v-if="!hasThinkTag(msg.content)">{{ msg.content }}</div>
          <div class="message-content" v-else>
            <div>{{ processMessageContent(msg).normalContent }}</div>
            <div class="think-container">
              <div class="think-header" @click="toggleThink(msg.id)">
                思考过程 <span class="toggle-icon">{{ isThinkExpanded(msg.id) ? '▼' : '►' }}</span>
              </div>
              <div class="think-body" v-show="isThinkExpanded(msg.id)">
                {{ processMessageContent(msg).thinkContent }}
              </div>
            </div>
          </div>
          <div class="message-time">
            {{ new Date(msg.timestamp).toLocaleTimeString() }}
          </div>
        </div>
      </div>
      <div v-else class="no-conversation">
        请选择或创建一个新对话
      </div>

      <!-- 输入区域 -->
      <div class="input-area" v-if="currentConversation">
        <textarea
          v-model="newMessage"
          @keyup.enter.exact="sendMessage"
          placeholder="输入消息，按Enter发送"
          :disabled="loading"
        ></textarea>
        <button 
          @click="sendMessage" 
          :disabled="loading || !newMessage.trim() || !currentConversation || !currentModelConfig"
        >
          {{ loading ? '发送中...' : '发送' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.chat-container {
  display: flex;
  height: 100%;
  background-color: #ffffff;
}

.conversations-sidebar {
  width: 250px;
  border-right: 1px solid #e0e0e0;
  display: flex;
  flex-direction: column;
  background-color: #f5f5f5;
}

.new-chat-btn {
  margin: 16px;
  padding: 10px;
  background-color: #42b983;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.new-chat-btn:hover {
  background-color: #3aa876;
}

.conversations-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.conversation-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px;
  margin: 4px 0;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.conversation-item:hover {
  background-color: #e0e0e0;
}

.conversation-item.active {
  background-color: #42b98333;
}

.delete-btn {
  padding: 4px 8px;
  background-color: #ff4444;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.2s;
}

.conversation-item:hover .delete-btn {
  opacity: 1;
}

.chat-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: #ffffff;
}

.model-selector {
  padding: 12px 16px;
  border-bottom: 1px solid #e0e0e0;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 12px;
  background-color: #f8f8f8;
}

.model-selector select {
  padding: 6px 12px;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  background-color: white;
  min-width: 120px;
  font-size: 14px;
}

.model-options {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}
  
.model-options-list {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.model-option-item {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  cursor: pointer;
  background-color: white;
  transition: all 0.2s;
}

.model-option-item:hover {
  background-color: #f0f0f0;
}

.model-option-item.active {
  background-color: #42b98333;
  border-color: #42b983;
}

.model-option-item input[type="radio"] {
  margin: 0;
}

.model-option-item label {
  cursor: pointer;
  font-size: 14px;
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  scroll-behavior: smooth; /* 添加平滑滚动效果 */
}

.message {
  margin: 12px 0;
  max-width: 80%;
  display: flex;
  flex-direction: column;
}

.message.user {
  margin-left: auto;
  align-items: flex-end;
}

.message.assistant {
  margin-right: auto;
  align-items: flex-start;
}

.message-content {
  padding: 12px 16px;
  border-radius: 12px;
  background-color: #f0f0f0;
  line-height: 1.4;
  white-space: pre-wrap;
  word-break: break-word;
  display: inline-block;
  max-width: 100%;
  min-width: 60px;  /* 添加最小宽度 */
}

.message.user .message-content {
  background-color: #42b983;
  color: white;
  text-align: right;
}

.message.assistant .message-content {
  text-align: left;
}

.message-time {
  font-size: 12px;
  color: #666;
  margin-top: 4px;
  text-align: right;
}

.input-area {
  padding: 20px;
  border-top: 1px solid #e0e0e0;
  display: flex;
  gap: 10px;
}

textarea {
  flex: 1;
  padding: 12px;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  resize: none;
  height: 60px;
  font-family: inherit;
}

.input-area button {
  padding: 0 20px;
  background-color: #42b983;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.input-area button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

.no-conversation {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #666;
  font-size: 1.2em;
}

@media (prefers-color-scheme: dark) {
  .chat-container {
    background-color: #1a1a1a;
  }

  .conversations-sidebar {
    background-color: #2a2a2a;
    border-right-color: #333;
  }

  .conversation-item:hover {
    background-color: #333;
  }

  .conversation-item.active {
    background-color: #42b98322;
  }

  .chat-main {
    background-color: #1a1a1a;
  }

  .model-selector {
    border-bottom-color: #333;
  }

  .model-selector select {
    background-color: #333;
    border-color: #444;
    color: #fff;
  }

  .message-content {
    background-color: #333;
    color: #fff;
  }

  .message.user .message-content {
    background-color: #42b983;
  }

  .message-time {
    color: #999;
  }

  .input-area {
    border-top-color: #333;
  }

  textarea {
    background-color: #333;
    border-color: #444;
    color: #fff;
  }

  .no-conversation {
    color: #999;
  }
}
.think-container {
  margin-top: 8px;
  border-left: 3px solid #42b983;
  border-radius: 6px;
  overflow: hidden;
}

.think-header {
  padding: 8px;
  background-color: rgba(66, 185, 131, 0.1);
  cursor: pointer;
  font-weight: bold;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.think-body {
  padding: 12px;
  white-space: pre-wrap;
  background-color: rgba(66, 185, 131, 0.05);
  word-break: break-word;
}

.toggle-icon {
  font-size: 12px;
}

.model-options {
  display: flex;
  align-items: center;
  margin-left: 20px;
  gap: 10px;
}

.model-options-list {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.model-option-item {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.model-option-item:hover {
  background-color: #f0f0f0;
}

.model-option-item.active {
  background-color: #42b98333;
}
.message-content {
  white-space: pre-wrap;
  word-break: break-word;
}

/* 添加数学公式样式 */
.math-formula {
  margin: 1em 0;
  padding: 0.5em;
  background-color: rgba(66, 185, 131, 0.05);
  border-left: 3px solid #42b983;
  font-family: monospace;
}
  
.message-content {
  padding: 12px 16px;
  border-radius: 12px;
  background-color: #f0f0f0;
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
}

/* 添加数学公式样式 */
.math-block {
  margin: 1em 0;
  padding: 0.8em;
  background-color: rgba(66, 185, 131, 0.05);
  border-left: 3px solid #42b983;
  font-family: monospace;
}

/* 添加计算过程样式 */
.calculation-process {
  margin: 1em 0;
  padding: 0.8em;
  background-color: rgba(66, 185, 131, 0.05);
  border-radius: 4px;
  font-family: monospace;
}

/* 添加步骤样式 */
.step {
  margin: 0.5em 0;
  padding-left: 1em;
  border-left: 2px solid #42b983;
}

/* 添加结果样式 */
.result {
  margin: 1em 0;
  padding: 0.5em;
  background-color: rgba(66, 185, 131, 0.1);
  border-radius: 4px;
  font-weight: bold;
}

/* 暗色模式适配 */
@media (prefers-color-scheme: dark) {
  .math-block,
  .calculation-process {
    background-color: rgba(66, 185, 131, 0.1);
  }
  
  .result {
    background-color: rgba(66, 185, 131, 0.15);
  }
}
</style>