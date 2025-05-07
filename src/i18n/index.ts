import { createI18n } from 'vue-i18n'

const messages = {
  zh: {
    message: {
      chat: '聊天',
      mcpService: 'MCP服务安装',
      parsingMCPConfigService: '解析MCP配置',
      toolManagement: '工具管理',
      settings: '设置',
      github: 'GitHub',
      modelSettings: '模型设置',
      generalSettings: '常规设置',
      about: '关于我们',
      greeting: '问候功能',
      enterName: '输入名字...',
      greet: '问候',
      commandExecution: '命令执行',
      enterCommand: '输入命令...',
      parameters: '参数（可选）',
      executing: '执行中...',
      execute: '执行',
      commandHistory: '命令历史',
      executionResult: '执行结果',
      executingCommand: '命令执行中...'
    }
  },
  'zh-TW': {
    message: {
      chat: '聊天',
      mcpService: 'MCP服務安裝',
      toolManagement: '工具管理',
      settings: '設置',
      github: 'GitHub',
      modelSettings: '模型設置',
      generalSettings: '常規設置',
      about: '關於我們',
      greeting: '問候功能',
      enterName: '輸入名字...',
      greet: '問候',
      commandExecution: '命令執行',
      enterCommand: '輸入命令...',
      parameters: '參數（可選）',
      executing: '執行中...',
      execute: '執行',
      commandHistory: '命令歷史',
      executionResult: '執行結果',
      executingCommand: '命令執行中...'
    }
  },
  ja: {
    message: {
      chat: 'チャット',
      mcpService: 'MCPサービスインストール',
      toolManagement: 'ツール管理',
      settings: '設定',
      github: 'GitHub',
      modelSettings: 'モデル設定',
      generalSettings: '一般設定',
      about: '私たちについて',
      greeting: '挨拶機能',
      enterName: '名前を入力...',
      greet: '挨拶',
      commandExecution: 'コマンド実行',
      enterCommand: 'コマンドを入力...',
      parameters: 'パラメータ（オプション）',
      executing: '実行中...',
      execute: '実行',
      commandHistory: 'コマンド履歴',
      executionResult: '実行結果',
      executingCommand: 'コマンド実行中...'
    }
  },
  ko: {
    message: {
      chat: '채팅',
      mcpService: 'MCP 서비스 설치',
      toolManagement: '도구 관리',
      settings: '설정',
      github: 'GitHub',
      modelSettings: '모델 설정',
      generalSettings: '일반 설정',
      about: '회사 소개',
      greeting: '인사 기능',
      enterName: '이름 입력...',
      greet: '인사',
      commandExecution: '명령어 실행',
      enterCommand: '명령어 입력...',
      parameters: '매개변수 (선택사항)',
      executing: '실행 중...',
      execute: '실행',
      commandHistory: '명령어 기록',
      executionResult: '실행 결과',
      executingCommand: '명령어 실행 중...'
    }
  },
  fr: {
    message: {
      chat: 'Chat',
      mcpService: 'Installation du Service MCP',
      toolManagement: 'Gestion des Outils',
      settings: 'Paramètres',
      github: 'GitHub',
      modelSettings: 'Paramètres du Modèle',
      generalSettings: 'Paramètres Généraux',
      about: 'À Propos',
      greeting: 'Fonction de Salutation',
      enterName: 'Entrez le nom...',
      greet: 'Saluer',
      commandExecution: 'Exécution de Commande',
      enterCommand: 'Entrez la commande...',
      parameters: 'Paramètres (Optionnel)',
      executing: 'Exécution en cours...',
      execute: 'Exécuter',
      commandHistory: 'Historique des Commandes',
      executionResult: 'Résultat de l\'Exécution',
      executingCommand: 'Commande en cours d\'exécution...'
    }
  },
  en: {
    message: {
      chat: 'Chat',
      mcpService: 'MCP Service Installation',
      toolManagement: 'Tool Management',
      settings: 'Settings',
      github: 'GitHub',
      modelSettings: 'Model Settings',
      generalSettings: 'General Settings',
      about: 'About Us',
      greeting: 'Greeting',
      enterName: 'Enter name...',
      greet: 'Greet',
      commandExecution: 'Command Execution',
      enterCommand: 'Enter command...',
      parameters: 'Parameters (Optional)',
      executing: 'Executing...',
      execute: 'Execute',
      commandHistory: 'Command History',
      executionResult: 'Execution Result',
      executingCommand: 'Executing command...'
    }
  },
  la: {
    message: {
      chat: 'Colloquium',
      mcpService: 'MCP Servitii Installatio',
      toolManagement: 'Instrumentorum Administratio',
      settings: 'Configurationes',
      github: 'GitHub',
      modelSettings: 'Exemplaris Configurationes',
      generalSettings: 'Generales Configurationes',
      about: 'De Nobis',
      greeting: 'Salutatio Functio',
      enterName: 'Nomen inducere...',
      greet: 'Salutare',
      commandExecution: 'Mandati Executio',
      enterCommand: 'Mandatum inducere...',
      parameters: 'Parametra (Optionalia)',
      executing: 'Executio in progressu...',
      execute: 'Exsequi',
      commandHistory: 'Historia Mandatorum',
      executionResult: 'Executionis Resultatum',
      executingCommand: 'Mandatum in executione...'
    },
    checkingTools: '检查工具状态',
    checkToolsFailed: '检查工具状态失败',
    appLoaded: '应用已加载',
    installing: '正在安装',
    installSuccess: '安装成功',
    installFailed: '安装失败',
    installResult: '安装结果',
    notInstalled: '未安装',
    installed: '已安装'
  }
}

export const i18n = createI18n({
  legacy: false,
  locale: 'zh',
  fallbackLocale: 'en',
  messages
})