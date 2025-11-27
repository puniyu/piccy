use crate::error::Error;
use serde::{Deserialize, Serialize};

use crate::Result;
use crate::common::AnimationInfo;
use base64::{Engine, engine::general_purpose::STANDARD};
pub use image::Rgb;
use image::{
    AnimationDecoder, DynamicImage,
    DynamicImage::ImageRgba8,
    GenericImageView, ImageFormat, ImageReader, RgbaImage,
    codecs::{gif::GifDecoder, webp::WebPDecoder},
    imageops::FilterType,
};
use std::{io::Cursor, path::Path, sync::Arc};

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
#[derive(Debug, Clone)]
pub enum FlipMode {
    /// 水平翻转
    Horizontal,
    /// 垂直翻转
    Vertical,
}

/// 图像拼接模式
#[derive(Debug, Clone)]
pub enum MergeMode {
    /// 水平拼接
    Horizontal,
    /// 垂直拼接
    Vertical,
}

#[derive(Default, Clone)]
pub struct ImageBuilder(pub(crate) Arc<Vec<u8>>);

impl ImageBuilder {
    pub fn new() -> Self {
        Self(Arc::new(Vec::new()))
    }

    pub fn with_path(&self, path: &Path) -> Result<Self> {
        let data = std::fs::read(path)?;
        Ok(Self(Arc::new(data)))
    }
    pub fn with_buffer(&self, buffer: Vec<u8>) -> Self {
        Self(Arc::new(buffer))
    }

    pub fn with_base64(&self, base64: &str) -> Result<Self> {
        let data = STANDARD.decode(base64)?;
        Ok(Self(Arc::new(data)))
    }
    
    pub fn build(&self) -> Image {
        Image(self.0.clone())
    }

}
#[derive(Clone)]
pub struct Image(Arc<Vec<u8>>);

impl Image {
    pub fn new(image: Vec<u8>) -> Self {
        Self(Arc::new(image))
    }

