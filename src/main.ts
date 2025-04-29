import { createApp } from "vue";
import App from "./App.vue";
import router from './router'
import { i18n } from './i18n'
import './style.css'  // 如果有全局样式文件，确保导入

createApp(App)
  .use(router)
  .use(i18n)
  .mount("#app");
