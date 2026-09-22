use cdump::cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("Diretório: {:?}", args.target_dir.display());
}
