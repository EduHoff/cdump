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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn setup_temp_dir(test_name: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("cdump_test_{test_name}_{timestamp}"));
        fs::create_dir_all(&dir).expect("failed to create temp test directory");
        dir
    }

    #[test]
    fn test_scan_text_and_binary_files() {
        let dir = setup_temp_dir("scan_text_binary");

        let text_file_path = dir.join("hello.txt");
        let mut text_file = File::create(&text_file_path).expect("failed to create test text file");
        text_file
            .write_all(b"Hello, world!")
            .expect("failed to write to test text file");

        let binary_file_path = dir.join("binary.bin");
        let mut binary_file =
            File::create(&binary_file_path).expect("failed to create test binary file");
        binary_file
            .write_all(&[0xFF, 0xFE, 0x00, 0x01])
            .expect("failed to write to test binary file");

        let args = Args {
            target_dir: dir.clone(),
            recursive: false,
            level: None,
            ignore_patterns: vec![],
            no_ignore: false,
            extensions: vec![],
            head: None,
            tail: None,
            line_numbers: false,
            output_file: None,
        };

        let result = scan_directory(&args).expect("failed to scan temp directory");

        let _ = fs::remove_dir_all(&dir);

        assert!(result.iter().any(|p| p.ends_with("hello.txt")));
        assert!(!result.iter().any(|p| p.ends_with("binary.bin")));
    }

    #[test]
    fn test_scan_recursion_depth() {
        let dir = setup_temp_dir("scan_recursion");

        let sub_dir = dir.join("subdir");
        fs::create_dir_all(&sub_dir).expect("failed to create subdir");

        let root_file = dir.join("root.txt");
        File::create(&root_file).expect("failed to create root file");

        let nested_file = sub_dir.join("nested.txt");
        File::create(&nested_file).expect("failed to create nested file");

        let args_non_recursive = Args {
            target_dir: dir.clone(),
            recursive: false,
            level: None,
            ignore_patterns: vec![],
            no_ignore: false,
            extensions: vec![],
            head: None,
            tail: None,
            line_numbers: false,
            output_file: None,
        };

        let result_non_rec =
            scan_directory(&args_non_recursive).expect("failed scan non-recursive");

        let args_recursive = Args {
            target_dir: dir.clone(),
            recursive: true,
            level: None,
            ignore_patterns: vec![],
            no_ignore: false,
            extensions: vec![],
            head: None,
            tail: None,
            line_numbers: false,
            output_file: None,
        };

        let result_rec = scan_directory(&args_recursive).expect("failed scan recursive");

        let _ = fs::remove_dir_all(&dir);

        assert!(result_non_rec.iter().any(|p| p.ends_with("root.txt")));
        assert!(!result_non_rec.iter().any(|p| p.ends_with("nested.txt")));

        assert!(result_rec.iter().any(|p| p.ends_with("root.txt")));
        assert!(result_rec.iter().any(|p| p.ends_with("nested.txt")));
    }
}
