use crate::image::ImageBuilder;
use crate::{Error, Result};
use image::codecs::gif::Repeat;
use image::{
    AnimationDecoder, DynamicImage::ImageRgba8, Frame, ImageReader, codecs::gif::GifDecoder,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::time::Duration;
use std::{io::Cursor, sync::Arc};

#[derive(Clone)]
pub struct Gif(Arc<Vec<u8>>);

impl Gif {
    pub fn new(image: ImageBuilder) -> Self {
        Self(image.0)
    }

    /// gif分解
    pub fn split(&self) -> Result<Vec<Vec<u8>>> {
        let image_data = self.0.as_slice();
        let cursor = Cursor::new(&image_data);
        let decoder = GifDecoder::new(cursor)?;
        let frames = decoder.into_frames().collect_frames()?;

        if frames.len() <= 1 {
            return Err(Error::Other("当前不是动图".to_string()))?;
        }

        let result = frames
            .into_iter()
            .map(|frame| {
                let mut buffer = Vec::new();
                let img = ImageRgba8(frame.into_buffer());
                img.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)?;
                Ok(buffer)
            })
            .collect::<Result<Vec<Vec<u8>>>>();
        let image = result?;
        Ok(image)
    }

    /// gif倒放
    pub fn reverse(&self) -> Result<Vec<u8>> {
        let image_data = self.0.as_slice();
        let cursor = Cursor::new(&image_data);
        let decoder = GifDecoder::new(cursor)?;
        let frames = decoder.into_frames().collect_frames()?;

        if frames.len() <= 1 {
            return Err(Error::Other("当前不是动图".to_string()))?;
        }

        let frames: Vec<Frame> = frames.into_iter().rev().collect();
        encode_gif(frames)
    }

    /// gif变速
    ///
    /// ## 参数
    /// - `duration`: 帧间隔时间(单位: 秒)
    ///
    pub fn change_duration(&self, duration: Duration) -> Result<Vec<u8>> {
        let image_data = self.0.as_slice();
        let cursor = Cursor::new(&image_data);
        let decoder = GifDecoder::new(cursor)?;
        let frames = decoder.into_frames().collect_frames()?;

        if frames.len() <= 1 {
            return Err(Error::Other("当前不是动图".to_string()))?;
        }

        let delay = image::Delay::from_saturating_duration(duration);
        let frames: Vec<Frame> = frames
            .into_iter()
            .map(|frame| {
                let left = frame.left();
                let top = frame.top();
                let buffer = frame.into_buffer();
                Frame::from_parts(buffer, left, top, delay)
            })
            .collect();
        encode_gif(frames)
    }
}

/// gif拼接
///
/// ## 参数
/// - `images`: 图片
/// - `duration`: 帧间隔时间
///
pub fn gif_merge(images: Vec<ImageBuilder>, duration: Option<Duration>) -> Result<Vec<u8>> {
    if images.is_empty() {
        return Err(Error::Other("至少需要一个图片".to_string()));
    }

    let first_image_data = images
        .first()
        .ok_or_else(|| Error::Other("图片列表为空".to_string()))?
        .0
        .as_slice();
    let (width, height) = {
        let cursor = Cursor::new(first_image_data);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        reader.into_dimensions()?
    };
    let frame_duration = duration.unwrap_or(Duration::from_millis(20));

    let frames: Result<Vec<Frame>> = images
        .into_par_iter()
        .map(|img_builder| {
            let data = img_builder.0.as_slice();
            let cursor = Cursor::new(data);
            let image = ImageReader::new(cursor).with_guessed_format()?.decode()?;
            let resized_image =
                image.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

            Ok(Frame::from_parts(
                resized_image.into(),
                0,
                0,
                image::Delay::from_saturating_duration(frame_duration),
            ))
        })
        .collect();
    let frames = frames?;
    encode_gif(frames)
}

fn encode_gif(frames: Vec<Frame>) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    {
        let mut encoder = image::codecs::gif::GifEncoder::new(&mut buffer);
        encoder.set_repeat(Repeat::Infinite)?;
        encoder.encode_frames(frames.into_iter())?;
    }
    Ok(buffer)
}
