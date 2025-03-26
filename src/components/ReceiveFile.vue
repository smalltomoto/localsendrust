<template>
    <div class="p-6 bg-gray-100 min-h-screen">
      <h1 class="text-2xl font-bold mb-4">接收文件</h1>
  
      <!-- 传输进度 -->
      <ul class="mt-4">
        <li v-for="file in receivedFiles" :key="file.name" class="p-2 bg-white shadow-md rounded-lg mt-2">
          {{ file.name }} - {{ file.progress }}%
          <div class="w-full bg-gray-200 h-2 mt-1">
            <div class="bg-green-500 h-2" :style="{ width: file.progress + '%' }"></div>
          </div>
        </li>
      </ul>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref } from "vue";
  
  const receivedFiles = ref([
    { name: "file1.txt", progress: 40 },
    { name: "file2.jpg", progress: 75 },
  ]);
  </script>
  