import { createRouter, createWebHistory } from 'vue-router'
import CommandExecutor from '../components/CommandExecutor.vue'
import Greeter from '../components/GreeterChat.vue'
import ToolManager from '../components/ToolManager.vue'
import ModelSettings from '../components/ModelSettings.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: CommandExecutor
  },
  {
    path: '/greeterChat',
    name: 'Greeter',
    component: Greeter
  },
  {
    path: '/tools',
    name: 'ToolManager',
    component: ToolManager
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