<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
// 导入 Tauri shell API
import { open } from '@tauri-apps/plugin-shell';
import { useI18n } from 'vue-i18n';

const { locale } = useI18n();

const languages = [
  { code: 'zh', name: '简体中文' },
  { code: 'zh-TW', name: '繁體中文' },
  { code: 'ja', name: '日本語' },
  { code: 'ko', name: '한국어' },
  { code: 'fr', name: 'Français' },
  { code: 'en', name: 'English' },
  { code: 'la', name: 'Latin' }
]

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

// 命令执行相关
const cmdInput = ref("");
const cmdArgs = ref("");
const cmdOutput = ref("");
const cmdError = ref("");
const isLoading = ref(false);
const commandHistory = ref<string[]>([]); // 添加命令历史记录数组

// 添加工具状态
const toolsStatus = ref({
  uv: "not Installed!",
  bun: "not Installed!"
});

// 添加设置菜单状态和切换方法
// Add settings submenu state
const showSettingsSubmenu = ref(false);

// Add GitHub open method
async function openGitHub() {
  await invoke("open_github_link");
}

// Add settings toggle method
function toggleSettingsSubmenu() {
  showSettingsSubmenu.value = !showSettingsSubmenu.value;
}

// 添加安装工具的方法
onMounted(async () => {
  console.log($t('message.appLoaded'));
  
  try {
    const status = await invoke("check_tools_status");
    toolsStatus.value = status as typeof toolsStatus.value;
  } catch (error) {
    console.error($t('message.checkToolsFailed'), error);
  }
});

async function installTool(tool: string) {
  try {
    isLoading.value = true;
    cmdError.value = "";
    cmdOutput.value = `${$t('message.installing')} ${tool}...`;
    console.log(`${$t('message.installing')} ${tool}...`);
    
    const result = await invoke("install_single_tool", { tool });
    console.log(`${$t('message.installResult')}:`, result);
    
    toolsStatus.value[tool as keyof typeof toolsStatus.value] = $t('message.installSuccess');
    cmdOutput.value = `${tool} ${$t('message.installSuccess')}！`;
  } catch (error) {
    console.error(`${$t('message.installFailed')}:`, error);
    cmdError.value = `${$t('message.installFailed')}: ${error}`;
    toolsStatus.value[tool as keyof typeof toolsStatus.value] = $t('message.installFailed');
  } finally {
    isLoading.value = false;
  }
}

// 修改语言切换方法的类型定义
// 修改语言切换方法
function changeLanguage(event: Event) {
  const target = event.target as HTMLSelectElement;
  locale.value = target.value;
}
</script>

<template>
  <div class="app-container">
    <nav class="side-nav">
      <div class="nav-links">
        <router-link to="/greeter">
          <i class="icon chat-icon"></i>
          <span>{{ $t('message.chat') }}</span>
        </router-link>
        <router-link to="/">
          <i class="icon server-icon"></i>
          <span>{{ $t('message.mcpService') }}</span>
        </router-link>
        <router-link to="/tools">
          <i class="icon tools-icon"></i>
          <span>{{ $t('message.toolManagement') }}</span>
        </router-link>
      </div>
      
      <div class="bottom-buttons">
        <button @click="openGitHub" class="nav-button">
          <i class="icon github-icon"></i>
          <span>{{ $t('message.github') }}</span>
        </button>
        
        <div class="settings-container">
          <button @click="toggleSettingsSubmenu" class="nav-button">
            <i class="icon settings-icon"></i>
            <span>{{ $t('message.settings') }}</span>
          </button>
          
          <div v-if="showSettingsSubmenu" class="submenu">
            <router-link to="/settings/model">
              <i class="icon model-icon"></i>
              <span>{{ $t('message.modelSettings') }}</span>
            </router-link>
            <router-link to="/settings/general">
              <i class="icon general-icon"></i>
              <span>{{ $t('message.generalSettings') }}</span>
            </router-link>
            <div class="language-selector">
              <i class="icon language-icon"></i>
              <select v-model="locale" @change="changeLanguage">
                <option v-for="lang in languages" :key="lang.code" :value="lang.code">
                  {{ lang.name }}
                </option>
              </select>
            </div>
            <router-link to="/about">
              <i class="icon about-icon"></i>
              <span>{{ $t('message.about') }}</span>
            </router-link>
          </div>
        </div>
      </div>
    </nav>
    <main class="content-container">
      <router-view></router-view>
    </main>
  </div>
