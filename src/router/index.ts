import { createRouter, createWebHistory } from 'vue-router'
import GreeterChat from '../components/GreeterChat.vue'
import MCPService from '../components/MCPService.vue'
import Tools from '../components/ToolManager.vue'
import ModelSettings from '../components/ModelSettings.vue'

const routes = [
  {
    path: '/',
    redirect: '/greeterChat'
  },
  {
    path: '/greeterChat',
    name: 'GreeterChat',
    component: GreeterChat
  },
  {
    path: '/mcpService',
    name: 'MCPService',
    component: MCPService
  },
  {
    path: '/tools',
    name: 'Tools',
    component: Tools
  },
  {
    path: '/settings/model',
    name: 'ModelSettings',
    component: ModelSettings
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router