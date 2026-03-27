use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Image processing failed")]
    Image {
        #[from]
        #[source]
        source: image::ImageError,
    },
    #[error("IO error")]
    Io {
        #[from]
        #[source]
        source: std::io::Error,
    },
    #[error("Base64 decode error")]
    Decode {
        #[from]
        #[source]
        source: base64::DecodeError,
    },
    #[error("Other error: {0}")]
    Other(String),
}
