import { invoke } from '@tauri-apps/api/core';
export async function scanDevices(): Promise<Device[]> {
  try {
    const devices: Device[] = await invoke<Device[]>("scan_devices");
    return devices;
  } catch (error) {
    console.error("Error scanning devices:", error);
    return [];
  }
}

export async function detectAndSetAffinity() {
  try {
    await invoke('detect_and_set_affinity');  // 调用 Rust 后端命令检测并设置亲和性
    console.log('亲和性已成功设置到大核');
  } catch (error) {
    console.error('检测和设置亲和性失败:', error);
  }
}