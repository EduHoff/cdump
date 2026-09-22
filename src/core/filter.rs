use crate::cli::Args;
use globset::{Glob, GlobSetBuilder};
use std::path::PathBuf;

pub fn filter_files(
    files: Vec<PathBuf>,
    args: &Args,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut ignore_builder = GlobSetBuilder::new();
    for pattern in &args.ignore_patterns {
        ignore_builder.add(Glob::new(pattern)?);
    }
    let ignore_set = ignore_builder.build()?;

    let mut filtered_files = Vec::new();

    for path in files {
        if let Some(path_str) = path.to_str()
            && ignore_set.is_match(path_str)
        {
            continue;
        }

        if let Some(file_name) = path.file_name().and_then(|n| n.to_str())
            && ignore_set.is_match(file_name)
        {
            continue;
        }

        if !args.extensions.is_empty() {
            let matches_extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext_str| args.extensions.iter().any(|allowed| allowed == ext_str));

            if !matches_extension {
                continue;
            }
        }

        filtered_files.push(path);
    }

    Ok(filtered_files)
}
