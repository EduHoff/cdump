use crate::cli::Args;
use ignore::WalkBuilder;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn is_valid_text_file(path: &Path) -> bool {
    let Ok(bytes) = fs::read(path) else {
        return false;
    };

    String::from_utf8(bytes).is_ok()
}

pub fn scan_directory(args: &Args) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    let mut walker_builder = WalkBuilder::new(&args.target_dir);

    if let Some(lvl) = args.level {
        walker_builder.max_depth(Some(lvl));
    } else if !args.recursive {
        walker_builder.max_depth(Some(1));
    }

    let walker = walker_builder.build();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() && is_valid_text_file(path) {
                    files.push(path.to_path_buf());
                }
            }
            Err(err) => eprintln!("Warning while scanning entry: {err}"),
        }
    }

    Ok(files)
}
