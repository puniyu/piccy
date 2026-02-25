mod types;

type Result<T> = napi::Result<T>;

use crate::types::{ImageFormat, Rgb};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use std::time::Duration;
use types::{FlipMode, ImageInfo, MergeMode};

/// 图像处理类
#[napi]
pub struct Image {
    inner: piccy_core::Image,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            inner: piccy_core::Image::from_bytes(vec![]),
        }
    }
}

#[napi]
impl Image {
    /// 创建图像实例
    #[napi(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// 从字节数据加载图像
    ///
    /// # 参数
    /// - `bytes`: 字节数据
    #[napi(factory)]
    pub fn from_bytes(bytes: Buffer) -> Result<Self> {
        let inner = piccy_core::Image::from_bytes(bytes.to_vec());
        Ok(Self { inner })
    }

    /// 从文件路径加载图像
    ///
    /// # 参数
    /// - `path`: 文件路径
    #[napi(factory)]
    pub fn from_path(path: String) -> Result<Self> {
        let inner = piccy_core::Image::from_path(&path)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 从 Base64 字符串加载图像
    ///
    /// # 参数
    /// - `base64`: Base64 编码的图像数据
    #[napi(factory)]
    pub fn from_base64(base64: String) -> Result<Self> {
        let inner = piccy_core::Image::from_base64(&base64)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 获取图像信息
    #[napi]
    pub fn info(&self) -> Result<ImageInfo> {
        let result = self
            .inner
            .info()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(result.into())
    }

    /// 编码为字节数据
    ///
    /// # 参数
    /// - `format`: 输出格式，默认 PNG
    #[napi]
    pub fn to_bytes(&self, format: Option<ImageFormat>) -> Result<Buffer> {
        let format = format
            .map(Into::into)
            .unwrap_or(piccy_core::ImageFormat::Png);
        let result = self
            .inner
            .to_bytes(format)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(result.to_vec().into())
    }

    /// 编码为 Base64 字符串
    ///
    /// # 参数
    /// - `format`: 输出格式，默认 PNG
    #[napi]
    pub fn to_base64(&self, format: Option<ImageFormat>) -> Result<String> {
        let format = format
            .map(Into::into)
            .unwrap_or(piccy_core::ImageFormat::Png);
        let result = self
            .inner
            .to_base64(format)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(result)
    }

    /// 保存到文件
    ///
    /// # 参数
    /// - `path`: 文件路径
    /// - `format`: 输出格式，默认 PNG
    #[napi]
    pub fn save(&self, path: String, format: Option<ImageFormat>) -> Result<()> {
        let format = format
            .map(Into::into)
            .unwrap_or(piccy_core::ImageFormat::Png);
        self.inner
            .save(&path, format)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(())
    }

    /// 裁剪图像
    ///
    /// # 参数
    /// - `x`: 裁剪的左上角 X 坐标
    /// - `y`: 裁剪的左上角 Y 坐标
    /// - `width`: 裁剪的宽度
    /// - `height`: 裁剪的高度
    #[napi]
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Result<Self> {
        let inner = self
            .inner
            .clone()
            .crop(x, y, width, height)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 缩放图像
    ///
    /// # 参数
    /// - `width`: 缩放后的宽度
    /// - `height`: 缩放后的高度
    #[napi]
    pub fn resize(&self, width: u32, height: u32) -> Result<Image> {
        let inner = self
            .inner
            .clone()
            .resize(width, height)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 旋转图像
    ///
    /// # 参数
    /// - `angle`: 旋转的角度（度）
    #[napi]
    pub fn rotate(&self, angle: f64) -> Result<Image> {
        let inner = self
            .inner
            .clone()
            .rotate(angle as f32)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 翻转图像
    ///
    /// # 参数
    /// - `mode`: 翻转模式，默认水平翻转
    #[napi]
    pub fn flip(&self, mode: Option<FlipMode>) -> Result<Self> {
        let inner = self
            .inner
            .clone()
            .flip(mode.map(Into::into))
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 灰度化图像
    #[napi]
    pub fn grayscale(&self) -> Result<Image> {
        let inner = self
            .inner
            .clone()
            .grayscale()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 反色图像
    #[napi]
    pub fn invert(&self) -> Result<Image> {
        let inner = self
            .inner
            .clone()
            .invert()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 颜色蒙版
    ///
    /// # 参数
    /// - `rgb`: RGB 颜色值，格式为 "r,g,b"
    #[napi]
    pub fn color_mask(&self, rgb: Rgb) -> Result<Image> {
        let inner = self
            .inner
            .clone()
            .color_mask(rgb.into())
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 幻影坦克
    ///
    /// # 参数
    /// - `hidden`: 需要隐藏的图片
    #[napi]
    pub fn mirage(&self, hidden: &Image) -> Result<Image> {
        let inner = self
            .inner
            .clone()
            .mirage(hidden.inner.clone())
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 分离动图帧
    ///
    /// # 返回值
    /// 返回所有帧的图像数组
    #[napi]
    pub fn split(&self) -> Result<Vec<Image>> {
        let frames = self
            .inner
            .clone()
            .split()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(frames.into_iter().map(|inner| Self { inner }).collect())
    }

    /// 反转动图帧顺序
    #[napi]
    pub fn reverse(&self) -> Result<Image> {
        let inner = self
            .inner
            .clone()
            .reverse()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 修改动图帧间隔
    ///
    /// # 参数
    /// - `duration`: 帧间隔时间（秒）
    #[napi]
    pub fn change_duration(&self, duration: u32) -> Result<Image> {
        let inner = self
            .inner
            .clone()
            .change_duration(Duration::from_secs(duration as u64))
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// 拼接图片
    ///
    /// # 参数
    /// - `images`: 需要拼接的其他图片
    /// - `mode`: 拼接模式，默认水平拼接
    #[napi]
    pub fn merge(&self, images: Vec<&Image>, mode: Option<MergeMode>) -> Result<Self> {
        let inner_images: Vec<&piccy_core::Image> = images.iter().map(|img| &img.inner).collect();
        let inner = self
            .inner
            .clone()
            .merge(inner_images, mode.map(Into::into))
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// GIF 拼接
    ///
    /// # 参数
    /// - `images`: 其他图片数组
    /// - `duration`: 帧间隔时间（秒），默认 1 秒
    #[napi]
    pub fn merge_gif(&self, images: Vec<&Image>, duration: Option<u32>) -> Result<Self> {
        let inner_images: Vec<&piccy_core::Image> = images.iter().map(|img| &img.inner).collect();
        let delay = duration.map(|d| Duration::from_secs(d as u64));
        let inner = self
            .inner
            .clone()
            .merge_gif(inner_images, delay)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }
}
