use crate::cli::{Cli, Format};
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
    let file_path = cli.input.clone();

    let input_bytes = match cli.input_format {
        Format::Bytes => read_file_as_bytes(file_path)?,
        Format::Base64 => read_file_as_base64(file_path)?,
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

    let output_file = match cli.output {
        Some(output_file) => output_file,
        None => {
            let mut path = cli.input.clone();
            if cli.decrypt {
                append_extension(&mut path, "dec");
            } else {
                append_extension(&mut path, "enc");
            }
            path
        }
    };

    match cli.output_format {
        Format::Bytes => write_file_as_bytes(output_file, output_bytes)?,
        Format::Base64 => write_file_as_base64(output_file, output_bytes)?,
    }

    if cli.remove {
        fs::remove_file(cli.input).context("Failed to remove file")?;
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
