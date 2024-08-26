use std::path::{Path, PathBuf};

use cargo_metadata::MetadataCommand;

pub fn get_workspace_file_path(file_path: &str) -> PathBuf {
    let metadata = MetadataCommand::new().exec().unwrap();
    let workspace_root = metadata.workspace_root;

    Path::new(&workspace_root).join(file_path)
}
