<template>
  <div class="flex items-center justify-center overflow-hidden">
    <div class="w-full max-w-md bg-white shadow-lg rounded-lg">
      <!-- 卡片头部 -->
      <div class="px-6 py-4 border-b border-gray-200  flex justify-center">
        <img :src="loadingIcon" alt="Loading" class="w-16 h-16 opacity-75"/>
        <!-- <h2 class="text-lg font-semibold text-gray-800 flex justify-center">可用设备</h2> -->
      </div>
      <!-- 设备列表区域：内部滚动 -->
      <div class="list-device" v-if="devices.length > 0">
        <div v-for="device in devices" :key="device.id" class="item-device" @click="selectDevice(device)">
          <div class="device-name">{{ device.name }} ({{ device.ip }})</div>
          <div class="text-green-500">在线</div>
        </div>
      </div>
      <!-- 按钮区域 -->
      
      <div class="px-6 py-4 bg-gray-50 border-t border-gray-200">
        <div class="flex flex-col space-y-2">
          <button class="w-full bg-blue-500 hover:bg-blue-600 text-white font-medium py-2 rounded-lg shadow-md"
            @click="handleScanDevices">
            扫描设备
          </button>
          
          <div v-if="loading" class="mt-4 flex justify-center space-x-2">
            <div class="w-3 h-3 bg-blue-500 rounded-full animate-bounce"></div>
            <div class="w-3 h-3 bg-blue-500 rounded-full animate-bounce [animation-delay:-0.2s]"></div>
            <div class="w-3 h-3 bg-blue-500 rounded-full animate-bounce [animation-delay:-0.4s]"></div>
          </div>
          <button class="w-full bg-blue-500 hover:bg-blue-600 text-white font-medium py-2 rounded-lg shadow-md"
            @click="$emit('view-history')">
            查看传输记录
          </button>
          <button class="w-full bg-gray-500 hover:bg-gray-600 text-white font-medium py-2 rounded-lg shadow-md"
            @click="$emit('view-settings')">
            设置
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { scanDevices } from "../api/scan";
import loadingIcon from "../assets/crab.svg"; 
const devices = ref<Device[]>([]);
const loading = ref(false);
async function handleScanDevices() {
  loading.value = true;
  try {
    const result = await scanDevices();
    devices.value = result;
  } catch (error) {
    console.error("🚀 ~ handleScanDevices ~ error:", error)
  }
  finally
  {
    loading.value =false;
  }
  
}

const selectDevice = (device: { id: number; name: string; ip: string }) => {
  console.log("选择设备:", device);
};
</script>
<style>
.list-device {
  height: 300px; /* 确保它不会超出容器 */
  overflow-y: auto; /* 使用 auto 而不是 scroll，避免始终显示滚动条 */
  width: 100%;
}

.item-device {
  display: flex;
  justify-content: space-between;
  padding: 10px 0; /* 添加间距使每个设备之间有一定的距离 */
}

</style>