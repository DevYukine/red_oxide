use std::fmt;

use derivative::Derivative;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, EnumIter, Derivative, Clone, Copy)]
#[derivative(Hash)]
pub enum ReleaseType {
    Flac24,
    Flac,
    Mp3320,
    Mp3V0,
}

impl fmt::Display for ReleaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReleaseType::Flac24 => write!(f, "24Bit FLAC"),
            ReleaseType::Flac => write!(f, "FLAC"),
            ReleaseType::Mp3320 => write!(f, "MP3 320"),
            ReleaseType::Mp3V0 => write!(f, "MP3 V0"),
        }
    }
}
