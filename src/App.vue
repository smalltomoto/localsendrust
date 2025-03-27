<!-- <template>
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
      <DeviceList v-show="activeTab === 'device'" />
      <SendFile v-if="activeTab === 'send'" />
      <ReceiveFile v-if="activeTab === 'receive'" />
      <History v-if="activeTab === 'history'" />
      <Setting v-if="activeTab === 'setting'" />
    </div>
  </div>
</template> -->

<template>
  <div class="flex flex-col min-h-screen">
    <nav class="p-4 flex justify-around">
      <router-link v-for="(tab, index) in tabs" :key="index" class="px-4 py-2 rounded-lg text-white bg-blue-300"
        :class="{ 'bg-blue-500': $route.path === tab.path }" :to="tab.path">
        {{ tab.label }}
      </router-link>
    </nav>
    <div class="p-4 w-full flex-1">
      <router-view v-slot="{ Component }">
        <keep-alive include="DeviceList,SendFile">
          <component :is="Component" />
        </keep-alive>
      </router-view>

    </div>
  </div>
</template>

<script setup lang="ts">

const tabs = [
  { path: "/deviceList", label: "设备列表" },
  { path: "/sendFile", label: "发送文件" },
  { path: "/receiveFile", label: "接收文件" },
  { path: "/history", label: "传输记录" },
  { path: "/setting", label: "设置" }
];



</script>
<style>
.navi-top {

  color: white !important;

}
</style>