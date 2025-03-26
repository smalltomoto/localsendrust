use tauri::command;
use crate::service::scanner::scan_device;
use crate::models::device::Device;

#[command]
pub async fn scan_devices()->Vec<Device>{
    println!("===== scan_devices is start =====");
    scan_device().await
}