use crate::{Result, types::image::ImageInfo};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use piccy_core::image;
use std::sync::LazyLock;

pub static IMAGE_CLIENT: LazyLock<image::Image> = LazyLock::new(image::Image::new);

#[napi]
pub struct Image(Vec<u8>);

#[napi]
impl Image {
    #[napi(constructor)]
    pub fn new(image_data: Buffer) -> Self {
        Image(image_data.to_vec())
    }

    /// 获取图像信息
    #[napi]
    pub fn info(&self) -> Result<ImageInfo> {
        let image_data = self.0.as_slice();
        let info = IMAGE_CLIENT
            .with_buffer(image_data.to_vec())
            .builder()
            .info()?;
        Ok(info.into())
    }

    /// 裁剪图像
    ///
    /// # 参数
    /// - `left`: 裁剪的左上角 X 坐标, 默认为 0
    /// - `top`: 裁剪的左上角 Y 坐标, 默认为 0
    /// - `width`: 裁剪的宽度, 默认为100
    /// - `height`: 裁剪的高度, 默认为100
    #[napi]
    pub fn crop(
        &self,
        left: Option<u32>,
        top: Option<u32>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Buffer> {
        let image_data = self.0.as_slice();
        let cropped_data = IMAGE_CLIENT
            .with_buffer(image_data.to_vec())
            .builder()
            .crop(left, top, width, height)?;
        Ok(Buffer::from(cropped_data))
    }
}
