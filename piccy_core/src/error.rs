use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("图片处理失败: {0}")]
    Image(#[from] image::ImageError),
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("Base64解码错误: {0}")]
    Decode(#[from] base64::DecodeError),
    #[error("其他错误: {0}")]
    Other(String),
}