</template>

<style>
.app-container {
  display: flex;
  min-height: 100vh;
  height: 100vh; /* 确保容器占满整个视口高度 */
  width: 100vw; /* 确保容器占满整个视口宽度 */
  overflow: hidden; /* 防止出现滚动条 */
  position: fixed; /* 固定位置 */
  top: 0;
  left: 0;
}

.side-nav {
  width: 200px;
  background-color: #f1f1f1;
  padding: 20px 0;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  height: 100%; /* 改为相对高度 */
  overflow-y: auto; /* 允许菜单内容滚动 */
}

.nav-links {
  display: flex;
  flex-direction: column;
}

.side-nav a, .nav-button {
  color: #2c3e50;
  text-decoration: none;
  padding: 10px 20px;
  margin: 5px 0;
  border-left: 3px solid transparent;
  display: flex;
  align-items: center;
  transition: all 0.3s ease;
  background: none;
  border: none;
  cursor: pointer;
  text-align: left;
  width: 100%;
  font-size: 1rem;
}

.side-nav a.router-link-active {
  color: #42b983;
  border-left: 3px solid #42b983;
  background-color: rgba(66, 185, 131, 0.1);
}

.icon {
  width: 20px;
  height: 20px;
  margin-right: 10px;
  background-size: contain;
  background-repeat: no-repeat;
  background-position: center;
}

.chat-icon {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%232c3e50' d='M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H5.17L4 17.17V4h16v12z'/%3E%3C/svg%3E");
}

.server-icon {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%232c3e50' d='M20 13H4c-1.1 0-2 .9-2 2v5c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2v-5c0-1.1-.9-2-2-2zm0 7H4v-5h16v5zm0-16H4c-1.1 0-2 .9-2 2v5c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 7H4V6h16v5z'/%3E%3C/svg%3E");
}

.tools-icon {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%232c3e50' d='M22.7 19l-9.1-9.1c.9-2.3.4-5-1.5-6.9-2-2-5-2.4-7.4-1.3L9 6 6 9 1.6 4.7C.4 7.1.9 10.1 2.9 12.1c1.9 1.9 4.6 2.4 6.9 1.5l9.1 9.1c.4.4 1 .4 1.4 0l2.3-2.3c.5-.4.5-1.1.1-1.4z'/%3E%3C/svg%3E");
}

.github-icon {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%232c3e50' d='M12 .3a12 12 0 0 0-3.8 23.4c.6.1.8-.3.8-.6v-2c-3.3.7-4-1.6-4-1.6-.6-1.4-1.4-1.8-1.4-1.8-1-.7.1-.7.1-.7 1.2 0 1.9 1.2 1.9 1.2 1 1.8 2.8 1.3 3.5 1 0-.8.4-1.3.7-1.6-2.7-.3-5.5-1.3-5.5-6 0-1.2.5-2.3 1.3-3.1-.2-.4-.6-1.6 0-3.2 0 0 1-.3 3.4 1.2a11.5 11.5 0 0 1 6 0c2.3-1.5 3.3-1.2 3.3-1.2.6 1.6.2 2.8 0 3.2.9.8 1.3 1.9 1.3 3.2 0 4.6-2.8 5.6-5.5 5.9.5.4.9 1.1.9 2.3v3.3c0 .3.1.7.8.6A12 12 0 0 0 12 .3'/%3E%3C/svg%3E");
}

