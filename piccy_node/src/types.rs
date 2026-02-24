use image::Rgb;
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[napi(object)]
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
    pub average_duration: Option<f64>,
}

impl From<piccy_core::ImageInfo> for ImageInfo {
    fn from(result: piccy_core::ImageInfo) -> Self {
        Self {
            width: result.width,
            height: result.height,
            is_multi_frame: result.is_multi_frame,
            frame_count: result.frame_count,
            average_duration: result.average_duration.map(|d| d as f64),
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
    fn from(other: FlipMode) -> Self {
        match other {
            FlipMode::Horizontal => piccy_core::FlipMode::Horizontal,
            FlipMode::Vertical => piccy_core::FlipMode::Vertical,
        }
    }
}

/// 图像拼接模式
#[derive(Debug, Clone)]
#[napi]
pub enum MergeMode {
    /// 水平拼接
    Horizontal,
    /// 垂直拼接
    Vertical,
}

impl From<MergeMode> for piccy_core::MergeMode {
    fn from(other: MergeMode) -> Self {
        match other {
            MergeMode::Horizontal => piccy_core::MergeMode::Horizontal,
            MergeMode::Vertical => piccy_core::MergeMode::Vertical,
        }
    }
}

