pub const CONFIG_FILE_NAME: &str = "red_oxide.config.json";

pub const PROJECT_NAME: &str = "red_oxide";

pub const CONFIG_PATH: &str = ".config";

pub const XDG_CONFIG_ENV: &str = "XDG_CONFIG_HOME";

pub const HOME_ENV: &str = "HOME";

pub const WINDOWS_APPDATA_ENV: &str = "APPDATA";

#[cfg(target_os = "windows")]
pub const WINDOWS_USERPROFILE_ENV: &str = "USERPROFILE";
#[cfg(target_os = "windows")]
pub const WINDOWS_HOMEDRIVE_ENV: &str = "HOMEDRIVE";
#[cfg(target_os = "windows")]
pub const WINDOWS_HOMEPATH_ENV: &str = "HOMEPATH";
