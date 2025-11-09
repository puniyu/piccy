mod error;
pub use error::Error;
mod common;
pub mod gif;
pub mod image;

pub type Result<T> = std::result::Result<T, Error>;
