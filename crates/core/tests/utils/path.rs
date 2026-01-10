//! 路径工具模块
//!
//! 本模块提供工作区路径解析功能，用于在测试中定位资源文件。

use std::path::{Path, PathBuf};

use cargo_metadata::MetadataCommand;

/// 获取工作区内文件的绝对路径
///
/// 通过 cargo_metadata 获取工作区根目录，然后拼接相对路径。
/// 这确保了无论从哪个目录运行测试，都能正确找到资源文件。
///
/// # 参数
///
/// * `file_path` - 相对于工作区根目录的文件路径
///
/// # 返回值
///
/// 返回文件的绝对路径
///
/// # 示例
///
/// ```ignore
/// let path = get_workspace_file_path("assets/image/png/test.png");
/// ```
pub fn get_workspace_file_path(file_path: &str) -> PathBuf {
    let metadata = MetadataCommand::new().exec().unwrap();
    let workspace_root = metadata.workspace_root;

    Path::new(&workspace_root).join(file_path)
}
