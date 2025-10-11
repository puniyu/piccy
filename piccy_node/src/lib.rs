mod error;
mod types;
mod image;
mod gif;

pub(crate) type Result<T> = std::result::Result<T, error::Error>;