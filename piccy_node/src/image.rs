use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use crate::{Result, types::image::ImageInfo};
use piccy_core::image;

#[napi]
pub fn get_image_info(image_data: Buffer) -> Result<ImageInfo> {
	let data = image_data.to_vec();
	let info = image::get_image_info(data)?;
	Ok(info.into())
}