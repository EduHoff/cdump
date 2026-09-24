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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Args;
    use std::path::PathBuf;

    #[test]
    fn test_extension_filter() {
        let args = Args {
            target_dir: PathBuf::from("."),
            recursive: false,
            level: None,
            ignore_patterns: vec![],
            no_ignore: false,
            extensions: vec![String::from("rs")],
            head: None,
            tail: None,
            line_numbers: false,
            output_file: None,
        };

        let files = vec![
            PathBuf::from("main.rs"),
            PathBuf::from("script.py"),
            PathBuf::from("lib.rs"),
        ];

        let result = filter_files(files, &args)
            .expect("failed to filter files by extension during unit test execution");

        assert_eq!(result.len(), 2);
        assert!(result.contains(&PathBuf::from("main.rs")));
        assert!(result.contains(&PathBuf::from("lib.rs")));
    }

    #[test]
    fn test_ignore_patterns_filter() {
        let args = Args {
            target_dir: PathBuf::from("."),
            recursive: false,
            level: None,
            ignore_patterns: vec![String::from("*.log"), String::from("temp_*")],
            no_ignore: false,
            extensions: vec![],
            head: None,
            tail: None,
            line_numbers: false,
            output_file: None,
        };

        let files = vec![
            PathBuf::from("main.rs"),
            PathBuf::from("app.log"),
            PathBuf::from("temp_data.txt"),
            PathBuf::from("README.md"),
        ];

        let result = filter_files(files, &args)
            .expect("failed to filter files by ignore patterns during unit test execution");

        assert_eq!(result.len(), 2);
        assert!(result.contains(&PathBuf::from("main.rs")));
        assert!(result.contains(&PathBuf::from("README.md")));
        assert!(!result.contains(&PathBuf::from("app.log")));
        assert!(!result.contains(&PathBuf::from("temp_data.txt")));
    }

    #[test]
    fn test_multiple_extensions_filter() {
        let args = Args {
            target_dir: PathBuf::from("."),
            recursive: false,
            level: None,
            ignore_patterns: vec![],
            no_ignore: false,
            extensions: vec![String::from("rs"), String::from("toml")],
            head: None,
            tail: None,
            line_numbers: false,
            output_file: None,
        };

        let files = vec![
            PathBuf::from("main.rs"),
            PathBuf::from("Cargo.toml"),
            PathBuf::from("script.py"),
        ];

        let result = filter_files(files, &args)
            .expect("failed to filter files by multiple extensions during unit test execution");

        assert_eq!(result.len(), 2);
        assert!(result.contains(&PathBuf::from("main.rs")));
        assert!(result.contains(&PathBuf::from("Cargo.toml")));
        assert!(!result.contains(&PathBuf::from("script.py")));
    }

    #[test]
    fn test_empty_files_input() {
        let args = Args {
            target_dir: PathBuf::from("."),
            recursive: false,
            level: None,
            ignore_patterns: vec![],
            no_ignore: false,
            extensions: vec![String::from("rs")],
            head: None,
            tail: None,
            line_numbers: false,
            output_file: None,
        };

        let files = vec![];

        let result = filter_files(files, &args)
            .expect("failed to handle empty files list during unit test execution");

        assert!(result.is_empty());
    }
}
