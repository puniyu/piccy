use crate::{Result, types::image::ImageInfo};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use piccy_core::image::Image;

#[napi]
pub fn get_image_info(image_data: Buffer) -> Result<ImageInfo> {
    let buffer = image_data.to_vec();
    let info = Image::new().with_buffer(buffer).builder().info()?;
    Ok(info.into())
}
