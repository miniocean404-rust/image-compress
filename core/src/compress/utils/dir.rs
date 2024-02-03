use std::error::Error;

use globset::{Glob, GlobSetBuilder};
use walkdir::WalkDir;

pub fn glob_dir(pattern: &str, dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut builder = GlobSetBuilder::new();
    let glob = Glob::new(pattern)?;
    builder.add(glob);

    let glob_set = builder.build()?;

    let mut result = vec![];

    let walk_dir = WalkDir::new(dir).max_depth(5);

    for entry in walk_dir.into_iter().filter_map(Result::ok) {
        if glob_set.is_match(entry.path()) {
            result.push(entry.path().display().to_string());
        }
    }

    Ok(result)
}
