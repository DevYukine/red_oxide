pub(crate) fn get_executable_name() -> String {
    if cfg!(target_os = "windows") {
        "imdl.exe".to_string()
    } else {
        "imdl".to_string()
    }
}
