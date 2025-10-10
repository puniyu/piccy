mod error;
pub use error::Error;
mod gif;
pub mod image;

pub type Result<T> = std::result::Result<T, Error>;