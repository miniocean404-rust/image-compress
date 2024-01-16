use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use oxipng::{optimize, InFile, Options, OutFile};

fn main() -> Result<(), Box<dyn Error>> {
    // let read_path = "image";
    let read_path = "D:\\soft-dev\\code\\work\\davinci\\davinci-web\\assets\\image";
    let out_path = "dist";

    if !Path::new(out_path).is_dir() {
        fs::create_dir_all("dist")?;
    }

    let options = Options::max_compression(); // 设置压缩级别，范围是 0 到 6

    let mut paths = fs::read_dir(read_path)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| -> Option<PathBuf> {
            let meta = entry.metadata().expect("期待元数据");
            let path = entry.path();

            if !meta.is_file() {
                return None;
            }

            match path.extension() {
                Some(ext) => {
                    if ext == "png" {
                        Some(path)
                    } else {
                        None
                    }
                }
                None => None,
            }
        });

    paths.try_for_each(|input| -> Result<(), Box<dyn Error>> {
        let mut output = PathBuf::from("dist");

        if let Some(filename) = input.file_name() {
            output.push(filename)
        }

        optimize(
            &InFile::Path(input),
            &OutFile::Path {
                path: Some(output),
                preserve_attrs: true,
            },
            &options,
        )?;

        Ok(())
    })?;

    Ok(())
}
