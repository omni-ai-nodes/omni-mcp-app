import { createRouter, createWebHistory } from 'vue-router'
import CommandExecutor from '../components/CommandExecutor.vue'
import Greeter from '../components/Greeter.vue'
import ToolManager from '../components/ToolManager.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: CommandExecutor
  },
  {
    path: '/greeter',
    name: 'Greeter',
    component: Greeter
  },
  {
    path: '/tools',
    name: 'ToolManager',
    component: ToolManager
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router