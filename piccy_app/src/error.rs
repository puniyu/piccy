use thiserror::Error;

#[derive(Error, Debug)]
pub  enum Error {
	#[error("{0}")]
	Tauri(#[from] tauri::Error),
	#[error("{0}")]
	PiccyCore(#[from] piccy_core::Error)
}

impl From<Error> for tauri::Error {
	fn from(error: Error) -> Self {
		tauri::Error::Anyhow(anyhow::anyhow!(error))
	}
}