use crate::error::Error;
use rayon::iter::ParallelIterator;

use crate::common::encode_gif;
use crate::{AnimationInfo, FlipMode, ImageFormat, ImageInfo, MergeMode, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use bytes::Bytes;
use image::{
    AnimationDecoder, DynamicImage,
    DynamicImage::ImageRgba8,
    Frame, GenericImageView, ImageReader, Rgb, RgbaImage,
    codecs::{gif::GifDecoder, webp::WebPDecoder},
    imageops::FilterType,
};
use rayon::iter::IntoParallelIterator;
use std::time::Duration;
use std::{io::Cursor, path::Path};

#[derive(Clone)]
pub struct Image(Bytes);

impl Image {
    /// 从文件路径加载图像
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let data = std::fs::read(path.as_ref())?;
        Ok(Self(data.into()))
    }

    /// 从字节数据加载图像
    pub fn from_bytes(bytes: impl Into<Bytes>) -> Self {
        Self(bytes.into())
    }

    /// 从 Base64 字符串加载图像
    pub fn from_base64(base64: impl Into<String>) -> Result<Self> {
        let data = STANDARD.decode(base64.into())?;
        Ok(Self(data.into()))
    }

    /// 获取内部字节数据
    pub fn into_bytes(self) -> Bytes {
        self.0
    }

    /// 获取图像信息
    pub fn info(&self) -> Result<ImageInfo> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        match reader.format() {
            Some(ImageFormat::Gif) => {
                let image = reader.decode()?;
                let decoder = GifDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;
                let animation_info = AnimationInfo::from(frames);
                Ok(ImageInfo {
                    width: image.width(),
                    height: image.height(),
                    is_multi_frame: animation_info.frame_count > 1,
                    frame_count: Some(animation_info.frame_count),
                    average_duration: animation_info.frame_delay,
                })
            }
            Some(ImageFormat::WebP) => {
                let image = reader.decode()?;
                let decoder = WebPDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;
                let animation_info = AnimationInfo::from(frames);
                Ok(ImageInfo {
                    width: image.width(),
                    height: image.height(),
                    is_multi_frame: animation_info.frame_count > 1,
                    frame_count: Some(animation_info.frame_count),
                    average_duration: animation_info.frame_delay,
                })
            }
            _ => {
                let image = reader.decode()?;
                Ok(ImageInfo {
                    width: image.width(),
                    height: image.height(),
                    is_multi_frame: false,
                    frame_count: Some(1),
                    average_duration: None,
                })
            }
        }
    }

    /// 编码为字节数据
    pub fn to_bytes(&self, format: ImageFormat) -> Result<Bytes> {
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = reader.decode()?;

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        match format {
            ImageFormat::Png => {
                use image::codecs::png::PngEncoder;
                let encoder = PngEncoder::new(&mut cursor);
                image.write_with_encoder(encoder)?;
            }
            ImageFormat::Jpeg => {
                use image::codecs::jpeg::JpegEncoder;
                let encoder = JpegEncoder::new(&mut cursor);
                image.write_with_encoder(encoder)?;
            }
            ImageFormat::WebP => {
                use image::codecs::webp::WebPEncoder;
                let encoder = WebPEncoder::new_lossless(&mut cursor);
                image.write_with_encoder(encoder)?;
            }
        }

        Ok(buffer.into())
    }

    /// 保存到文件
    pub fn save(&self, path: impl AsRef<Path>, format: ImageFormat) -> Result<()> {
        let bytes = self.to_bytes(format)?;
        std::fs::write(path.as_ref(), bytes)?;
        Ok(())
    }

    /// 编码为 Base64 字符串
    pub fn to_base64(&self, format: ImageFormat) -> Result<String> {
        let bytes = self.to_bytes(format)?;
        Ok(STANDARD.encode(bytes))
    }

    /// 裁剪图像
    ///
    /// # 参数
    /// - `x`: 裁剪的左上角 X 坐标
    /// - `y`: 裁剪的左上角 Y 坐标
    /// - `width`: 裁剪的宽度
    /// - `height`: 裁剪的高度
    pub fn crop(self, x: u32, y: u32, width: u32, height: u32) -> Result<Self> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = reader.decode()?;

        let (image_width, image_height) = (image.width(), image.height());
        if x + width > image_width || y + height > image_height {
            return Err(Error::Other("裁剪区域超出图像范围".to_string()));
        }

        let cropped = image.view(x, y, width, height).to_image();
        let mut buffer = Vec::new();
        ImageRgba8(cropped).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(Self(buffer.into()))
    }

    /// 缩放图像
    ///
    /// # 参数
    /// - `width`: 缩放后的宽度
    /// - `height`: 缩放后的高度
    pub fn resize(self, width: u32, height: u32) -> Result<Self> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = reader.decode()?;

        let resized = image.resize_exact(width, height, FilterType::Lanczos3);
        let mut buffer = Vec::new();
        ImageRgba8(resized.to_rgba8()).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(Self(buffer.into()))
    }

    /// 旋转图像
    ///
    /// # 参数
    /// - `angle`: 旋转的角度（度）
    pub fn rotate(self, angle: f32) -> Result<Self> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = reader.decode()?.to_rgba8();

        let rotated = imageproc::geometric_transformations::rotate_about_center(
            &image,
            angle.to_radians(),
            imageproc::geometric_transformations::Interpolation::Bilinear,
            image::Rgba([0, 0, 0, 0]),
        );

        let mut buffer = Vec::new();
        ImageRgba8(rotated).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(Self(buffer.into()))
    }

    /// 翻转图像
    ///
    /// # 参数
    /// - `mode`: 翻转模式
    pub fn flip(self, mode: Option<FlipMode>) -> Result<Self> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = reader.decode()?;

        let mode = mode.unwrap_or_default();
        let flipped = match mode {
            FlipMode::Horizontal => image.fliph(),
            FlipMode::Vertical => image.flipv(),
        };

        let mut buffer = Vec::new();
        ImageRgba8(flipped.to_rgba8()).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(Self(buffer.into()))
    }

    /// 灰度化图像
    pub fn grayscale(self) -> Result<Self> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = reader.decode()?;

        let gray = image.grayscale();
        let mut buffer = Vec::new();
        ImageRgba8(gray.to_rgba8()).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(Self(buffer.into()))
    }

    /// 反色图像
    pub fn invert(self) -> Result<Self> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let mut image = reader.decode()?.into_rgba8();

        image.pixels_mut().for_each(|pixel| {
            let [r, g, b, a] = pixel.0;
            pixel.0 = [255 - r, 255 - g, 255 - b, a];
        });

        let mut buffer = Vec::new();
        ImageRgba8(image).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(Self(buffer.into()))
    }

    /// 颜色蒙版
    ///
    /// # 参数
    /// - `color`: RGB 颜色值
    pub fn color_mask(self, color: Rgb<u8>) -> Result<Self> {
        use image::ImageFormat;
        let Rgb([r, g, b]) = color;

        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let mut image = reader.decode()?.into_rgba8();

        image.pixels_mut().for_each(|pixel| {
            let [red, green, blue, alpha] = pixel.0;
            let src_alpha = alpha as f32 / 255.0;
            pixel.0 = [
                ((r as f32) * src_alpha * 0.5 + (red as f32) * (1.0 - src_alpha * 0.5)).round()
                    as u8,
                ((g as f32) * src_alpha * 0.5 + (green as f32) * (1.0 - src_alpha * 0.5)).round()
                    as u8,
                ((b as f32) * src_alpha * 0.5 + (blue as f32) * (1.0 - src_alpha * 0.5)).round()
                    as u8,
                alpha,
            ];
        });

        let mut buffer = Vec::new();
        image.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(Self(buffer.into()))
    }

    /// 幻影坦克
    ///
    /// # 参数
    /// - `hidden`: 需要隐藏的图片
    pub fn mirage(self, hidden: Self) -> Result<Self> {
        use image::ImageFormat;
        let wlight = 1.0f32;
        let blight = 0.5f32;

        let info1 = self.info()?;
        let info2 = hidden.info()?;

        let w = info1.width.min(info2.width);
        let h = info1.height.min(info2.height);

        let cursor1 = Cursor::new(&self.0);
        let img1 = ImageReader::new(cursor1).with_guessed_format()?.decode()?;

        let cursor2 = Cursor::new(&hidden.0);
        let img2 = ImageReader::new(cursor2).with_guessed_format()?.decode()?;

        let img1 = img1
            .resize_exact(w, h, image::imageops::CatmullRom)
            .to_rgba8();
        let img2 = img2
            .resize_exact(w, h, image::imageops::CatmullRom)
            .to_rgba8();

        let calculate_luminance = |pixel: &image::Rgba<u8>, light_factor: f32| -> f32 {
            (0.299 * pixel.0[0] as f32 + 0.587 * pixel.0[1] as f32 + 0.114 * pixel.0[2] as f32)
                * light_factor
        };

        let mut out_img = RgbaImage::new(w, h);
        out_img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let wpixel = img1.get_pixel(x, y);
            let bpixel = img2.get_pixel(x, y);

            let wc = calculate_luminance(wpixel, wlight);
            let bc = calculate_luminance(bpixel, blight);

            let a = (255.0 - wc + bc).clamp(0.0, 255.0);
            let r = if a > 0.0 {
                (bc / a * 255.0).min(255.0)
            } else {
                0.0
            };

            *pixel = image::Rgba([
                r.round() as u8,
                r.round() as u8,
                r.round() as u8,
                a.round() as u8,
            ]);
        });

        let mut buffer = Vec::new();
        out_img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(Self(buffer.into()))
    }

    /// 分离动图帧
    ///
    pub fn split(self) -> Result<Vec<Self>> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;

        match reader.format() {
            Some(ImageFormat::Gif) => {
                let decoder = GifDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;

                if frames.len() <= 1 {
                    return Err(Error::Other("当前不是动图".to_string()));
                }
                frames
                    .into_iter()
                    .map(|frame| {
                        let mut buffer = Vec::new();
                        let img = ImageRgba8(frame.into_buffer());
                        img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
                        Ok(Image(buffer.into()))
                    })
                    .collect()
            }
            Some(ImageFormat::WebP) => {
                let decoder = WebPDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;

                if frames.len() <= 1 {
                    return Err(Error::Other("当前不是动图".to_string()));
                }

                frames
                    .into_iter()
                    .map(|frame| {
                        let mut buffer = Vec::new();
                        let img = ImageRgba8(frame.into_buffer());
                        img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
                        Ok(Self(buffer.into()))
                    })
                    .collect()
            }
            _ => Err(Error::Other("当前不是动图".to_string())),
        }
    }

    /// 反转动图帧顺序
    pub fn reverse(self) -> Result<Self> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;

        match reader.format() {
            Some(ImageFormat::Gif) => {
                let decoder = GifDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;

                if frames.len() <= 1 {
                    return Err(Error::Other("当前不是动图".to_string()));
                }

                let reversed_frames: Vec<Frame> = frames.into_iter().rev().collect();
                encode_gif(reversed_frames).map(Self)
            }
            Some(ImageFormat::WebP) => {
                let decoder = WebPDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;

                if frames.len() <= 1 {
                    return Err(Error::Other("当前不是动图".to_string()));
                }

                let reversed_frames: Vec<Frame> = frames.into_iter().rev().collect();
                encode_gif(reversed_frames).map(Self)
            }
            _ => Err(Error::Other("当前不是动图".to_string())),
        }
    }

    /// 修改动图帧间隔
    ///
    /// # 参数
    /// - `duration`: 帧间隔时间
    pub fn change_duration(self, duration: Duration) -> Result<Self> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;

        match reader.format() {
            Some(ImageFormat::Gif) => {
                let decoder = GifDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;

                if frames.len() <= 1 {
                    return Err(Error::Other("当前不是动图".to_string()));
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
                encode_gif(frames).map(Self)
            }
            Some(ImageFormat::WebP) => {
                let decoder = WebPDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;

                if frames.len() <= 1 {
                    return Err(Error::Other("当前不是动图".to_string()));
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
                encode_gif(frames).map(Self)
            }
            _ => Err(Error::Other("当前不是动图".to_string())),
        }
    }

    /// 拼接图片
    ///
    /// # 参数
    /// - `images`: 需要拼接的其他图片
    /// - `mode`: 拼接模式
    pub fn merge(self, images: Vec<&Image>, mode: Option<MergeMode>) -> Result<Self> {
        use image::ImageFormat;
        use image::imageops;
        let mut all_images = vec![self];
        all_images.extend(images.iter().map(|img| (*img).clone()));

        let decoded_images: Result<Vec<DynamicImage>> = all_images
            .into_iter()
            .map(|img| {
                let cursor = Cursor::new(&img.0);
                let reader = ImageReader::new(cursor).with_guessed_format()?;
                reader.decode().map_err(Error::from)
            })
            .collect();

        let decoded_images = decoded_images?;

        if decoded_images.is_empty() {
            return Err(Error::Other("没有有效的图像数据".to_string()));
        }
        let mode = mode.unwrap_or_default();

        let merged_image = match mode {
            MergeMode::Horizontal => {
                let min_height = decoded_images
                    .iter()
                    .map(|img| img.height())
                    .min()
                    .unwrap_or(0);
                let total_width: u32 = decoded_images
                    .iter()
                    .map(|img| {
                        let scale = min_height as f32 / img.height() as f32;
                        (img.width() as f32 * scale) as u32
                    })
                    .sum();
                let mut merged_image = ImageRgba8(RgbaImage::new(total_width, min_height));
                let mut current_x: u32 = 0;
                for image in &decoded_images {
                    let scale = min_height as f32 / image.height() as f32;
                    let scaled_width = (image.width() as f32 * scale) as u32;
                    let resized_image =
                        image.resize_exact(scaled_width, min_height, FilterType::Triangle);
                    imageops::overlay(&mut merged_image, &resized_image, current_x as i64, 0);
                    current_x += scaled_width;
                }
                merged_image
            }
            MergeMode::Vertical => {
                let max_width = decoded_images
                    .iter()
                    .map(|img| img.width())
                    .max()
                    .unwrap_or(0);
                let total_height = decoded_images.iter().map(|img| img.height()).sum();
                let mut merged_image = ImageRgba8(RgbaImage::new(max_width, total_height));
                let mut current_y = 0;

                for image in &decoded_images {
                    let resized_image =
                        image.resize_exact(max_width, image.height(), FilterType::Triangle);
                    imageops::overlay(&mut merged_image, &resized_image, 0, current_y as i64);
                    current_y += resized_image.height();
                }
                merged_image
            }
        };

        let mut buffer = Vec::new();
        merged_image
            .into_rgba8()
            .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(Self(buffer.into()))
    }

    /// GIF 拼接
    ///
    /// # 参数
    /// - `images`: 其他图片列表
    /// - `duration`: 帧间隔时间
    pub fn merge_gif(self, images: Vec<&Image>, duration: Option<Duration>) -> Result<Self> {
        let mut all_images = vec![self];
        all_images.extend(images.iter().map(|img| (*img).clone()));

        if all_images.is_empty() {
            return Err(Error::Other("至少需要一个图片".to_string()));
        }

        let first_image = all_images.first().unwrap();
        let info = first_image.info()?;
        let width = info.width;
        let height = info.height;
        let frame_duration = duration.unwrap_or(Duration::from_millis(20));

        let frames: Result<Vec<Frame>> = all_images
            .into_par_iter()
            .map(|image| {
                let cursor = Cursor::new(&image.0);
                let img = ImageReader::new(cursor).with_guessed_format()?.decode()?;
                let resized_image = img.resize_exact(width, height, FilterType::Lanczos3);
                Ok(Frame::from_parts(
                    resized_image.into_rgba8(),
                    0,
                    0,
                    image::Delay::from_saturating_duration(frame_duration),
                ))
            })
            .collect();

        encode_gif(frames?).map(Self)
    }
}
