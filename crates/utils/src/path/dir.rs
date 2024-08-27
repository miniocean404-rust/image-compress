use std::path::Path;

pub fn dirname(path: &str) -> Option<String> {
    let path_obj = Path::new(path);
    path_obj
        .parent()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
}
