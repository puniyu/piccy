use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[napi(object)]
pub struct ImageInfo {
    /// 图片大小，单位为字节
    pub size: u32,
    /// 图片宽度
    pub width: u32,
    /// 图片高度
    pub height: u32,
    /// 是否为动图
    pub is_multi_frame: bool,
    /// 动图帧数
    pub frame_count: Option<u32>,
    /// 动图平均帧间隔
    pub average_duration: Option<f64>,
}

impl From<piccy_core::ImageInfo> for ImageInfo {
    fn from(result: piccy_core::ImageInfo) -> Self {
        let animation = result.animation;
        Self {
            size: result.size as u32,
            width: result.dimensions.width,
            height: result.dimensions.height,
            is_multi_frame: animation.is_some(),
            frame_count: animation.map(|info| info.frame_count),
            average_duration: animation.map(|info| info.frame_delay as f64),
        }
    }
}

#[derive(Debug, Clone)]
#[napi]
pub enum FlipMode {
    /// 水平翻转
    Horizontal,
    /// 垂直翻转
    Vertical,
}

impl From<FlipMode> for piccy_core::FlipMode {
    fn from(mode: FlipMode) -> Self {
        match mode {
            FlipMode::Horizontal => piccy_core::FlipMode::Horizontal,
            FlipMode::Vertical => piccy_core::FlipMode::Vertical,
        }
    }
}

/// 图片拼接模式
#[derive(Debug, Clone)]
#[napi]
pub enum MergeMode {
    /// 水平拼接
    Horizontal,
    /// 垂直拼接
    Vertical,
}

impl From<MergeMode> for piccy_core::MergeMode {
    fn from(mode: MergeMode) -> Self {
        match mode {
            MergeMode::Horizontal => piccy_core::MergeMode::Horizontal,
            MergeMode::Vertical => piccy_core::MergeMode::Vertical,
        }
    }
}

#[derive(Debug, Clone)]
#[napi]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
}

impl From<ImageFormat> for piccy_core::ImageFormat {
    fn from(image: ImageFormat) -> Self {
        match image {
            ImageFormat::Png => piccy_core::ImageFormat::Png,
            ImageFormat::Jpeg => piccy_core::ImageFormat::Jpeg,
            ImageFormat::WebP => piccy_core::ImageFormat::WebP,
        }
    }
}

#[napi(object)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Rgb> for image::Rgb<u8> {
    fn from(value: Rgb) -> Self {
        Self::from([value.r, value.g, value.b])
    }
}
