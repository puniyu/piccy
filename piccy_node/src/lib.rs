mod common;
mod types;

type Result<T> = napi::Result<T>;

use crate::types::ImageFormat;
use common::parse_rgb;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use piccy_core::Image;
use std::time::Duration;
use types::{FlipMode, ImageInfo, MergeMode};

/// 获取图像信息
#[napi]
pub fn image_info(image_data: Buffer) -> Result<ImageInfo> {
    let image = Image::from_bytes(image_data.to_vec());
    let info = image
        .info()
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(info.into())
}

/// 转换图像格式
#[napi]
pub fn image_convert(image_data: Buffer, format: Option<ImageFormat>) -> Result<Buffer> {
    let image = Image::from_bytes(image_data.to_vec());
    let result = image
        .to_bytes(
            format
                .map(Into::into)
                .unwrap_or(piccy_core::ImageFormat::Png),
        )
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.to_vec().into())
}

/// 裁剪图像
///
/// # 参数
/// - `left`: 裁剪的左上角 X 坐标, 默认为 0
/// - `top`: 裁剪的左上角 Y 坐标, 默认为 0
/// - `width`: 裁剪的宽度, 默认为100
/// - `height`: 裁剪的高度, 默认为100
#[napi]
pub fn image_crop(
    image_data: Buffer,
    left: Option<u32>,
    top: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<Buffer> {
    let image = Image::from_bytes(image_data.to_vec());
    let result = image
        .crop(
            left.unwrap_or(0),
            top.unwrap_or(0),
            width.unwrap_or(100),
            height.unwrap_or(100),
        )
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}

/// 缩放图像
///
/// ## 参数
/// - `width`: 缩放后的宽度
/// - `height`: 缩放后的高度
#[napi]
pub fn image_resize(image_data: Buffer, width: u32, height: u32) -> Result<Buffer> {
    let image = Image::from_bytes(image_data.to_vec());
    let result = image
        .resize(width, height)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}

/// 旋转图像
///
/// ## 参数
/// - `angle`: 旋转的角度
#[napi]
pub fn image_rotate(image_data: Buffer, angle: f64) -> Result<Buffer> {
    let image = Image::from_bytes(image_data.to_vec());
    let result = image
        .rotate(angle as f32)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}

/// 翻转图像
///
/// ## 参数
/// - `mode`: 翻转模式
///
#[napi]
pub fn image_flip(image_data: Buffer, mode: FlipMode) -> Result<Buffer> {
    let image = Image::from_bytes(image_data.to_vec());
    let result = image
        .flip(mode.into())
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}

/// 灰度化图像
#[napi]
pub fn image_grayscale(image_data: Buffer) -> Result<Buffer> {
    let image = Image::from_bytes(image_data.to_vec());
    let result = image
        .grayscale()
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}

/// 反色图像
#[napi]
pub fn image_invert(image_data: Buffer) -> Result<Buffer> {
    let image = Image::from_bytes(image_data.to_vec());
    let result = image
        .invert()
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}

/// 颜色蒙版
#[napi]
pub fn image_color_mask(image_data: Buffer, rgb: String) -> Result<Buffer> {
    let image = Image::from_bytes(image_data.to_vec());
    let mask_color = parse_rgb(&rgb)?;
    let result = image
        .color_mask(mask_color)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}

/// 幻影坦克
///
/// ## 参数
/// - `image`: 需要显示的图片
/// - `image2`: 需要隐藏的图片
#[napi]
pub fn image_mirage(image_data: Buffer, image_data2: Buffer) -> Result<Buffer> {
    let image = Image::from_bytes(image_data.to_vec());
    let image2 = Image::from_bytes(image_data2.to_vec());
    let result = image
        .mirage(image2)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}

/// 拼接图片
///
/// ## 参数
/// - `images`: 需要拼接的图片实例
/// - `mode`: 拼接模式
#[napi]
pub fn image_merge(images: Vec<Buffer>, mode: MergeMode) -> Result<Buffer> {
    let image_data: Vec<Image> = images
        .into_iter()
        .map(|image| Image::from_bytes(image.to_vec()))
        .collect();
    let result = Image::merge(image_data, mode.into())
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}

/// gif分解
#[napi]
pub fn gif_split(image_data: Buffer) -> Result<Vec<Buffer>> {
    let image = Image::from_bytes(image_data.to_vec());
    let result = image
        .split()
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result
        .into_iter()
        .map(|img| img.into_bytes().to_vec().into())
        .collect())
}

/// gif倒放
#[napi]
pub fn gif_reverse(image_data: Buffer) -> Result<Buffer> {
    let image = Image::from_bytes(image_data.to_vec());
    let result = image
        .reverse()
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}

/// gif倒放
///
/// ## 参数
/// - `duration`: 帧间隔时间(单位: 秒)
#[napi]
pub fn gif_change_duration(image_data: Buffer, duration: u32) -> Result<Buffer> {
    let image = Image::from_bytes(image_data.to_vec());
    let result = image
        .change_duration(Duration::from_secs(duration as u64))
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}

/// gif拼接
///
/// ## 参数
/// - `images`: 图片
/// - `duration`: 帧间隔时间
#[napi]
pub fn gif_merge(images: Vec<Buffer>, duration: Option<u32>) -> Result<Buffer> {
    let image_data: Vec<Image> = images
        .into_iter()
        .map(|image| Image::from_bytes(image.to_vec()))
        .collect();
    let delay = duration.map(|d| Duration::from_secs(d as u64));
    let result =
        Image::merge_gif(image_data, delay).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(result.into_bytes().to_vec().into())
}
