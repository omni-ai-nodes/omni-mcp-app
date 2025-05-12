interface ServerConfig {
    name: string;
    type: 'command' | 'sse';
    command?: string;
    url?: string;
    isOpen?: boolean;
}

const config: ServerConfig[] = [
    {
        name: "本地文件工具",
        type: "command",
        command: "node ~/path/to/mcp-server.js",
        isOpen: true
    },
    {
        name: "网络搜索工具",
        type: "sse",
        url: "http://localhost:3000/mcp-sse",
        isOpen: true
    }
    // 可以添加更多服务器配置
];

export default config;