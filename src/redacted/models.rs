use std::fmt;

use clap::ValueEnum;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Debug, PartialEq, Eq, EnumIter, Derivative, Clone, Copy, Serialize, Deserialize, ValueEnum,
)]
#[derivative(Hash)]
pub enum ReleaseType {
    Flac24,
    Flac,
    Mp3320,
    Mp3V0,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Category {
    Music = 0,
    Applications = 1,
    EBooks = 2,
    Audiobooks = 3,
    ELearningVideos = 4,
    Comedy = 5,
    Comics = 6,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Release {
    Album = 1,
    Soundtrack = 3,
    EP = 5,
    Anthology = 6,
    Compilation = 7,
    Single = 9,
    LiveAlbum = 11,
    Remix = 13,
    Bootleg = 14,
    Interview = 15,
    Mixtape = 16,
    Demo = 17,
    ConcertRecording = 18,
    DJMix = 19,
    Unknown = 21,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Bitrate {
    K192 = 0,
    APS = 1,
    V2 = 2,
    V1 = 3,
    K256 = 4,
    APX = 5,
    V0 = 6,
    K320 = 7,
    Lossless = 8,
    Lossless24Bit = 9,
    Other = 10,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Format {
    Mp3 = 0,
    Flac = 1,
    Aac = 2,
    Ac3 = 3,
    Dts = 4,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Media {
    CD = 0,
    DVD = 1,
    Vinyl = 2,
    Soundboard = 3,
    SACD = 4,
    DAT = 5,
    Cassette = 6,
    WEB = 7,
    BluRay = 8,
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "Music" => Category::Music,
            "Applications" => Category::Applications,
            "E-Books" => Category::EBooks,
            "Audiobooks" => Category::Audiobooks,
            "E-Learning Videos" => Category::ELearningVideos,
            "Comedy" => Category::Comedy,
            "Comics" => Category::Comics,
            _ => panic!("Unknown category: {}, please report this to Github", value),
        }
    }
}

impl From<&str> for Release {
    fn from(value: &str) -> Self {
        match value {
            "Album" => Release::Album,
            "Soundtrack" => Release::Soundtrack,
            "EP" => Release::EP,
            "Anthology" => Release::Anthology,
            "Compilation" => Release::Compilation,
            "Single" => Release::Single,
            "Live Album" => Release::LiveAlbum,
            "Remix" => Release::Remix,
            "Bootleg" => Release::Bootleg,
            "Interview" => Release::Interview,
            "Mixtape" => Release::Mixtape,
            "Demo" => Release::Demo,
            "Concert Recording" => Release::ConcertRecording,
            "DJ Mix" => Release::DJMix,
            _ => Release::Unknown,
        }
    }
}

impl From<&str> for Bitrate {
    fn from(value: &str) -> Self {
        match value {
            "192" => Bitrate::K192,
            "APS (VBR)" => Bitrate::APS,
            "V2 (VBR)" => Bitrate::V2,
            "V1 (VBR)" => Bitrate::V1,
            "256" => Bitrate::K256,
            "APX (VBR)" => Bitrate::APX,
            "V0 (VBR)" => Bitrate::V0,
            "320" => Bitrate::K320,
            "Lossless" => Bitrate::Lossless,
            "24bit Lossless" => Bitrate::Lossless24Bit,
            _ => Bitrate::Other,
        }
    }
}

impl From<&str> for Format {
    fn from(value: &str) -> Self {
        match value {
            "MP3" => Format::Mp3,
            "FLAC" => Format::Flac,
            "AAC" => Format::Aac,
            "AC3" => Format::Ac3,
            "DTS" => Format::Dts,
            _ => panic!("Unknown format: {}, please report this to Github", value),
        }
    }
}

impl From<&str> for Media {
    fn from(value: &str) -> Self {
        match value {
            "CD" => Media::CD,
            "DVD" => Media::DVD,
            "Vinyl" => Media::Vinyl,
            "Soundboard" => Media::Soundboard,
            "SACD" => Media::SACD,
            "DAT" => Media::DAT,
            "Cassette" => Media::Cassette,
            "WEB" => Media::WEB,
            "Blu-Ray" => Media::BluRay,
            _ => panic!("Unknown media: {}, please report this to Github", value),
        }
    }
}

impl Category {
    pub fn as_int(&self) -> u8 {
        *self as u8
    }
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
