<template>
  <div class="flex items-center justify-center overflow-hidden">
    <div class="w-full max-w-md bg-white shadow-lg rounded-lg">
      <!-- 卡片头部 -->
      <div class="px-6 py-4 border-b border-gray-200  flex justify-center">
        <img :src="loadingIcon" alt="Loading" class="w-16 h-16 opacity-75" />
      </div>
      <!-- 设备列表区域：内部滚动 -->
      <div class="list-device" v-if="devices.length > 0">
        <div v-for="device in devices" :key="device.id" class="item-device" :class="{
          'bg-blue-500 text-white': selectedDevice === device.id,
          'hover:bg-gray-200': selectedDevice !== device.id
        }" @click="selectDevice(device)">
          <div class="device-name">{{ device.name }} ({{ device.ip }})</div>
          <div class="text-green-500">在线</div>
        </div>
      </div>
      <div v-if="touch && devices.length == 0">
        <div class="flex items-center justify-center text-red font-medium py-2 rounded-lg shadow-md">
          当前没有设备在线
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
import { useDeviceStore } from "../store/devicestore";
import { useRouter } from "vue-router";
import { onActivated } from 'vue';
const devices = ref<Device[]>([]);
const loading = ref(false);
const touch = ref(false);
const selectedDevice = ref<number | null>(null);
const deviceStore = useDeviceStore();
const router = useRouter();

async function handleScanDevices() {
  devices.value = [];
  loading.value = true;
  try {
    const result = await scanDevices();
    touch.value = true;
    devices.value = result;
  } catch (error) {
    console.error("🚀 ~ handleScanDevices ~ error:", error)
  }
  finally {
    loading.value = false;
    touch.value = false;
  }
}

const selectDevice = (device: Device) => {
  selectedDevice.value = (device.id) === selectedDevice.value ? null : (device.id);
  deviceStore.setSelectedDevice(device);
  router.push('/sendFile');
};

onActivated(() => {
  console.log('deviceList 页面被恢复');
});

</script>
<style>
.list-device {
  height: 300px;
  overflow-y: auto;
  width: 100%;
}

.item-device {
  padding: 10px;
  border: 1px solid #ddd;
  cursor: pointer;
  transition: background-color 0.2s ease-in-out;
}
</style>