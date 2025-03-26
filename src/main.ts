import { createApp,nextTick  } from "vue";
import App from "./App.vue";
import './style/tailwind.css'
import {emit} from "@tauri-apps/api/event"
import { getTauriVersion } from "@tauri-apps/api/app";
const app = createApp(App);
app.mount("#app");
nextTick(async () => {
  const tauriVersion = await getTauriVersion();
    if (tauriVersion) {
    emit("vue-loaded");
    console.log("Vue 已发送 vue-loaded 事件");
  } else {
    console.warn("Tauri 运行环境未检测到，跳过 emit");
  }
});
