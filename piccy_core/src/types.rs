use image::Frame;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone)]
pub struct AnimationInfo {
    /// 帧数
    pub frame_count: u32,
    /// 帧间隔时间（毫秒）
    pub frame_delay: Option<f32>,
}

impl From<&[Frame]> for AnimationInfo {
    fn from(frames: &[Frame]) -> Self {
        let frame_count = frames.len() as u32;
        let frame_delay = if frame_count > 0 {
            let avg_delay: u64 = frames
                .iter()
                .map(|frame| frame.delay().numer_denom_ms().0 as u64)
                .sum::<u64>()
                / frame_count.max(1) as u64;
            Some(avg_delay as f32)
        } else {
            None
        };
        Self {
            frame_count,
            frame_delay,
        }
    }
}

impl From<Vec<Frame>> for AnimationInfo {
    fn from(frames: Vec<Frame>) -> Self {
        frames.as_slice().into()
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

/// 图像翻转模式
#[derive(Debug, Clone, Default)]
pub enum FlipMode {
    /// 水平翻转
    #[default]
    Horizontal,
    /// 垂直翻转
    Vertical,
}

/// 图像拼接模式
#[derive(Debug, Clone, Default)]
pub enum MergeMode {
    /// 水平拼接
    #[default]
    Horizontal,
    /// 垂直拼接
    Vertical,
}

#[derive(Debug, Clone)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
}
