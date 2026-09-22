use crate::cli::Args;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

pub fn print_files(files: &[PathBuf], args: &Args) -> io::Result<()> {
    let mut writer: Box<dyn Write> = match &args.output_file {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(BufWriter::new(io::stdout())),
    };

    let mut first = true;

    for file_path in files {
        if first {
            first = false;
        } else {
            writeln!(writer, "\n")?;
        }

        let display_path = get_relative_path(&args.target_dir, file_path);
        let header = format!("{display_path}:");

        let divider = "=".repeat(header.chars().count());

        writeln!(writer, "{divider}")?;
        writeln!(writer, "{header}")?;
        writeln!(writer, "{divider}")?;

        process_and_write_file(file_path, &mut writer, args)?;
    }

    Ok(())
}

fn get_relative_path(target_dir: &Path, file_path: &Path) -> String {
    if let Ok(stripped) = file_path.strip_prefix(target_dir) {
        if stripped.as_os_str().is_empty() {
            file_path.to_string_lossy().into_owned()
        } else {
            stripped.to_string_lossy().into_owned()
        }
    } else {
        file_path.to_string_lossy().into_owned()
    }
}

fn process_and_write_file(
    path: &PathBuf,
    writer: &mut Box<dyn Write>,
    args: &Args,
) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let selected_lines: Box<dyn Iterator<Item = (usize, &String)>> = if let Some(n) = args.head {
        Box::new(lines.iter().enumerate().take(n))
    } else if let Some(n) = args.tail {
        let start = lines.len().saturating_sub(n);
        Box::new(lines.iter().enumerate().skip(start))
    } else {
        Box::new(lines.iter().enumerate())
    };

    for (index, line) in selected_lines {
        if args.line_numbers {
            writeln!(writer, "{:4} | {}", index + 1, line)?;
        } else {
            writeln!(writer, "{line}")?;
        }
    }

    Ok(())
}
