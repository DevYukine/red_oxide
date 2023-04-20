// Windows
#[cfg(target_os = "windows")]
pub fn get_flac_executable() -> String {
    return "flac.exe".to_string()
}

#[cfg(target_os = "windows")]
pub fn get_sox_executable() -> String {
    return "sox.exe".to_string()
}

#[cfg(target_os = "windows")]
pub fn get_lame_executable() -> String{
    return "lame.exe".to_string()
}

#[cfg(target_os = "windows")]
pub fn get_imdl_executable_name() -> String {
    return "imdl.exe".to_string()
}

// Linux or Mac
#[cfg(not(target_os = "windows"))]
pub fn get_flac_executable() -> String {
    return "flac".to_string()
}

#[cfg(not(target_os = "windows"))]
pub fn get_sox_executable() -> String {
    return "sox".to_string()
}

#[cfg(not(target_os = "windows"))]
pub fn get_lame_executable() -> String {
    return "lame".to_string()
}

#[cfg(not(target_os = "windows"))]
pub fn get_imdl_executable_name() -> String {
    return "imdl".to_string()
}