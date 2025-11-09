use napi_derive::napi;
use piccy_core::image::Rgb;

#[derive(Debug, Clone)]
#[napi]
pub enum FlipMode {
	/// 水平翻转
	Horizontal,
	/// 垂直翻转
	Vertical,
}

impl From<FlipMode> for piccy_core::image::FlipMode {
	fn from(other: FlipMode) -> Self {
		match other {
			FlipMode::Horizontal => piccy_core::image::FlipMode::Horizontal,
			FlipMode::Vertical => piccy_core::image::FlipMode::Vertical,
		}
	}
}

/// 图像拼接模式
#[derive(Debug, Clone)]
#[napi]
pub enum MergeMode {
	/// 水平拼接
	Horizontal,
	/// 垂直拼接
	Vertical,
}

impl From<MergeMode> for piccy_core::image::MergeMode {
	fn from(other: MergeMode) -> Self {
		match other {
			MergeMode::Horizontal => piccy_core::image::MergeMode::Horizontal,
			MergeMode::Vertical => piccy_core::image::MergeMode::Vertical,
		}
	}
}

pub(crate) fn parse_rgb(rgb_str: &str) -> napi::Result<Rgb<u8>> {
	let parts: Vec<&str> = rgb_str.split(',').collect();
	if parts.len() != 3 {
		return Err(napi::Error::from_reason("RGB string must have three components"));
	}

	let rgb: Result<Vec<u8>, _> = parts
		.iter()
		.map(|&part| {
			part.trim().parse::<u8>()
			    .map_err(|_| napi::Error::from_reason("Invalid RGB component"))
		})
		.collect();

	let [r, g, b] = rgb?
		.try_into()
		.map_err(|_| napi::Error::from_reason("Failed to parse RGB components"))?;

	Ok(Rgb([r, g, b]))
}