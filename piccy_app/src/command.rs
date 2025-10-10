use crate::error::Error;
use piccy_core::image;
use tauri::ipc::Response;
use tauri::{AppHandle, Manager, command, Runtime};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

#[command]
pub(crate) fn image_info(image_data: Vec<u8>) -> tauri::Result<image::ImageInfo> {
    let info = image::Image::new().with_buffer(image_data).builder().info();
    info.map_err(|e| tauri::Error::from(Error::from(e)))
}
#[command]
pub(crate) fn image_crop(
    image_data: Vec<u8>,
    left: Option<u32>,
    top: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
) -> tauri::Result<Response> {
    let image_result = image::Image::new()
        .with_buffer(image_data)
        .builder()
        .crop(left, top, width, height)
        .map_err(|e| tauri::Error::from(Error::from(e)));

    match image_result {
        Ok(data) => Ok(Response::new(data)),
        Err(e) => Err(e),
    }
}

#[command]
pub(crate) fn download_file<R: Runtime>(app_handle: AppHandle<R>, data: Vec<u8>) -> tauri::Result<()> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let download_dir = app_handle.path().download_dir()?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("{}.png", timestamp);
    let path = download_dir.join(filename);
    std::fs::write(path.as_path(), data)?;
    app_handle.dialog()
       .message(format!("文件保存在{}",path.to_string_lossy()))
       .kind(MessageDialogKind::Info)
       .blocking_show();
    Ok(())
}
