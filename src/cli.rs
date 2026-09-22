use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    // Diretório alvo para o dump
    #[arg(default_value = ".")]
    pub target_dir: PathBuf,

    // Varredura recursiva em subdiretórios
    #[arg(short = 'R', long)]
    pub recursive: bool,

    // Limita a profundidade da busca em diretórios
    #[arg(short = 'L', long, value_name = "NUM")]
    pub level: Option<usize>,

    // Ignora arquivos ou pastas que correspondam ao padrão
    #[arg(short = 'I', long = "ignore", value_name = "PATTERN")]
    pub ignore_patterns: Vec<String>,

    // Não respeita arquivos .gitignore automaticamente
    #[arg(long)]
    pub no_ignore: bool,

    // Filtra apenas por extensões específicas (ex: -e rs -e toml)
    #[arg(short = 'e', long = "extension", value_name = "EXT")]
    pub extensions: Vec<String>,

    // Exibe apenas as primeiras N linhas de cada arquivo (comportamento de head)
    #[arg(short = 'H', long, value_name = "NUM", conflicts_with = "tail")]
    pub head: Option<usize>,

    // Exibe apenas as últimas N linhas de cada arquivo (comportamento de tail)
    #[arg(short = 'T', long, value_name = "NUM", conflicts_with = "head")]
    pub tail: Option<usize>,

    // Adiciona números de linha na exibição do conteúdo
    #[arg(long)]
    pub line_numbers: bool,

    // Salva a saída em um arquivo em vez do terminal
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    pub output_file: Option<PathBuf>,
}
