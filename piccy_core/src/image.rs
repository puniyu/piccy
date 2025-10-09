use crate::error::Error;
use serde::{Deserialize, Serialize};

use base64::{Engine, engine::general_purpose::STANDARD};
use image::{
    AnimationDecoder, ImageFormat, ImageReader,
    codecs::{gif::GifDecoder, webp::WebPDecoder},
};
use std::{io::Cursor, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageInfo {
    /// 图像宽度
    pub width: u32,
    /// 图像高度
    pub height: u32,
    /// 是否为动图
    pub is_multi_frame: bool,
    /// 动图帧数
    pub frame_count: Option<u32>,
    /// 动图平均帧间隔
    pub average_duration: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum ImageSource {
    Buffer(Vec<u8>),
    Path(String),
    Base64(String),
}

impl From<Vec<u8>> for ImageSource {
    fn from(value: Vec<u8>) -> Self {
        Self::Buffer(value)
    }
}

impl From<&Path> for ImageSource {
    fn from(value: &Path) -> Self {
        Self::Path(value.to_string_lossy().to_string())
    }
}

impl From<String> for ImageSource {
    fn from(value: String) -> Self {
        Self::Base64(value)
    }
}


/// 获取图像信息
///
/// # 参数
/// `source`: 图像源，可以是 `Vec<u8>`、`&Path` 或 Base64 字符串
///
/// # 返回值
/// 返回 `ImageInfo` 结构体，包含图像的宽度、高度、是否为动图、帧数和平均帧间隔等信息
///
/// # 示例
///
/// ```rust
/// let info = get_image_info("./image.png")?;
///```
pub fn get_image_info(source: impl Into<ImageSource>) -> Result<ImageInfo, Error> {
    let image_data = match source.into() {
        ImageSource::Buffer(buffer) => buffer,
        ImageSource::Path(path) => std::fs::read(&path)?,
        ImageSource::Base64(base64_str) => STANDARD.decode(&base64_str)?,
    };

    let reader = ImageReader::new(Cursor::new(&image_data)).with_guessed_format()?;

    match reader.format() {
        Some(ImageFormat::Gif) => {
            let image = reader.decode()?;
            let decoder = GifDecoder::new(Cursor::new(&image_data))?;
            let frames = decoder.into_frames().collect_frames()?;
            let (frame_count, average_duration) = calculate_frame_info(&frames);

            Ok(ImageInfo {
                width: image.width(),
                height: image.height(),
                is_multi_frame: frame_count > 1,
                frame_count: Some(frame_count),
                average_duration,
            })
        }
        Some(ImageFormat::WebP) => {
            let image = reader.decode()?;
            let decoder = WebPDecoder::new(Cursor::new(&image_data))?;
            let frames = decoder.into_frames().collect_frames()?;
            let (frame_count, average_duration) = calculate_frame_info(&frames);

            Ok(ImageInfo {
                width: image.width(),
                height: image.height(),
                is_multi_frame: frame_count > 1,
                frame_count: Some(frame_count),
                average_duration,
            })
        }
        _ => {
            let image = reader.decode()?;

            Ok(ImageInfo {
                width: image.width(),
                height: image.height(),
                is_multi_frame: false,
                frame_count: Some(1),
                average_duration: Some(0.0),
            })
        }
    }
}

/// 计算帧信息
fn calculate_frame_info(frames: &[image::Frame]) -> (u32, Option<f32>) {
    let frame_count = frames.len() as u32;
    let average_duration = if frame_count > 1 {
        let total_duration= frames
            .iter()
            .map(|frame| frame.delay().numer_denom_ms().0)
            .sum::<u32>();
        Some((total_duration as f32 / 1000.0) / frame_count as f32)
    } else {
        Some(0.0f32)
    };
    (frame_count, average_duration)
}
