use std::{ffi::OsStr, path::Path};

pub fn change_file_ext(path: &str, ext: &str) -> String {
    let path = Path::new(path);
    // let filename_no_ext = path.file_stem().unwrap_or(OsStr::new(""));
    path.with_extension(ext).to_string_lossy().to_string()
}

pub fn filename_no_ext(path: &str) -> String {
    let path = Path::new(path);
    path.file_stem()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy()
        .to_string()
}
