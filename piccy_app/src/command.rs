use crate::error::Error;
use piccy_core::image::{self, Image};
use tauri::ipc::Response;
use tauri::{AppHandle, Manager, Runtime, command};

#[command]
pub(crate) fn image_info(image: Vec<u8>) -> tauri::Result<image::ImageInfo> {
    let image_data = image::ImageBuilder::new()
        .with_buffer(image);
    Image::new(image_data)
        .info()
        .map_err(|e| tauri::Error::from(Error::from(e)))
}
#[command]
pub(crate) fn image_crop(
    image: Vec<u8>,
    left: Option<u32>,
    top: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
) -> tauri::Result<Response> {
    let image_data = image::ImageBuilder::new()
        .with_buffer(image);

    let result = Image::new(image_data)
        .crop(left, top, width, height)
        .map_err(|e| tauri::Error::from(Error::from(e)));

    match result {
        Ok(data) => Ok(Response::new(data)),
        Err(e) => Err(e),
    }
}

#[command]
pub(crate) fn download_file<R: Runtime>(
    app_handle: AppHandle<R>,
    data: Vec<u8>,
) -> tauri::Result<String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let download_dir = app_handle.path().download_dir()?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("{}.png", timestamp);
    let path = download_dir.join(filename);
    std::fs::write(path.as_path(), &data)?;
    Ok(path.to_string_lossy().to_string())
}
