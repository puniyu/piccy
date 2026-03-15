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
    error::{
        ImageError, ImageFormatHint, ParameterError, ParameterErrorKind, UnsupportedError,
        UnsupportedErrorKind,
    },
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
                let decoder = GifDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;
                let animation_info = AnimationInfo::from(&frames);
                let (width, height) = frames.first()
                    .map(|f| (f.buffer().width(), f.buffer().height()))
                    .unwrap_or((0, 0));
                Ok(ImageInfo {
                    width,
                    height,
                    is_multi_frame: animation_info.frame_count > 1,
                    frame_count: Some(animation_info.frame_count),
                    average_duration: animation_info.frame_delay,
                })
            }
            Some(ImageFormat::WebP) => {
                let decoder = WebPDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;
                let animation_info = AnimationInfo::from(&frames);
                let (width, height) = frames.first()
                    .map(|f| (f.buffer().width(), f.buffer().height()))
                    .unwrap_or((0, 0));
                Ok(ImageInfo {
                    width,
                    height,
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
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Result<Self> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = reader.decode()?;

        let (image_width, image_height) = (image.width(), image.height());
        if x + width > image_width || y + height > image_height {
            return Err(ImageError::Parameter(ParameterError::from_kind(
                ParameterErrorKind::DimensionMismatch,
            ))
            .into());
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
    pub fn resize(&self, width: u32, height: u32) -> Result<Self> {
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
    pub fn rotate(&self, angle: f32) -> Result<Self> {
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
    pub fn flip(&self, mode: Option<FlipMode>) -> Result<Self> {
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
    pub fn grayscale(&self) -> Result<Self> {
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
    pub fn invert(&self) -> Result<Self> {
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
    pub fn color_mask(&self, color: Rgb<u8>) -> Result<Self> {
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
    pub fn mirage(&self, hidden: &Self) -> Result<Self> {
        let cursor1 = Cursor::new(&self.0);
        let img1 = ImageReader::new(cursor1).with_guessed_format()?.decode()?;

        let cursor2 = Cursor::new(&hidden.0);
        let img2 = ImageReader::new(cursor2).with_guessed_format()?.decode()?;

        let w = img1.width().min(img2.width());
        let h = img1.height().min(img2.height());

        let img1_rgba = img1.resize_exact(w, h, FilterType::Lanczos3).to_rgba8();
        let img2_rgba = img2.resize_exact(w, h, FilterType::Lanczos3).to_rgba8();


        let calc_avg_luminance = |img: &RgbaImage| -> f32 {
            let sum: f32 = img.pixels().take(1000).map(|p| {
                0.299 * p.0[0] as f32 + 0.587 * p.0[1] as f32 + 0.114 * p.0[2] as f32
            }).sum();
            sum / 1000.0
        };

        let white_avg = calc_avg_luminance(&img1_rgba);
        let black_avg = calc_avg_luminance(&img2_rgba);

        let white_light = if white_avg < 100.0 {
            1.2
        } else if white_avg > 180.0 {
            0.9 
        } else {
            1.0
        };

        let black_light = if black_avg < 80.0 {
            0.4
        } else if black_avg > 150.0 {
            0.6 
        } else {
            0.5
        };

        self.mirage_internal(img1_rgba, img2_rgba, white_light, black_light)
    }

    /// 幻影坦克内部实现
    fn mirage_internal(
        &self,
        img1: RgbaImage,
        img2: RgbaImage,
        white_light: f32,
        black_light: f32,
    ) -> Result<Self> {
        use image::ImageFormat;

        let w = img1.width();
        let h = img1.height();

        let is_color = {
            let check_color = |img: &RgbaImage| -> bool {
                img.pixels().take(100).any(|p| {
                    let [r, g, b, _] = p.0;
                    r != g || g != b
                })
            };
            check_color(&img1) || check_color(&img2)
        };

        const LUM_R: f32 = 0.299;
        const LUM_G: f32 = 0.587;
        const LUM_B: f32 = 0.114;

        use rayon::prelude::*;
        use std::sync::Arc;

        let img1 = Arc::new(img1);
        let img2 = Arc::new(img2);

        let pixels: Vec<_> = (0..h)
            .into_par_iter()
            .flat_map(|y| {
                let img1 = Arc::clone(&img1);
                let img2 = Arc::clone(&img2);
                (0..w).into_par_iter().map(move |x| {
                    let wpixel = img1.get_pixel(x, y);
                    let bpixel = img2.get_pixel(x, y);

                    if is_color {
                        let mut result = [0u8; 4];
                        for (i, r) in result.iter_mut().enumerate().take(3) {
                            let wc = wpixel.0[i] as f32 * white_light;
                            let bc = bpixel.0[i] as f32 * black_light;
                            let a = (255.0 - wc + bc).clamp(1.0, 255.0);
                            *r = ((bc / a * 255.0).min(255.0).round()) as u8;
                        }
                        let wc_avg = (LUM_R * wpixel.0[0] as f32
                            + LUM_G * wpixel.0[1] as f32
                            + LUM_B * wpixel.0[2] as f32)
                            * white_light;
                        let bc_avg = (LUM_R * bpixel.0[0] as f32
                            + LUM_G * bpixel.0[1] as f32
                            + LUM_B * bpixel.0[2] as f32)
                            * black_light;
                        result[3] = ((255.0 - wc_avg + bc_avg).clamp(0.0, 255.0).round()) as u8;
                        (x, y, image::Rgba(result))
                    } else {
                        let wc = (LUM_R * wpixel.0[0] as f32
                            + LUM_G * wpixel.0[1] as f32
                            + LUM_B * wpixel.0[2] as f32)
                            * white_light;
                        let bc = (LUM_R * bpixel.0[0] as f32
                            + LUM_G * bpixel.0[1] as f32
                            + LUM_B * bpixel.0[2] as f32)
                            * black_light;

                        let a = (255.0 - wc + bc).clamp(0.0, 255.0);
                        let r = if a > 0.0 {
                            (bc / a * 255.0).min(255.0)
                        } else {
                            0.0
                        };

                        let r_u8 = r.round() as u8;
                        (x, y, image::Rgba([r_u8, r_u8, r_u8, a.round() as u8]))
                    }
                })
            })
            .collect();

        let mut out_img = RgbaImage::new(w, h);
        for (x, y, pixel) in pixels {
            out_img.put_pixel(x, y, pixel);
        }

        let mut buffer = Vec::new();
        out_img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(Self(buffer.into()))
    }

    /// 分离动图帧
    ///
    pub fn split(&self) -> Result<Vec<Self>> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;

        match reader.format() {
            Some(ImageFormat::Gif) => {
                let decoder = GifDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;

                if frames.len() <= 1 {
                    return Err(
                        ImageError::Unsupported(UnsupportedError::from_format_and_kind(
                            ImageFormatHint::Unknown,
                            UnsupportedErrorKind::GenericFeature("animation".to_string()),
                        ))
                        .into(),
                    );
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
                    return Err(
                        ImageError::Unsupported(UnsupportedError::from_format_and_kind(
                            ImageFormatHint::Unknown,
                            UnsupportedErrorKind::GenericFeature("animation".to_string()),
                        ))
                        .into(),
                    );
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
            _ => Err(
                ImageError::Unsupported(UnsupportedError::from_format_and_kind(
                    ImageFormatHint::Unknown,
                    UnsupportedErrorKind::GenericFeature("animation".to_string()),
                ))
                .into(),
            ),
        }
    }

    /// 反转动图帧顺序
    pub fn reverse(&self) -> Result<Self> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;

        match reader.format() {
            Some(ImageFormat::Gif) => {
                let decoder = GifDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;

                if frames.len() <= 1 {
                    return Err(
                        ImageError::Unsupported(UnsupportedError::from_format_and_kind(
                            ImageFormatHint::Unknown,
                            UnsupportedErrorKind::GenericFeature("animation".to_string()),
                        ))
                        .into(),
                    );
                }

                let reversed_frames: Vec<Frame> = frames.into_iter().rev().collect();
                encode_gif(reversed_frames).map(Self)
            }
            Some(ImageFormat::WebP) => {
                let decoder = WebPDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;

                if frames.len() <= 1 {
                    return Err(
                        ImageError::Unsupported(UnsupportedError::from_format_and_kind(
                            ImageFormatHint::Unknown,
                            UnsupportedErrorKind::GenericFeature("animation".to_string()),
                        ))
                        .into(),
                    );
                }

                let reversed_frames: Vec<Frame> = frames.into_iter().rev().collect();
                encode_gif(reversed_frames).map(Self)
            }
            _ => Err(
                ImageError::Unsupported(UnsupportedError::from_format_and_kind(
                    ImageFormatHint::Unknown,
                    UnsupportedErrorKind::GenericFeature("animation".to_string()),
                ))
                .into(),
            ),
        }
    }

    /// 修改动图帧间隔
    ///
    /// # 参数
    /// - `duration`: 帧间隔时间
    pub fn change_duration(&self, duration: Duration) -> Result<Self> {
        use image::ImageFormat;
        let cursor = Cursor::new(&self.0);
        let reader = ImageReader::new(cursor).with_guessed_format()?;

        match reader.format() {
            Some(ImageFormat::Gif) => {
                let decoder = GifDecoder::new(Cursor::new(&self.0))?;
                let frames = decoder.into_frames().collect_frames()?;

                if frames.len() <= 1 {
                    return Err(
                        ImageError::Unsupported(UnsupportedError::from_format_and_kind(
                            ImageFormatHint::Unknown,
                            UnsupportedErrorKind::GenericFeature("animation".to_string()),
                        ))
                        .into(),
                    );
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
                    return Err(
                        ImageError::Unsupported(UnsupportedError::from_format_and_kind(
                            ImageFormatHint::Unknown,
                            UnsupportedErrorKind::GenericFeature("animation".to_string()),
                        ))
                        .into(),
                    );
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
            _ => Err(
                ImageError::Unsupported(UnsupportedError::from_format_and_kind(
                    ImageFormatHint::Unknown,
                    UnsupportedErrorKind::GenericFeature("animation".to_string()),
                ))
                .into(),
            ),
        }
    }

    /// 拼接图片
    ///
    /// # 参数
    /// - `images`: 需要拼接的其他图片
    /// - `mode`: 拼接模式
    pub fn merge(&self, images: Vec<&Image>, mode: Option<MergeMode>) -> Result<Self> {
        use image::ImageFormat;
        use image::imageops;
        let mut all_images: Vec<&Image> = Vec::with_capacity(1 + images.len());
        all_images.push(self);
        all_images.extend(images);

        let decoded_images: Result<Vec<DynamicImage>> = all_images
            .iter()
            .map(|img| {
                let cursor = Cursor::new(&img.0);
                let reader = ImageReader::new(cursor).with_guessed_format()?;
                reader.decode().map_err(Error::from)
            })
            .collect();

        let decoded_images = decoded_images?;

        if decoded_images.is_empty() {
            return Err(Error::Other("No valid image data".to_string()));
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
    pub fn merge_gif(&self, images: Vec<&Image>, duration: Option<Duration>) -> Result<Self> {
        let mut all_images: Vec<&Image> = Vec::with_capacity(1 + images.len());
        all_images.push(self);
        all_images.extend(images);

        if all_images.is_empty() {
            return Err(Error::Other("At least one image is required".to_string()));
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
