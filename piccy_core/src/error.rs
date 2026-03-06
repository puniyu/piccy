use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Image processing failed: {0}")]
    Image(#[from] image::ImageError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Base64 decode error: {0}")]
    Decode(#[from] base64::DecodeError),
    #[error("Other error: {0}")]
    Other(String),
}
