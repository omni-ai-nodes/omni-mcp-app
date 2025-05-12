import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport, StdioServerParameters } from "@modelcontextprotocol/sdk/client/stdio.js";
import { SSEClientTransport } from "@modelcontextprotocol/sdk/client/sse.js";
import { Tool } from "@modelcontextprotocol/sdk/types.js";
import { homedir } from 'os';
import config from "./mcp-server-config.js";

interface MCPToolResult {
    content: string;
}

interface ServerConfig {
    name: string;
    type: 'command' | 'sse';
    command?: string;
    url?: string;
    isOpen?: boolean;
}

export class MCPClient {
    static getOpenServers(): string[] {
        return config.filter(cfg => cfg.isOpen).map(cfg => cfg.name);
    }

    private sessions: Map<string, Client> = new Map();
    private transports: Map<string, StdioClientTransport | SSEClientTransport> = new Map();

    constructor() {
        // 移除 OpenAI 初始化
    }

    async connectToServer(serverName: string): Promise<void> {
        const serverConfig = config.find(cfg => cfg.name === serverName) as ServerConfig;
        if (!serverConfig) {
            throw new Error(`Server configuration not found for: ${serverName}`);
        }

        let transport: StdioClientTransport | SSEClientTransport;
        if (serverConfig.type === 'command' && serverConfig.command) {
            transport = await this.createCommandTransport(serverConfig.command);
        } else if (serverConfig.type === 'sse' && serverConfig.url) {
            transport = await this.createSSETransport(serverConfig.url);
        } else {
            throw new Error(`Invalid server configuration for: ${serverName}`);
        }

        const client = new Client(
            {
                name: "mcp-client",
                version: "1.0.0"
            },
            {
                capabilities: {
                    prompts: {},
                    resources: {},
                    tools: {}
                }
            }
        );
        await client.connect(transport);
        
        this.sessions.set(serverName, client);
        this.transports.set(serverName, transport);
        // 列出可用工具
        const response = await client.listTools();
        console.log(`\nConnected to server '${serverName}' with tools:`, response.tools.map((tool: Tool) => tool.name));
    }

    private async createCommandTransport(shell: string): Promise<StdioClientTransport> {
        const [command, ...shellArgs] = shell.split(' ');
        if (!command) {
            throw new Error("Invalid shell command");
        }
        // 处理参数中的波浪号路径
        const args = shellArgs.map(arg => {
            if (arg.startsWith('~/')) {
                return arg.replace('~', homedir());
            }
            return arg;
        });
        
        const serverParams: StdioServerParameters = {
            command,
            args,
            env: Object.fromEntries(
                Object.entries(process.env).filter(([_, v]) => v !== undefined)
            ) as Record<string, string>
        };
        return new StdioClientTransport(serverParams);
    }

    private async createSSETransport(url: string): Promise<SSEClientTransport> {
        return new SSEClientTransport(new URL(url));
    }

    async processQuery(query: string, sendMessageCallback: (content: string) => Promise<void>): Promise<string> {
        if (this.sessions.size === 0) {
            throw new Error("未连接到任何服务器");
        }

        // 获取所有服务器的工具列表
        const availableTools: any[] = [];
        for (const [serverName, session] of this.sessions) {
            const response = await session.listTools();
            const tools = response.tools.map((tool: Tool) => ({
                type: "function" as const,
                function: {
                    name: `${serverName}__${tool.name}`,
                    description: `[${serverName}] ${tool.description}`,
                    parameters: tool.inputSchema
                }
            }));
            availableTools.push(...tools);
        }

        // 使用回调函数发送消息，而不是直接调用 OpenAI API
        await sendMessageCallback(JSON.stringify({
            tools: availableTools,
            query: query
        }));

        const finalText: string[] = [];
        
        // 这里不再处理 OpenAI 的响应，而是由外部处理
        // 返回工具列表信息作为参考
        return `已连接到 ${this.sessions.size} 个 MCP 服务器，共有 ${availableTools.length} 个可用工具`;
    }
    
    async callTool(toolName: string, toolArgs: any): Promise<MCPToolResult> {
        const [serverName, actualToolName] = toolName.split('__');
        const session = this.sessions.get(serverName);
        
        if (!session) {
            throw new Error(`服务器 ${serverName} 未找到`);
        }
        
        try {
            // 执行工具调用
            const result = await session.callTool({
                name: actualToolName,
                arguments: toolArgs
            });
            
            // 返回工具调用结果
            return result as MCPToolResult;
        } catch (error) {
            throw new Error(`调用工具 ${serverName}:${actualToolName} 失败: ${error.message}`);
        }
    }
    
    // 关闭所有连接
    async disconnect(): Promise<void> {
        for (const [serverName, transport] of this.transports) {
            try {
                await transport.close();
                console.log(`Disconnected from server: ${serverName}`);
            } catch (error) {
                console.error(`Error disconnecting from server ${serverName}:`, error);
            }
        }
        
        this.sessions.clear();
        this.transports.clear();
    }
}