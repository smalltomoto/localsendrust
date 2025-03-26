<template>
    <div class="p-6 bg-gray-100 min-h-screen">
      <h1 class="text-2xl font-bold mb-4">设置</h1>
  
      <!-- 设备名称 -->
      <div class="mb-4">
        <label class="block font-semibold">设备名称:</label>
        <input v-model="deviceName" type="text" class="mt-1 p-2 border rounded-lg w-full" />
      </div>
  
      <!-- 存储路径 -->
      <div class="mb-4">
        <label class="block font-semibold">存储路径:</label>
        <input v-model="storagePath" type="text" class="mt-1 p-2 border rounded-lg w-full" />
        <button class="mt-2 px-4 py-2 bg-blue-500 text-white rounded-lg shadow-md" @click="chooseFolder">
          选择文件夹
        </button>
      </div>
  
      <!-- 端口号 -->
      <div class="mb-4">
        <label class="block font-semibold">端口号:</label>
        <input v-model="port" type="number" class="mt-1 p-2 border rounded-lg w-full" />
      </div>
  
      <button class="mt-4 px-4 py-2 bg-green-500 text-white rounded-lg shadow-md" @click="saveSettings">
        保存设置
      </button>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref } from "vue";
  
  const deviceName = ref("我的设备");
  const storagePath = ref("/user/downloads");
  const port = ref(8080);
  
  const chooseFolder = () => {
    console.log("选择存储路径");
    // 这里可以调用 Tauri API 选择文件夹
  };
  
  const saveSettings = () => {
    console.log("保存设置:", { deviceName: deviceName.value, storagePath: storagePath.value, port: port.value });
  };
  </script>
  