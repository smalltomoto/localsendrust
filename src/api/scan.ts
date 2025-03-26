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