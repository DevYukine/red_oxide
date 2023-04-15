use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedactedApiError {
    #[error("Redacted API returned 401 Unauthorized with error message: {0}")]
    AuthError(String),

    #[error("Redacted API returned no response body")]
    BodyError,

    #[error("Redacted API returned an error while uploading a torrent: {0}")]
    UploadError(String),
}
