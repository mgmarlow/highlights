use clap::{Parser, ValueEnum};
use std::fs;

mod format;
mod parser;

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    Summary,
    Json,
    Sqlite,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    filename: String,

    #[arg(long, default_value = "summary")]
    format: Format,

    #[arg(long)]
    outfile: Option<String>,
}

fn main() {
    let args = Args::parse();

    let expanded_path = shellexpand::tilde(&args.filename);
    let contents = fs::read_to_string(expanded_path.as_ref()).expect("Failed to read clippings");
    let parsed = parser::parse(&contents);

    match args.format {
        Format::Summary => format::summary(&parsed),
        Format::Json => format::json(&parsed, args.outfile.as_deref()),
        Format::Sqlite => format::sqlite(&parsed, args.outfile.as_deref()),
    }
}
