import { ref, onMounted, computed, nextTick } from 'vue';
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

// Define MCPClient class once at the top
class MCPClient {
  private servers: string[] = [];

  async connectToServer(serverName: string) {
    try {
      // 暂时移除 Tauri 命令调用，使用模拟实现
      this.servers.push(serverName);
      console.log(`Connected to server: ${serverName}`);
    } catch (error) {
      console.error(`Failed to connect to server ${serverName}:`, error);
      throw error;
    }
  }

  async disconnect() {
    try {
      // 暂时移除 Tauri 命令调用，使用模拟实现
      this.servers = [];
      console.log('Disconnected from all servers');
    } catch (error) {
      console.error('Failed to disconnect:', error);
      throw error;
    }
  }

  async processQuery(query: string, callback: (content: string) => void) {
    try {
      if (this.servers.length === 0) {
        callback('错误：未连接到任何 MCP 服务器');
        return;
      }

      // 遍历所有已连接的服务器
      for (const serverName of this.servers) {
        try {
          // 使用 execute_command 来处理查询
          const response = await invoke('execute_command', {
            cmd: 'mcp',
            args: ['query', serverName, query]
          });
          
          if (typeof response === 'string') {
            callback(response);
          } else {
            console.error('意外的响应类型:', response);
            callback('处理查询时出现错误：响应格式不正确');
          }
        } catch (serverError) {
          console.error(`服务器 ${serverName} 处理查询失败:`, serverError);
          callback(`服务器 ${serverName} 处理查询失败: ${serverError.message}`);
        }
      }
    } catch (error) {
      console.error('处理查询失败:', error);
      callback(`处理查询时出现错误: ${error.message}`);
    }
  }
}

// 确保所有必要的变量和函数都已正确导出
export const newMessage = ref('');
export const loading = ref(false);
export const availableModels = ref(['openai', 'ollama']); // 改为响应式数组
export const currentModel = ref('openai');
export const modelConfigs = ref<Record<string, ModelConfig>>({});
export const streamingContent = ref('');
export const eventSource = ref<EventSource | null>(null);
export const modelOptions = ref<Record<string, string[]>>({});
export const selectedModelOption = ref<string>('');

// 添加或确保这些变量的声明
export const mcpClient = ref<MCPClient | null>(null);
export const mcpServers = ref<string[]>([]);
export const selectedMcpServers = ref<string[]>([]);
export const isMcpConnected = ref(false);
export const mcpLoading = ref(false);
export const conversations = ref<Conversation[]>([]);
export const currentConversation = ref<Conversation | null>(null);
// 添加新的函数用于保存模型状态
export function saveModelState() {
  localStorage.setItem('currentModel', currentModel.value);
  localStorage.setItem('selectedModelOption', selectedModelOption.value);
}

export async function initMcpClient() {
  try {
    mcpLoading.value = true;
    // Initialize any MCP-related setup here
    await loadMcpServers(); // Reload server list
    mcpLoading.value = false;

    // If mcpClient is already initialized, return
    if (mcpClient.value) {
      console.log('MCP client already initialized');
      return;
    }
    
    console.log('Initializing MCP client...');
    // Create a new instance of MCPClient
    mcpClient.value = new MCPClient();
    
    console.log('MCP client initialized successfully');
  } catch (error) {
    console.error('Failed to initialize MCP client:', error);
    mcpLoading.value = false;
    throw error; // Rethrow the error to handle it in the calling code
  }
}

// 加载自定义模型配置
export async function loadCustomConfigs() {
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
        modelOptions.value[config.provider] = [config.model];
      }
    });
    
    // 从 localStorage 恢复模型状态
    const savedModel = localStorage.getItem('currentModel');
    const savedModelOption = localStorage.getItem('selectedModelOption');
    
    if (savedModel && availableModels.value.includes(savedModel)) {
      currentModel.value = savedModel;
      if (savedModelOption && modelOptions.value[savedModel]?.includes(savedModelOption)) {
        selectedModelOption.value = savedModelOption;
      } else if (modelOptions.value[savedModel]?.length > 0) {
        selectedModelOption.value = modelOptions.value[savedModel][0];
      }
    }
    
    console.log('加载的模型配置:', modelConfigs.value);
    console.log('模型选项:', modelOptions.value);
  } catch (error) {
    console.error('加载自定义模型配置失败:', error);
  }
}

// 获取当前模型的配置
export const currentModelConfig = computed(() => {
  return modelConfigs.value[currentModel.value] || null;
});

// 加载 MCP 服务器列表
export  async function loadMcpServers() {
  try {
    const servers = await invoke('get_all_mcp_servers');
    mcpServers.value = servers;
  } catch (error) {
    console.error('加载MCP服务器列表失败:', error);
  }
}

// 保存对话记录到本地存储
export function saveConversations() {
  localStorage.setItem('chatConversations', JSON.stringify(conversations.value));
}

// 创建新对话
export function createNewConversation() {
    const newConv: Conversation = {
        id: Date.now().toString(),
        title: '新对话',
        messages: [],
        timestamp: Date.now(),
        model: currentModel.value
    };
    conversations.value.unshift(newConv);
    currentConversation.value = newConv;
    
    // 在保存对话前加载最新配置
    loadCustomConfigs().then(() => {
        saveConversations();
    }).catch(error => {
        console.error('加载配置失败:', error);
        saveConversations();
    });
}

// 删除对话
export function deleteConversation(conv: Conversation) {
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
export function changeModel(model: string) {
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
    saveModelState(); // 保存模型状态
  }
}

// 选择具体的模型选项
export function selectModelOption(option: string) {
  selectedModelOption.value = option;
  saveModelState(); // 保存模型状态
}

