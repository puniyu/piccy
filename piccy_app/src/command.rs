use crate::error::Error;
use piccy_core::{FlipMode, Image, ImageInfo};
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
    left: u32,
    top: u32,
    width: u32,
    height: u32,
) -> tauri::Result<Response> {
    let img = Image::from_bytes(image);

    let result = img
        .crop(left, top, width, height)
        .map_err(|e| tauri::Error::from(Error::from(e)));

    match result {
        Ok(data) => Ok(Response::new(data.into_bytes().to_vec())),
        Err(e) => Err(e),
    }
}

#[command]
pub(crate) fn image_resize(
    image: Vec<u8>,
    width: u32,
    height: u32,
) -> tauri::Result<Response> {
    let img = Image::from_bytes(image);
    let result = img
        .resize(width, height)
        .map_err(|e| tauri::Error::from(Error::from(e)));

    match result {
        Ok(data) => Ok(Response::new(data.into_bytes().to_vec())),
        Err(e) => Err(e),
    }
}

#[command]
pub(crate) fn image_rotate(image: Vec<u8>, angle: f32) -> tauri::Result<Response> {
    let img = Image::from_bytes(image);
    let result = img
        .rotate(angle)
        .map_err(|e| tauri::Error::from(Error::from(e)));

    match result {
        Ok(data) => Ok(Response::new(data.into_bytes().to_vec())),
        Err(e) => Err(e),
    }
}

#[command]
pub(crate) fn image_flip(image: Vec<u8>, mode: String) -> tauri::Result<Response> {
    let img = Image::from_bytes(image);
    let flip_mode = match mode.as_str() {
        "horizontal" => FlipMode::Horizontal,
        "vertical" => FlipMode::Vertical,
        _ => FlipMode::Horizontal,
    };

    let result = img
        .flip(Some(flip_mode))
        .map_err(|e| tauri::Error::from(Error::from(e)));

    match result {
        Ok(data) => Ok(Response::new(data.into_bytes().to_vec())),
        Err(e) => Err(e),
    }
}

#[command]
pub(crate) fn image_grayscale(image: Vec<u8>) -> tauri::Result<Response> {
    let img = Image::from_bytes(image);
    let result = img
        .grayscale()
        .map_err(|e| tauri::Error::from(Error::from(e)));

    match result {
        Ok(data) => Ok(Response::new(data.into_bytes().to_vec())),
        Err(e) => Err(e),
    }
}

#[command]
pub(crate) fn image_invert(image: Vec<u8>) -> tauri::Result<Response> {
    let img = Image::from_bytes(image);
    let result = img
        .invert()
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
