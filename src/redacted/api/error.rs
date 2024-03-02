use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedactedApiError {
    #[error("Redacted API returned {0} with error message: {1}")]
    NoSuccessStatusCodeError(StatusCode, String),

    #[error("Redacted API returned no response body")]
    BodyError,

    #[error("Redacted API returned an error while uploading a torrent: {0}")]
    UploadError(String),

    #[error("Redacted API returned an error while downloading a torrent: {0}")]
    DownloadError(String),
}
