<template>
    <div class="p-6 bg-gray-100 min-h-screen">
      <h1 class="text-2xl font-bold mb-4">传输记录</h1>
  
      <!-- 传输记录列表 -->
      <div v-if="history.length > 0">
        <ul>
          <li v-for="item in history" :key="item.id" class="p-4 bg-white shadow-md rounded-lg mt-2">
            <div class="flex justify-between items-center">
              <div>
                <p class="font-semibold">{{ item.fileName }}</p>
                <p class="text-sm text-gray-500">{{ item.timestamp }}</p>
              </div>
              <span
                class="px-2 py-1 text-sm font-semibold rounded-lg"
                :class="{
                  'bg-green-500 text-white': item.status === '成功',
                  'bg-red-500 text-white': item.status === '失败',
                }"
              >
                {{ item.status }}
              </span>
            </div>
          </li>
        </ul>
  
        <button class="mt-4 px-4 py-2 bg-red-500 text-white rounded-lg shadow-md" @click="clearHistory">
          清空记录
        </button>
      </div>
  
      <p v-else class="text-gray-500">暂无传输记录</p>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref } from "vue";
  
  const history = ref([
    { id: 1, fileName: "file1.txt", timestamp: "2025-03-19 10:00", status: "成功" },
    { id: 2, fileName: "file2.jpg", timestamp: "2025-03-19 10:30", status: "失败" },
  ]);
  
  const clearHistory = () => {
    history.value = [];
  };
  </script>
  