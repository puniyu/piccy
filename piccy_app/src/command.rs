use crate::error::Error;
use piccy_core::{Image, ImageInfo};
use tauri::ipc::Response;
use tauri::{AppHandle, Manager, Runtime, command};

#[command]
pub(crate) fn image_info(image: Vec<u8>) -> tauri::Result<ImageInfo> {
    let image = Image::from_bytes(image);
    image.info().map_err(|e| tauri::Error::from(Error::from(e)))
}
#[command]
pub(crate) fn image_crop(
    image: Vec<u8>,
    left: Option<u32>,
    top: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
) -> tauri::Result<Response> {
    let img = Image::from_bytes(image);

    let result = img
        .crop(
            left.unwrap_or(0),
            top.unwrap_or(0),
            width.unwrap_or(100),
            height.unwrap_or(100),
        )
        .map_err(|e| tauri::Error::from(Error::from(e)));

    match result {
        Ok(data) => Ok(Response::new(data.into_bytes().to_vec())),
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
