use crate::cli::Cli;
use crate::crypt::*;
use crate::utils::*;
use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::process::exit;

mod cli;
mod crypt;
mod utils;

/// Run the program
///
/// # Arguments
///
/// * `cli` - The CLI arguments
///
/// # Returns
///
/// * `Result<()>` - The result of the operation
pub fn run(cli: Cli) -> Result<()> {
    let input_bytes = if let Some(input_file) = cli.input_file.clone() {
        read_bytes_from_file(input_file)?
    } else {
        read_bytes_from_stdin()?
    };

    let prompt_text = if cli.decrypt {
        "Enter the password"
    } else {
        "Set a password"
    };
    let password = get_password(prompt_text)?;

    let output_bytes = if cli.decrypt {
        decrypt_bytes(input_bytes, password)?
    } else {
        encrypt_bytes(input_bytes, password)?
    };

    // output the bytes to a file or stdout
    if let Some(output_file) = cli.output_file {
        write_bytes_to_file(output_file, output_bytes)?;
    } else {
        write_bytes_to_stdout(output_bytes)?;
    }

    if let (Some(input_file), true) = (cli.input_file, cli.remove) {
        fs::remove_file(input_file).context("Failed to remove file")?;
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();
    match run(cli) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {}", err.to_string());
            exit(1);
        }
    }
}
