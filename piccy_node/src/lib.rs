mod error;
mod types;
mod image;

pub(crate) type Result<T> = std::result::Result<T, error::Error>;