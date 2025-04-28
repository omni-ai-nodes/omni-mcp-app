<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");
const containerWidth = ref("100%");

// 从本地存储加载用户之前调整的宽度
onMounted(() => {
  if (typeof window !== 'undefined') {
    const savedWidth = localStorage.getItem('greeterWidth');
    if (savedWidth) {
      containerWidth.value = savedWidth;
    }
  }
});

// 保存用户调整的宽度到本地存储
function saveContainerWidth() {
  if (typeof window !== 'undefined') {
    localStorage.setItem('greeterWidth', containerWidth.value);
  }
}

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="greeting-section" :style="{ width: containerWidth }">
    <h2>问候功能</h2>
    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="输入名字..." />
      <button type="submit">问候</button>
    </form>
    <p class="greeting-message">{{ greetMsg }}</p>
  </div>
</template>

<style scoped>
.greeting-section {
  margin: 2rem auto;
  max-width: 800px;
  padding: 1.5rem;
  border-radius: 8px;
  background-color: #f9f9f9;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transition: width 0.3s ease;
}

.row {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-bottom: 1rem;
}

#greet-input {
  flex: 1;
  min-width: 200px;
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid #ccc;
}

button {
  padding: 8px 16px;
  border-radius: 4px;
  background-color: #42b983;
  color: white;
  border: none;
  cursor: pointer;
  transition: background-color 0.2s;
}

button:hover {
  background-color: #3aa876;
}

.greeting-message {
  margin-top: 1rem;
  padding: 1rem;
  background-color: #f1f1f1;
  border-radius: 4px;
  font-weight: 500;
}

@media (max-width: 768px) {
  .greeting-section {
    width: 95% !important;
    padding: 1rem;
  }
  
  .row {
    flex-direction: column;
  }
  
  #greet-input {
    width: 100%;
    margin-right: 0;
    margin-bottom: 8px;
  }
}

@media (prefers-color-scheme: dark) {
  .greeting-section {
    background-color: #2a2a2a;
  }
  
  .greeting-message {
    background-color: #333;
  }
  
  #greet-input {
    background-color: #333;
    border-color: #444;
    color: #fff;
  }
}
</style>