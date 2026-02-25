mod error;
#[doc(inline)]
pub use error::Error;
mod common;
mod image;
#[doc(inline)]
pub use image::*;
mod types;
#[doc(inline)]
pub use types::*;

pub type Result<T> = std::result::Result<T, Error>;