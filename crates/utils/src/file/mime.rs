use infer::get_from_path;

pub fn get_mime_for_path(file_path: &str) -> anyhow::Result<&str> {
    let infer_type_option = get_from_path(file_path)?;

    match infer_type_option {
        Some(infer_type) => Ok(infer_type.mime_type()),
        None => Ok(""),
    }
}

pub fn get_mime_for_memory(buffer: &[u8]) -> &str {
    let infer_type_option = infer::get(buffer);
    match infer_type_option {
        Some(infer_type) => infer_type.mime_type(),
        None => "",
    }
}
