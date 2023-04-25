use std::path::PathBuf;

use clap::{Parser, Subcommand};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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

fn get_password() -> String {
    let password = rpassword::prompt_password("Password: ").unwrap();
    return password;
}

fn encrypt(file_name: PathBuf, password: String) {
    // read file binary
    let file = std::fs::read(file_name.clone()).expect("Unable to read file");

    // ask for password
    let mc = new_magic_crypt!(password, 256);

    let encrypted_file = mc.encrypt_bytes_to_bytes(&file);

    let new_file_name = format!("{}.enc", file_name.to_str().unwrap());

    // write encrypted file
    std::fs::write(new_file_name, encrypted_file).expect("Unable to write file");
}

fn decrypt(file: PathBuf, password: String) {
    // read file binary
    let file = std::fs::read(file).expect("Unable to read file");

    // ask for password
    let mc = new_magic_crypt!(password, 256);

    if let Ok(decrypted_file) = mc.decrypt_bytes_to_bytes(&file) {
        std::fs::write("test.txt.dec", decrypted_file).expect("Unable to write file");
    } else {
        eprintln!("Invalid password");
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lock { file } => {
            let password = get_password();
            encrypt(file, password);
        }
        Commands::Unlock { file } => {
            let password = get_password();
            decrypt(file, password);
        }
    }
}
