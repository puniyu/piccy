mod error;

use tauri::command;
use piccy_core::image;
use crate::error::Error;

#[command]
fn image_info(image_data: Vec<u8>) -> tauri::Result<image::ImageInfo> {
    let info = image::Image::new().with_buffer(image_data).builder().info();
    info.map_err(|e| tauri::Error::from(Error::from(e)))
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            image_info
        ])
        .setup(move |app| {
            #[cfg(debug_assertions)]
            {
                use tauri::Manager;
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap();
}
