#[derive(Debug, Clone)]
pub struct AnimationInfo {
    /// 帧数
    pub frame_count: u32,
    /// 帧间隔时间（毫秒）
    pub frame_delay: Option<f32>,
}

impl From<&[image::Frame]> for AnimationInfo {
    fn from(frames: &[image::Frame]) -> Self {
        let frame_count = frames.len() as u32;
        let frame_delay = if frame_count > 0 {
            let avg_delay: u64 = frames
                .iter()
                .map(|frame| frame.delay().numer_denom_ms().0 as u64)
                .sum::<u64>() / frame_count.max(1) as u64;
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

impl From<Vec<image::Frame>> for AnimationInfo {
    fn from(frames: Vec<image::Frame>) -> Self {
        frames.as_slice().into()
    }
}
