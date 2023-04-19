pub fn get_flac_executable() -> String {
    if cfg!(target_os = "windows") {
        "flac.exe".to_string()
    } else {
        "flac".to_string()
    }
}

pub fn get_sox_executable() -> String {
    if cfg!(target_os = "windows") {
        "sox.exe".to_string()
    } else {
        "sox".to_string()
    }
}

pub fn get_lame_executable() -> String {
    if cfg!(target_os = "windows") {
        "lame.exe".to_string()
    } else {
        "lame".to_string()
    }
}

pub fn get_imdl_executable_name() -> String {
    if cfg!(target_os = "windows") {
        "imdl.exe".to_string()
    } else {
        "imdl".to_string()
    }
}
