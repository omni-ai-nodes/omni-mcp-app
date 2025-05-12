<script setup lang="ts">
import {
  conversations,
  currentConversation,
  newMessage,
  loading,
  availableModels,
  currentModel,
  modelConfigs,
  streamingContent,
  eventSource,
  modelOptions,
  selectedModelOption,
  expandedThinks,
  saveModelState,
  loadCustomConfigs,
  currentModelConfig,
  loadMcpServers,
  saveConversations,
  createNewConversation,
  deleteConversation,
  changeModel,
  selectModelOption,
  closeEventSource,
  scrollToBottom,
  sendMessage,
  hasThinkTag,
  formatThinkContent,
  toggleThink,
  isThinkExpanded,
  mcpClient,
  mcpServers,
  selectedMcpServers,
  isMcpConnected,  // Make sure this is included
  processMessageContent,  // Add this import
  initMcpClient,         // Add this import
  mcpLoading,      // Add this import  processMessageContent
  connectToMcpServers,  // 添加这一行
  disconnectMcp,        // 添加这一行
  stopSending,
} from './GreeterChat.ts';
import { onMounted as vueOnMounted, onUnmounted } from 'vue'

// 组件挂载时初始化
// 从本地存储加载对话记录
vueOnMounted(async () => {
  await loadCustomConfigs()
  await loadMcpServers()
  const savedConversations = localStorage.getItem('chatConversations');
  if (savedConversations) {
    conversations.value = JSON.parse(savedConversations);
    // 为旧的对话添加模型字段
    conversations.value = conversations.value.map(conv => ({
      ...conv,
      model: conv.model || currentModel.value // 使用当前选择的模型作为默认值
    }));
    if (conversations.value.length > 0) {
      currentConversation.value = conversations.value[0];
    }
  }
  
  // 初始化 MCP 客户端
  await initMcpClient();
})

// 组件卸载时断开 MCP 连接
onUnmounted(() => {
  disconnectMcp();
})
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
          @click="() => {
            currentConversation = conv;
            currentModel = conv.model;
            // 使用 scrollToBottom 函数，它内部已经使用了 nextTick
            scrollToBottom();
          }"
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
            <div class="think-container" v-if="processMessageContent(msg).thinkContent">
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
        <div class="mcp-servers-selector">
          <div class="mcp-servers-list">
            <div class="mcp-servers-label">MCP Server:</div>
            <div 
              v-for="server in mcpServers" 
              :key="server.server_name"
              class="mcp-server-item"
            >
              <input 
                type="checkbox"
                :id="server.server_name"
                :value="server.server_name"
                v-model="selectedMcpServers"
              >
              <label :for="server.server_name">{{ server.server_name }}</label>
            </div>
          </div>
          <div v-if="selectedMcpServers.length > 0" class="mcp-connect-button">
          <button 
            @click="connectToMcpServers" 
            :disabled="isMcpConnected || mcpLoading"
          >
            {{ mcpLoading ? '连接中...' : (isMcpConnected ? '已连接' : '连接') }}
          </button>
        </div>
        </div>
        <div class="message-input">
          <textarea
            v-model="newMessage"
            @keyup.enter.exact="sendMessage"
            placeholder="输入消息，按Enter发送"
            :disabled="loading"
          ></textarea>
          <button 
            @click="loading ? stopSending() : sendMessage()" 
            :disabled="(!newMessage.trim() || !currentConversation || !currentModelConfig) && !loading"
          >
            {{ loading ? '停止' : '发送' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
<style lang="scss" scoped>
@use './GreeterChat.scss';
</style>
