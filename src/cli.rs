use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file, defaults to stdin
    #[arg(short, long = "input", value_name = "FILE")]
    pub input_file: Option<PathBuf>,

    /// Output file, defaults to stdout
    #[arg(short, long = "output", value_name = "FILE")]
    pub output_file: Option<PathBuf>,

    /// Decrypt input
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub decrypt: bool,

    /// Remove input file
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub remove: bool,
}
