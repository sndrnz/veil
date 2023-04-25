use std::{path::PathBuf, process::exit};

use clap::{Parser, Subcommand};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// output file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// encrypt file
    Lock {
        /// file to encrypt
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// decrypt file
    Unlock {
        /// file to decrypt
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

enum VeilError {
    EncryptionError,
    DecryptionError,
}

impl VeilError {
    fn handle(self) {
        match self {
            VeilError::EncryptionError => {
                eprintln!("Failed to encrypt file");
                exit(1);
            }
            VeilError::DecryptionError => {
                eprintln!("Failed to decrypt file");
                exit(1);
            }
        }
    }
}

fn get_password() -> String {
    let password = rpassword::prompt_password("Password: ").unwrap();
    return password;
}

fn encrypt(file_path: PathBuf, password: String) -> Result<Vec<u8>, VeilError> {
    let file = std::fs::read(file_path).map_err(|_| VeilError::EncryptionError)?;

    let mc = new_magic_crypt!(password, 256);
    let encrypted_file = mc.encrypt_bytes_to_bytes(&file);

    Ok(encrypted_file)
}

fn decrypt(file_path: PathBuf, password: String) -> Result<Vec<u8>, VeilError> {
    let file = std::fs::read(file_path).map_err(|_| VeilError::DecryptionError)?;

    let mc = new_magic_crypt!(password, 256);

    let decrypted_file = mc
        .decrypt_bytes_to_bytes(&file)
        .map_err(|_| VeilError::DecryptionError)?;

    Ok(decrypted_file)
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Lock { file } => {
            let password = get_password();
            encrypt(file, password)
        }
        Commands::Unlock { file } => {
            let password = get_password();
            decrypt(file, password)
        }
    };

    let mut output_file = PathBuf::from("out");

    if let Some(output) = cli.output {
        output_file = output;
    }

    match result {
        Ok(content) => {
            std::fs::write(output_file, content).unwrap_or_else(|_| {
                eprintln!("Unable to write to file");
                exit(1);
            });
        }
        Err(e) => e.handle(),
    }
}
