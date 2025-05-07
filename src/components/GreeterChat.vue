<script setup lang="ts">
import { ref, onMounted } from 'vue';
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

const conversations = ref<Conversation[]>([]);
const currentConversation = ref<Conversation | null>(null);
const newMessage = ref('');
const loading = ref(false);
const availableModels = ref(['openai', 'ollama']); // 改为响应式数组
const currentModel = ref('openai');

// 加载自定义模型配置
async function loadCustomConfigs() {
  try {
    const configs = await invoke('get_custom_configs', { filterType: 'ALL' });
    const customModels = (configs as any[]).map(config => config.provider);
    availableModels.value = [...customModels];
  } catch (error) {
    console.error('加载自定义模型配置失败:', error);
  }
}

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
    title: `新对话 ${conversations.value.length + 1}`,
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
    saveConversations();
  }
}

// 发送消息
async function sendMessage() {
  if (!newMessage.value.trim() || !currentConversation.value || loading.value) return;
  
  const userMessage: Message = {
    id: Date.now().toString(),
    content: newMessage.value,
    role: 'user',
    timestamp: Date.now()
  };
  
  currentConversation.value.messages.push(userMessage);
  newMessage.value = '';
  loading.value = true;
  
  try {
    const response = await invoke('chat_with_model', { 
      message: userMessage.content,
      model: currentConversation.value.model
    });
    const assistantMessage: Message = {
      id: (Date.now() + 1).toString(),
      content: response as string,
      role: 'assistant',
      timestamp: Date.now()
    };
    currentConversation.value.messages.push(assistantMessage);
  } catch (error) {
    console.error('发送消息失败:', error);
  } finally {
    loading.value = false;
    saveConversations();
  }
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
      </div>

      <div v-if="currentConversation" class="chat-messages">
        <div
          v-for="msg in currentConversation.messages"
          :key="msg.id"
          class="message"
          :class="msg.role"
        >
          <div class="message-content">{{ msg.content }}</div>
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
        <button @click="sendMessage" :disabled="loading || !newMessage.trim()">
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
  padding: 16px;
  border-bottom: 1px solid #e0e0e0;
  display: flex;
  align-items: center;
  gap: 10px;
}

.model-selector select {
  padding: 8px;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  background-color: white;
  cursor: pointer;
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.message {
  margin: 12px 0;
  max-width: 80%;
}

.message.user {
  margin-left: auto;
}

.message.assistant {
  margin-right: auto;
}

.message-content {
  padding: 12px 16px;
  border-radius: 12px;
  background-color: #f0f0f0;
  line-height: 1.4;
}

.message.user .message-content {
  background-color: #42b983;
  color: white;
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
</style>