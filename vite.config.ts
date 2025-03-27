import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";
// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue(),
    tailwindcss()],
  clearScreen: false,
  server: {
    host: "0.0.0.0", 
    port: 5173,      
  }
}));
