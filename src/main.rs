use clap::{Parser, ValueEnum};
use std::fs;

mod parser;

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    Summary,
    Json,
    Csv,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    filename: String,

    #[arg(long, default_value = "summary")]
    format: Format,
}

fn main() {
    let args = Args::parse();

    let expanded_path = shellexpand::tilde(&args.filename);
    let contents = fs::read_to_string(expanded_path.as_ref()).expect("Failed to read clippings");
    let parsed = parser::parse(&contents);

    println!("read {} highlights", parsed.len());
}
