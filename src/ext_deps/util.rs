// Windows
#[cfg(target_os = "windows")]
pub fn get_flac_executable() -> String {
    "flac.exe".to_string()
}

#[cfg(target_os = "windows")]
pub fn get_sox_executable() -> String {
    "sox.exe".to_string()
}

#[cfg(target_os = "windows")]
pub fn get_lame_executable() -> String {
    "lame.exe".to_string()
}

#[cfg(target_os = "windows")]
pub fn get_imdl_executable_name() -> String {
    "imdl.exe".to_string()
}

// Linux or Mac
#[cfg(not(target_os = "windows"))]
pub fn get_flac_executable() -> String {
    "flac".to_string()
}

#[cfg(not(target_os = "windows"))]
pub fn get_sox_executable() -> String {
    "sox".to_string()
}

#[cfg(not(target_os = "windows"))]
pub fn get_lame_executable() -> String {
    "lame".to_string()
}

#[cfg(not(target_os = "windows"))]
pub fn get_imdl_executable_name() -> String {
    "imdl".to_string()
}
