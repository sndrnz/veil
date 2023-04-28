use std::{path::PathBuf, process::exit};

use anyhow::{Context, Result};

use clap::{ArgAction, Parser};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// output file
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// decrypt instead of encrypt
    #[arg(short, long, action = ArgAction::SetTrue)]
    decrypt: bool,

    /// input file
    #[arg()]
    input: PathBuf,
}

fn get_password() -> Result<String> {
    rpassword::prompt_password("Password: ").context("Failed to get password")
}

fn read_file_as_bytes(file_path: PathBuf) -> Result<Vec<u8>> {
    std::fs::read(file_path).context("Failed to read file")
}

fn write_file_as_bytes(file_path: PathBuf, bytes: Vec<u8>) -> Result<()> {
    std::fs::write(file_path, bytes).context("Failed to write file")
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

fn run(cli: Cli) -> Result<()> {
    let file_path = cli.input;
    let input_bytes = read_file_as_bytes(file_path)?;

    let password = get_password()?;

    let output_bytes = if cli.decrypt {
        decrypt_bytes(input_bytes, password)?
    } else {
        encrypt_bytes(input_bytes, password)?
    };

    let mut output_file = PathBuf::from("out");
    if let Some(output) = cli.output {
        output_file = output;
    }

    write_file_as_bytes(output_file, output_bytes)?;

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
