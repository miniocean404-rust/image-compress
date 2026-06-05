pub fn is_test() -> bool {
    cfg!(test)
}

pub fn is_debug() -> bool {
    cfg!(debug_assertions)
}

pub fn is_release() -> bool {
    !cfg!(debug_assertions)
}

pub fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

pub fn is_linux() -> bool {
    cfg!(target_os = "linux")
}
