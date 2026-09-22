use cdump::cli::Args;
use cdump::core::{filter, printer, scanner};
use clap::Parser;
use std::process;

fn main() {
    let args = Args::parse();

    let raw_files = match scanner::scan_directory(&args) {
        Ok(files) => files,
        Err(err) => {
            eprintln!("Error scanning directory: {err}");
            process::exit(1);
        }
    };

    let filtered_files = match filter::filter_files(raw_files, &args) {
        Ok(files) => files,
        Err(err) => {
            eprintln!("Error applying filters: {err}");
            process::exit(1);
        }
    };

    if filtered_files.is_empty() {
        println!("No files found matching the criteria.");
        return;
    }

    if let Err(err) = printer::print_files(&filtered_files, &args) {
        eprintln!("Error processing files: {err}");
        process::exit(1);
    }
}
