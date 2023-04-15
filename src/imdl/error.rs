use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashError {
    #[error("Hash error")]
    HashError(),
}
