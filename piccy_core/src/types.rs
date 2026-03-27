use image::Frame;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct AnimationInfo {
    /// 动画帧数
    pub frame_count: u32,
    /// 平均帧延迟，单位为毫秒
    pub frame_delay: f32,
}

impl Default for AnimationInfo {
    fn default() -> Self {
        Self {
            frame_count: 0,
            frame_delay: 0.0,
        }
    }
}

impl From<&[Frame]> for AnimationInfo {
    fn from(frames: &[Frame]) -> Self {
        let frame_count = frames.len() as u32;
        let frame_delay = if frame_count > 0 {
            frames
                .iter()
                .map(|frame| frame.delay().numer_denom_ms().0 as f32)
                .sum::<f32>()
                / frame_count as f32
        } else {
            0.0
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

impl From<&Vec<Frame>> for AnimationInfo {
    fn from(frames: &Vec<Frame>) -> Self {
        frames.as_slice().into()
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct ImageInfo {
    /// 图片大小，单位为字节
    pub size: usize,
    pub dimensions: Dimensions,
    pub animation: Option<AnimationInfo>,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct Dimensions {
    /// 图片宽度，单位为像素
    pub width: u32,
    /// 图片高度，单位为像素
    pub height: u32,
}
/// 图片翻转模式
#[derive(Debug, Clone, Default)]
pub enum FlipMode {
    /// 水平翻转
    #[default]
    Horizontal,
    /// 垂直翻转
    Vertical,
}

/// 图片拼接模式
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
