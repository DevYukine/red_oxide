use thiserror::Error;

#[derive(Error, Debug)]
pub enum UpdaterError {
    #[error("No prebuild found for the current platform")]
    NoPrebuildFoundError,
}
