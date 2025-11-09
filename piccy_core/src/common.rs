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
        let duration = if frame_count > 1 {
            let total_duration = frames
                .iter()
                .map(|frame| frame.delay().numer_denom_ms().0)
                .sum::<u32>();
            Some((total_duration as f32 / 1000.0) / frame_count as f32)
        } else {
            Some(0.0f32)
        };
        Self {
            frame_count,
            frame_delay: duration,
        }
    }
}

impl From<Vec<image::Frame>> for AnimationInfo {
    fn from(frames: Vec<image::Frame>) -> Self {
        frames.as_slice().into()
    }
}
