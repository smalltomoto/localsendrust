<template>
    <div class="p-6 bg-gray-100 min-h-screen">
      <h1 class="text-2xl font-bold mb-4">发送文件</h1>
  
      <!-- 目标设备 -->
      <div class="bg-white p-4 rounded-lg shadow-md">
        <h2 class="text-lg font-semibold">目标设备: {{ targetDevice.name }}</h2>
        <p class="text-gray-600">{{ targetDevice.ip }}</p>
      </div>
  
      <!-- 拖拽上传区域 -->
      <div
        class="mt-4 p-6 border-2 border-dashed border-gray-400 rounded-lg text-center cursor-pointer"
        @drop="handleDrop"
        @dragover.prevent
      >
        <p class="text-gray-600">拖拽文件到这里 或者</p>
        <button class="mt-2 px-4 py-2 bg-blue-500 text-white rounded-lg shadow-md" @click="selectFile">
          选择文件
        </button>
      </div>
  
      <!-- 选中文件列表 -->
      <ul class="mt-4">
        <li
          v-for="file in selectedFiles"
          :key="file.name"
          class="p-2 bg-white shadow-md rounded-lg mt-2"
        >
          {{ file.name }} ({{ file.size }} bytes)
        </li>
      </ul>
  
      <!-- 发送按钮 -->
      <button class="mt-4 px-4 py-2 bg-green-500 text-white rounded-lg shadow-md" @click="sendFiles">
        发送文件
      </button>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref } from "vue";
  
  const targetDevice = ref({ name: "设备 A", ip: "192.168.1.101" });
  const selectedFiles = ref<File[]>([]);
  
  const handleDrop = (event: DragEvent) => {
    event.preventDefault();
    if (event.dataTransfer?.files.length) {
      selectedFiles.value = Array.from(event.dataTransfer.files);
    }
  };
  
  const selectFile = () => {
    const input = document.createElement("input");
    input.type = "file";
    input.multiple = true;
    input.onchange = () => {
      if (input.files) {
        selectedFiles.value = Array.from(input.files);
      }
    };
    input.click();
  };
  
  const sendFiles = () => {
    console.log("发送文件:", selectedFiles.value);
  };
  </script>
  