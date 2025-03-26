#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::{Listener, Manager};
mod commands;
mod models;
mod service;
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![
//             commands::network::scan_devices
//         ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::network::scan_devices])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.hide().unwrap();

            let app_handle = app.handle();
            let win_clone = window.clone();

            app_handle.listen("vue-loaded", move |_| {
                win_clone.show().unwrap();
                println!("Tauri 收到 vue-loaded 事件，窗口已显示");
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