.settings-icon {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%232c3e50' d='M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z'/%3E%3C/svg%3E");
  }
  
  .model-icon {
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%232c3e50' d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z'/%3E%3C/svg%3E");
  }
  
  .general-icon {
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%232c3e50' d='M3 17v2h6v-2H3zM3 5v2h10V5H3zm10 16v-2h8v-2h-8v-2h-2v6h2zM7 9v2H3v2h4v2h2V9H7zm14 4v-2H11v2h10zm-6-4h2V7h4V5h-4V3h-2v6z'/%3E%3C/svg%3E");
  }
  
  .about-icon {
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%232c3e50' d='M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z'/%3E%3C/svg%3E");
  }

/* 响应式布局 */
@media (max-width: 768px) {
  .app-container {
    position: relative; /* 移动端使用相对定位 */
    height: auto; /* 允许移动端自适应高度 */
    min-height: 100vh;
  }
  
  .side-nav {
    width: 100%;
    height: auto;
    padding: 10px 0;
    flex-direction: row;
    justify-content: space-between;
    position: sticky; /* 菜单固定在顶部 */
    top: 0;
    z-index: 100;
  }
  
  .nav-links {
    flex-direction: row;
    flex-wrap: wrap;
    justify-content: center;
  }
  
  .side-nav a, .nav-button {
    padding: 8px 15px;
    margin: 3px;
    border-left: none;
    border-bottom: 3px solid transparent;
  }
  
  .side-nav a.router-link-active {
    border-left: none;
    border-bottom: 3px solid #42b983;
  }
  
  .bottom-buttons {
    display: flex;
    margin-top: 0;
  }
  
  .submenu {
    position: absolute;
    top: 100%;
    bottom: auto;
    left: auto;
    right: 0;
    width: 200px;
  }
  
  .content-container {
    padding: 10px;
  }
}
</style>

<style scoped>
.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.features-toggle {
  margin: 2rem auto;
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.features-toggle button {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
}

.language-icon {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%232c3e50' d='M12.87 15.07l-2.54-2.51.03-.03c1.74-1.94 2.98-4.17 3.71-6.53H17V4h-7V2H8v2H1v1.99h11.17C11.5 7.92 10.44 9.75 9 11.35 8.07 10.32 7.3 9.19 6.69 8h-2c.73 1.63 1.73 3.17 2.98 4.56l-5.09 5.02L4 19l5-5 3.11 3.11.76-2.04zM18.5 10h-2L12 22h2l1.12-3h4.75L21 22h2l-4.5-12zm-2.62 7l1.62-4.33L19.12 17h-3.24z'/%3E%3C/svg%3E");
}

@media (prefers-color-scheme: dark) {
  .language-icon {
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%23ffffff' d='M12.87 15.07l-2.54-2.51.03-.03c1.74-1.94 2.98-4.17 3.71-6.53H17V4h-7V2H8v2H1v1.99h11.17C11.5 7.92 10.44 9.75 9 11.35 8.07 10.32 7.3 9.19 6.69 8h-2c.73 1.63 1.73 3.17 2.98 4.56l-5.09 5.02L4 19l5-5 3.11 3.11.76-2.04zM18.5 10h-2L12 22h2l1.12-3h4.75L21 22h2l-4.5-12zm-2.62 7l1.62-4.33L19.12 17h-3.24z'/%3E%3C/svg%3E");
  }
}

.language-selector {
  padding: 10px 20px;
  display: flex;
  align-items: center;
  gap: 10px;
  border-bottom: 1px solid #e0e0e0;
}

.language-selector select {
  flex: 1;
  padding: 5px;
  border-radius: 4px;
  border: 1px solid #ddd;
  background: transparent;
  color: inherit;
  cursor: pointer;
}

@media (prefers-color-scheme: dark) {
  .language-selector {
    border-bottom: 1px solid #333;
  }
  
  .language-selector select {
    border-color: #444;
  }
}
</style>
