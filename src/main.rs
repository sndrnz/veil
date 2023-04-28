use base64_light::*;
use std::{fs, path::PathBuf, process::exit};

use anyhow::{Context, Result};

use clap::{ArgAction, Parser, ValueEnum};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Output file
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Decrypt input
    #[arg(short, long, action = ArgAction::SetTrue)]
    decrypt: bool,

    /// Remove input file
    #[arg(short, long, action = ArgAction::SetTrue)]
    remove: bool,

    /// Input format
    #[arg(short = 'f', long, value_name = "FORMAT", value_enum, default_value_t = Format::Bytes)]
    input_format: Format,

    /// Output format
    #[arg(short = 'F', long, value_name = "FORMAT", value_enum, default_value_t = Format::Bytes)]
    output_format: Format,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
enum Format {
    /// Output as bytes
    Bytes,
    /// Output as Base64 encoded string
    Base64,
}

fn get_password(prompt: &str) -> Result<String> {
    let prompt = format!("{}: ", prompt);
    rpassword::prompt_password(prompt).context("Failed to get password")
}

fn read_file_as_bytes(file_path: PathBuf) -> Result<Vec<u8>> {
    fs::read(file_path).context("Failed to read file")
}

fn read_file_as_base64(file_path: PathBuf) -> Result<Vec<u8>> {
    let content = fs::read_to_string(file_path).context("Failed to read file as string")?;
    Ok(base64_decode(content.as_str()))
}

fn write_file_as_bytes(file_path: PathBuf, bytes: Vec<u8>) -> Result<()> {
    fs::write(file_path, bytes).context("Failed to write file")
}

fn write_file_as_base64(file_path: PathBuf, bytes: Vec<u8>) -> Result<()> {
    let content = base64_encode_bytes(&bytes);
    fs::write(file_path, content).context("Failed to write file")
}

fn encrypt_bytes(bytes: Vec<u8>, password: String) -> Result<Vec<u8>> {
    let mc = new_magic_crypt!(password, 256);
    let encrypted_file = mc.encrypt_bytes_to_bytes(&bytes);

    Ok(encrypted_file)
}

fn decrypt_bytes(bytes: Vec<u8>, password: String) -> Result<Vec<u8>> {
    let mc = new_magic_crypt!(password, 256);

    let decrypted_file = mc
        .decrypt_bytes_to_bytes(&bytes)
        .context("Failed to decrypt file")?;

    Ok(decrypted_file)
}

fn append_extension(path: &mut PathBuf, extension: &str) {
    let old_extension = path.extension();

    match old_extension {
        Some(ext) => {
            let new_extension = format!("{}.{}", ext.to_str().unwrap(), extension);
            path.set_extension(new_extension)
        }
        None => path.set_extension(extension),
    };
}

fn run(cli: Cli) -> Result<()> {
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
            eprintln!("{}", err.to_string());
            exit(1);
        }
    }
}
