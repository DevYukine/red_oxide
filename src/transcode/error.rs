use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranscodeError {
    #[error("FLAC file \"{0}\" has a sample rate {1}, which is not 88.2 , 176.4 or 96kHz but needs resampling, this is unsupported")]
    UnknownSampleRateError(PathBuf, u32),

    #[error("FLAC file \"{0}\" has more than 2 channels, unsupported")]
    TranscodeDownmixError(PathBuf),

    #[error("Output directory \"{0}\" already exists, aborting")]
    OutputDirectoryExist(PathBuf),

    #[error("Some FLAC was incorrectly marked as 24bit.")]
    Invalid24BitFlac,
}
