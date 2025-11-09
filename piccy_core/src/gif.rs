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

        let reversed_frames: Vec<Frame> = frames.into_iter().rev().collect();

        let mut buffer = Vec::new();
        {
            let mut encoder = image::codecs::gif::GifEncoder::new(&mut buffer);
            encoder.set_repeat(Repeat::Infinite)?;
            encoder.encode_frames(reversed_frames.into_iter())?;
        }

        Ok(buffer)
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
        let modified_frames: Vec<Frame> = frames
            .into_iter()
            .map(|frame| {
                let left = frame.left();
                let top = frame.top();
                let buffer = frame.into_buffer();
                Frame::from_parts(buffer, left, top, delay)
            })
            .collect();

        let mut buffer = Vec::new();
        {
            let mut encoder = image::codecs::gif::GifEncoder::new(&mut buffer);
            encoder.set_repeat(Repeat::Infinite)?;
            encoder.encode_frames(modified_frames.into_iter())?;
        }

        Ok(buffer)
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

    let cursor = Cursor::new(images.first().unwrap().0.as_slice());
    let reader = ImageReader::new(cursor).with_guessed_format()?;
    let image_info = reader.into_dimensions()?;
    let (width, height) = image_info;

    let frames = images
        .into_par_iter()
        .map(|img_builder| {
            let data = img_builder.0.as_slice();
            let cursor = Cursor::new(data);
            let reader = ImageReader::new(cursor).with_guessed_format()?;
            let image = reader.decode()?;
            let resized_image =
                image.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
            let frame = Frame::from_parts(
                resized_image.into(),
                0,
                0,
                image::Delay::from_saturating_duration(
                    duration.unwrap_or(Duration::from_secs(0.02 as u64)),
                ),
            );
            Ok(frame)
        })
        .collect::<Result<Vec<Frame>>>();
    let frames = frames?;

    let mut buffer = Vec::new();
    {
        let mut encoder = image::codecs::gif::GifEncoder::new(&mut buffer);
        encoder.set_repeat(Repeat::Infinite)?;
        encoder.encode_frames(frames.into_iter())?;
    }
    Ok(buffer)
}
