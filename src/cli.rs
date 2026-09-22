use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(default_value = ".")]
    pub target_dir: PathBuf,

    #[arg(short = 'R', long)]
    pub recursive: bool,

    #[arg(short = 'L', long, value_name = "NUM")]
    pub level: Option<usize>,

    #[arg(short = 'I', long = "ignore", value_name = "PATTERN")]
    pub ignore_patterns: Vec<String>,

    #[arg(long)]
    pub no_ignore: bool,

    #[arg(short = 'e', long = "extension", value_name = "EXT")]
    pub extensions: Vec<String>,

    #[arg(short = 'H', long, value_name = "NUM", conflicts_with = "tail")]
    pub head: Option<usize>,

    #[arg(short = 'T', long, value_name = "NUM", conflicts_with = "head")]
    pub tail: Option<usize>,

    #[arg(long)]
    pub line_numbers: bool,

    #[arg(short = 'o', long = "output", value_name = "FILE")]
    pub output_file: Option<PathBuf>,
}
