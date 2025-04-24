<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="greeting-section">
    <h2>问候功能</h2>
    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="输入名字..." />
      <button type="submit">问候</button>
    </form>
    <p>{{ greetMsg }}</p>
  </div>
</template>

<style scoped>
.greeting-section {
  margin-top: 2rem;
  border-top: 1px solid #ccc;
  padding-top: 1rem;
}

.row {
  display: flex;
  justify-content: center;
}

#greet-input {
  margin-right: 5px;
}
</style>