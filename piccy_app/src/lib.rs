mod command;
mod error;

use command::{
    download_file, image_crop, image_flip, image_grayscale, image_info, image_invert,
    image_resize, image_rotate,
};
use tauri::Manager;
use tauri_plugin_fs::FsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            image_info,
            image_crop,
            image_resize,
            image_rotate,
            image_flip,
            image_grayscale,
            image_invert,
            download_file
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(debug_assertions)]
            {
                window.open_devtools();
            }

            let scope = app.fs_scope();
            let download_dir = app.path().download_dir().unwrap();
            scope.allow_directory(download_dir, false).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap();
}
