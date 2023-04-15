use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImdlError {
    #[error("Hash error")]
    HashError,
}
