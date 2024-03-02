use crate::built_info;
use crate::updater::error::UpdaterError::NoPrebuildFoundError;
use std::fmt::Display;

#[derive(Debug)]
pub struct ReleaseVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Display for ReleaseVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ReleaseVersionCompareResult {
    OutdatedMajor,
    OutdatedMinor,
    OutdatedPatch,
    EqualOrNewer,
}

pub fn get_current_release_version() -> ReleaseVersion {
    ReleaseVersion {
        major: built_info::PKG_VERSION_MAJOR.parse().unwrap(),
        minor: built_info::PKG_VERSION_MINOR.parse().unwrap(),
        patch: built_info::PKG_VERSION_PATCH.parse().unwrap(),
    }
}

pub fn get_filename_for_current_target_triple() -> anyhow::Result<String> {
    return match built_info::TARGET {
        "x86_64-unknown-freebsd" => Ok("red_oxide-FreeBSD-x86_64".to_string()),
        "x86_64-unknown-linux-gnu" => Ok("red_oxide-Linux-x86_64-gnu".to_string()),
        "x86_64-unknown-linux-musl" => Ok("red_oxide-Linux-x86_64-musl".to_string()),
        "i686-unknown-linux-gnu" => Ok("red_oxide-Linux-i686-gnu".to_string()),
        "aarch64-unknown-linux-gnu" => Ok("red_oxide-Linux-aarch64".to_string()),
        "armv7-unknown-linux-gnueabihf" => Ok("red_oxide-Linux-armv7".to_string()),
        "x86_64-pc-windows-msvc" => Ok("red_oxide-Windows-x86_64.exe".to_string()),
        "i686-pc-windows-msvc" => Ok("red_oxide-Windows-i686.exe".to_string()),
        "x86_64-apple-darwin" => Ok("red_oxide-Darwin-x86_64".to_string()),
        "aarch64-apple-darwin" => Ok("red_oxide-Darwin-aarch64".to_string()),
        _ => Err(NoPrebuildFoundError.into()),
    };
}

pub fn compare_latest_release_to_current_version(
    latest: &ReleaseVersion,
    current: &ReleaseVersion,
) -> ReleaseVersionCompareResult {
    if latest.major > current.major {
        return ReleaseVersionCompareResult::OutdatedMajor;
    }

    if latest.minor > current.minor {
        return ReleaseVersionCompareResult::OutdatedMinor;
    }

    if latest.patch > current.patch {
        return ReleaseVersionCompareResult::OutdatedPatch;
    }

    return ReleaseVersionCompareResult::EqualOrNewer;
}
