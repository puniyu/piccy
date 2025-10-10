use crate::error::Error;
use serde::{Deserialize, Serialize};

use base64::{Engine, engine::general_purpose::STANDARD};
use image::{
    AnimationDecoder, ImageFormat, ImageReader,
    codecs::{gif::GifDecoder, webp::WebPDecoder},
};
use std::{io::Cursor, path::Path, sync::Arc};

#[derive(Default, Clone)]
pub struct Image(Arc<Vec<u8>>);

impl Image {
    pub fn new() -> Self {
        Image(Arc::new(Vec::new()))
    }

    pub fn with_path(&self, path: &Path) -> Result<Self, Error> {
        let data = std::fs::read(path)?;
        Ok(Image(Arc::new(data)))
    }
    pub fn with_buffer(&self, buffer: Vec<u8>) -> Self {
        Image(Arc::new(buffer))
    }

    pub fn with_base64(&self, base64: &str) -> Result<Self, Error> {
        let data = STANDARD.decode(base64)?;
        Ok(Image(Arc::new(data)))
    }

    pub fn builder(&self) -> ImageBuilder {
        ImageBuilder(self.0.clone())
    }
}
#[derive(Clone)]
pub struct ImageBuilder(Arc<Vec<u8>>);

impl ImageBuilder {
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
    pub fn info(self) -> Result<ImageInfo, Error> {
        let image_data = self.0.as_slice();
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
}

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

/// 计算帧信息
fn calculate_frame_info(frames: &[image::Frame]) -> (u32, Option<f32>) {
    let frame_count = frames.len() as u32;
    let average_duration = if frame_count > 1 {
        let total_duration = frames
            .iter()
            .map(|frame| frame.delay().numer_denom_ms().0)
            .sum::<u32>();
        Some((total_duration as f32 / 1000.0) / frame_count as f32)
    } else {
        Some(0.0f32)
    };
    (frame_count, average_duration)
}
