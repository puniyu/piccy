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

impl From<piccy_core::image::ImageInfo> for ImageInfo {
    fn from(result: piccy_core::image::ImageInfo) -> Self {
        Self {
            width: result.width,
            height: result.height,
            is_multi_frame: result.is_multi_frame,
            frame_count: result.frame_count,
            average_duration: result.average_duration.map(|d| d as f64),
        }
    }
}