    /// 获取图像信息
    ///
    /// # 返回值
    /// 返回 [ImageInfo] 结构体，包含图像的宽度、高度、是否为动图、帧数和平均帧间隔等信息
    ///
    pub fn info(&self) -> Result<ImageInfo> {
        let image_data = self.0.as_slice();
        let cursor = Cursor::new(&image_data);
        let reader = ImageReader::new(cursor.clone()).with_guessed_format()?;
        match reader.format() {
            Some(ImageFormat::Gif) => {
                let image = reader.decode()?;
                let decoder = GifDecoder::new(cursor.clone())?;
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
                let decoder = WebPDecoder::new(cursor)?;
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

    /// 裁剪图像
    ///
    /// # 参数
    /// - `left`: 裁剪的左上角 X 坐标, 默认为 0
    /// - `top`: 裁剪的左上角 Y 坐标, 默认为 0
    /// - `width`: 裁剪的宽度, 默认为100
    /// - `height`: 裁剪的高度, 默认为100
    pub fn crop(
        &self,
        left: Option<u32>,
        top: Option<u32>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<u8>> {
        let left = left.unwrap_or(0);
        let top = top.unwrap_or(0);
        let width = width.unwrap_or(100);
        let height = height.unwrap_or(100);
        let image_data = self.0.as_slice();
        let cursor = Cursor::new(&image_data);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = reader.decode()?;
        let (image_width, image_height) = (image.width(), image.height());

        if left + width > image_width || top + height > image_height {
            return Err(Error::Other("裁剪区域超出图像范围".to_string()));
        };
        let cropped_img = image.view(left, top, width, height).to_image();
        let mut buffer = Vec::new();
        ImageRgba8(cropped_img).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(buffer)
    }

    /// 缩放图像
    ///
    /// ## 参数
    /// - `width`: 缩放后的宽度
    /// - `height`: 缩放后的高度
    pub fn resize(&self, width: u32, height: u32) -> Result<Vec<u8>> {
        let image_data = self.0.as_slice();
        let cursor = Cursor::new(&image_data);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = reader.decode()?;
        let resized_image = image
            .resize_exact(width, height, FilterType::Lanczos3)
            .into_rgba8();
        let mut buffer = Vec::new();
        ImageRgba8(resized_image).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(buffer)
    }

    /// 旋转图像
    ///
    /// ## 参数
    /// - `angle`: 旋转的角度
    pub fn rotate(&self, angle: f32) -> Result<Vec<u8>> {
        let image_data = self.0.as_slice();
        let cursor = Cursor::new(&image_data);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = reader.decode()?.to_rgba8();

        let rotated_image = imageproc::geometric_transformations::rotate_about_center(
            &image,
            angle.to_radians(),
            imageproc::geometric_transformations::Interpolation::Bilinear,
            image::Rgba([0, 0, 0, 0]),
        );

        let mut buffer = Vec::new();
        ImageRgba8(rotated_image).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(buffer)
    }

    /// 翻转图像
    ///
    /// ## 参数
    /// - `mode`: 翻转模式
    ///
    pub fn flip(&self, mode: FlipMode) -> Result<Vec<u8>> {
        let image_data = self.0.as_slice();
        let cursor = Cursor::new(&image_data);
        let reader = ImageReader::new(cursor).with_guessed_format()?;

        let image = reader.decode()?;

        let horizontal_image = match mode {
            FlipMode::Horizontal => image.fliph(),
            FlipMode::Vertical => image.flipv(),
        };
        let mut buffer = Vec::new();
        ImageRgba8(horizontal_image.to_rgba8())
            .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(buffer)
    }

    /// 灰度化图像
    pub fn grayscale(&self) -> Result<Vec<u8>> {
        let image_data = self.0.as_slice();
        let cursor = Cursor::new(&image_data);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = reader.decode()?;
        let grayscale_image = image.grayscale();
        let mut buffer = Vec::new();
        ImageRgba8(grayscale_image.to_rgba8())
            .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(buffer)
    }

    /// 反色图像
    pub fn invert(&self) -> Result<Vec<u8>> {
        let image_data = self.0.as_slice();
        let cursor = Cursor::new(&image_data);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let mut image = reader.decode()?.into_rgba8();
        image.pixels_mut().for_each(|pixel| {
            let [r, g, b, a] = pixel.0;
            pixel.0 = [255 - r, 255 - g, 255 - b, a];
        });
        let mut buffer = Vec::new();
        ImageRgba8(image).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(buffer)
    }

    /// 颜色蒙版
    ///
    /// ## 参数
    /// - `rgba`: rgba代码
    pub fn color_mask(&self, rgba: Rgb<u8>) -> Result<Vec<u8>> {
        let image_data = self.0.as_slice();
        let cursor = Cursor::new(&image_data);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let mut image = reader.decode()?.into_rgba8();

        let Rgb([r, g, b]) = rgba;

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
        Ok(buffer)
    }

    /// 幻影坦克
    ///
    /// ## 参数
    /// - `image`: 需要隐藏的图片
    pub fn mirage(&self, image: ImageBuilder) -> Result<Vec<u8>> {
        let wlight = 1.0f32;
        let blight = 0.5f32;

        let image2 = image.0;

        let info1 = self.info()?;
        let info2 = Image(image2.clone()).info()?;

        let w = info1.width.min(info2.width);
        let h = info1.height.min(info2.height);

        let img1 = ImageReader::new(Cursor::new(self.0.as_slice()))
            .with_guessed_format()?
            .decode()?;
        let img2 = ImageReader::new(Cursor::new(image2.as_slice()))
            .with_guessed_format()?
            .decode()?;

        let img1 = img1
            .resize_exact(w, h, image::imageops::CatmullRom)
            .to_rgba8();

        let img2 = img2
            .resize_exact(w, h, image::imageops::CatmullRom)
            .to_rgba8();

        let calculate_luminance = |pixel: &image::Rgba<u8>, light_factor: f32| -> f32 {
            (0.299 * pixel.0[0] as f32 + 0.587 * pixel.0[1] as f32 + 0.114 * pixel.0[2] as f32) * light_factor
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
                a.round() as u8
            ]);
        });

        let mut buffer = Vec::new();
        out_img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
        Ok(buffer)
    }
}

/// 拼接图片
///
/// ## 参数
/// - `images`: 需要拼接的图片实例
/// - `mode`: 拼接模式
pub fn image_merge(images: Vec<ImageBuilder>, mode: MergeMode) -> Result<Vec<u8>> {
    use image::imageops;
    if images.is_empty() {
        return Err(Error::Other("至少需要一个图片".to_string()));
    };

    let decoded_images = images
        .iter()
        .map(|image_data| {
            let cursor = Cursor::new(image_data.0.as_slice());
            let decoder = ImageReader::new(cursor).with_guessed_format()?;
            decoder.decode().map_err(Error::from)
        })
        .collect::<Result<Vec<DynamicImage>>>();

    let decoded_images = decoded_images?;

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
    Ok(buffer)
}
