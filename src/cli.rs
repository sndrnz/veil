use clap::{ArgAction, Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    /// Output file
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Decrypt input
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub decrypt: bool,

    /// Remove input file
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub remove: bool,

    /// Input format
    #[arg(short = 'f', long, value_name = "FORMAT", value_enum, default_value_t = Format::Bytes)]
    pub input_format: Format,

    /// Output format
    #[arg(short = 'F', long, value_name = "FORMAT", value_enum, default_value_t = Format::Bytes)]
    pub output_format: Format,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum Format {
    /// Output as bytes
    Bytes,
    /// Output as Base64 encoded string
    Base64,
}