// 关闭 SSE 连接
export function closeEventSource() {
  if (eventSource.value) {
    eventSource.value.close();
    eventSource.value = null;
  }
}

// 使用 SSE 发送消息
// 添加一个新的方法来处理滚动
export function scrollToBottom() {
  // 使用 nextTick 确保 DOM 更新后再滚动
  nextTick(() => {
    const chatMessages = document.querySelector('.chat-messages');
    if (chatMessages) {
      chatMessages.scrollTop = chatMessages.scrollHeight;
    }
  });
}

// 在 sendMessage 函数中的适当位置添加滚动调用
export async function sendMessage() {
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
    
    // 如果有选中的 MCP 服务器，则使用 MCP 客户端处理消息
    if (selectedMcpServers.value.length > 0 && mcpClient.value) {
      try {
        // 连接到 MCP 服务器（如果尚未连接）
        // 如果选择了MCP服务器，使用MCP客户端处理查询
        if (mcpClient.value && selectedMcpServers.value.length > 0) {
              // 如果未连接，先尝试连接
              if (!isMcpConnected.value) {
                await connectToMcpServers();
            }
          }
        
        // 使用 MCPClient 处理查询，传入回调函数用于处理消息
        const result = await mcpClient.value.processQuery(
          newMessage.value,
          async (content: string) => {
            // 这里处理 MCP 服务器返回的消息
            const assistantMessage: Message = {
              id: Date.now().toString(),
              content: content,
              role: 'assistant',
              timestamp: Date.now()
            };
            currentConversation.value?.messages.push(assistantMessage);
            saveConversations();
            scrollToBottom();
          }
        );
        
        // 清空输入框
        newMessage.value = '';
        
      } catch (error) {
        console.error('MCP 处理消息失败:', error);
        // 添加错误消息
        const errorMessage: Message = {
          id: Date.now().toString(),
          content: `处理消息时出错: ${error.message}`,
          role: 'assistant',
          timestamp: Date.now()
        };
        currentConversation.value?.messages.push(errorMessage);
        saveConversations();
      }
      
      // 这里需要添加 return 语句，防止继续执行下面的代码
      loading.value = false;
      return;
    }
  
    // 原有的消息处理逻辑
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
                // 直接调用 scrollToBottom，不需要 setTimeout
                scrollToBottom();
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
    // 消息发送完成后再次滚动到底部，直接调用 scrollToBottom
    scrollToBottom();
  }
}

// 用于跟踪哪些思考内容被展开
export const expandedThinks = ref<Set<string>>(new Set());

// 检查消息内容是否包含 <think> 标签
export function hasThinkTag(content: string): boolean {
    // Implement the logic to check for the think tag in the content
    return content.includes('think');
}

// 格式化包含 <think> 标签的内容
export function formatThinkContent(content: string): string {
  // 将消息内容分为 <think> 标签内外两部分
  const thinkRegex = /<think>([\s\S]*?)<\/think>/g;
  let formattedContent = content;
  
  // 替换所有 <think> 标签为可折叠的 HTML
  formattedContent = formattedContent.replace(thinkRegex, '<div class="think-tag-content">$1</div>');
  
  return formattedContent;
}

// 切换思考内容的展开/折叠状态
export function toggleThink(messageId: string): void {
  if (expandedThinks.value.has(messageId)) {
    expandedThinks.value.delete(messageId);
  } else {
    expandedThinks.value.add(messageId);
  }
}

// 检查特定消息的思考内容是否展开
export function isThinkExpanded(messageId: string): boolean {
  return expandedThinks.value.has(messageId);
}

// 处理消息显示，将 <think> 标签内容替换为折叠区域
// 在 processMessageContent 函数中添加格式化处理
export function processMessageContent(msg: Message): { normalContent: string, thinkContent: string | null } {
  let content = msg.content;
  if (!hasThinkTag(content)) {
    return { normalContent: content, thinkContent: null };
  }
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
  
  // 修复正则表达式匹配逻辑
  const thinkRegex = /<think>([\s\S]+?)<\/think>/g;  // 修改为全局匹配
  const matches = [...content.matchAll(thinkRegex)];
  
  if (matches.length === 0) {
    return { normalContent: content, thinkContent: null };
  }

  // 提取所有 think 内容并拼接
  const thinkContents = matches.map(m => m[1].trim());
  const thinkContent = thinkContents.join('\n\n');
  
  // 移除所有 think 标签
  const normalContent = content.replace(thinkRegex, '').trim();

  return { 
    normalContent: normalContent || ' ',  // 避免空内容导致的显示问题
    thinkContent: thinkContent || null 
  };
}

// 连接到选定的MCP服务器
export const connectToMcpServers = async () => {
  if (!mcpClient.value) return;
  
  mcpLoading.value = true;
  try {
    // 断开所有连接
    await mcpClient.value.disconnect();
    isMcpConnected.value = false; // 重置连接状态
    
    // 连接到选定的服务器
    for (const serverName of selectedMcpServers.value) {
      await mcpClient.value.connectToServer(serverName);
    }
    
    isMcpConnected.value = true; // 设置连接状态为已连接
  } catch (error) {
    console.error('连接MCP服务器失败:', error);
    isMcpConnected.value = false; // 连接失败时重置状态
  } finally {
    mcpLoading.value = false;
  }
};


// 断开MCP连接
export const disconnectMcp = async () => {
  if (mcpClient.value) {
    try {
      await mcpClient.value.disconnect();
      isMcpConnected.value = false;
    } catch (error) {
      console.error('断开MCP连接失败:', error);
    }
  }
};

