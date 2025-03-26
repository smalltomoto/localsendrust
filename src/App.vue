<template>
  <div class="flex flex-col min-h-screen">
    <nav class="p-4 flex justify-around">
      <button v-for="(tab, index) in tabs" :key="index"
      class="px-4 py-2 rounded-lg text-white bg-blue-300"
      :class="{ 'bg-blue-500': activeTab === tab.name }"
        @click="activeTab = tab.name">
        {{ tab.label }}
      </button>
    </nav> 
     <div class="p-4 w-full flex-1">
      <DeviceList v-if="activeTab === 'device'" />
      <SendFile v-if="activeTab === 'send'" />
      <ReceiveFile v-if="activeTab === 'receive'" />
      <History v-if="activeTab === 'history'" />
      <Setting v-if="activeTab === 'setting'" />
    </div>
  </div>
</template>

<script setup lang="ts">

import DeviceList from "./components/DeviceList.vue";
import SendFile from "./components/SendFile.vue";
import ReceiveFile from "./components/ReceiveFile.vue";
import History from "./components/History.vue";
import Setting from "./components/Setting.vue";
import { ref } from "vue";

const activeTab = ref("device");

const tabs = [
  { name: "device", label: "设备列表" },
  { name: "send", label: "发送文件" },
  { name: "receive", label: "接收文件" },
  { name: "history", label: "传输记录" },
  { name: "setting", label: "设置" }
];


</script>
<style>
.navi-top{
 
  color: white !important;

}
</style>