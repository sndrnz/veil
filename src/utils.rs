pub use crate::cli::Cli;
use anyhow::{anyhow, Context, Result};
use std::{
    fs,
    io::{self, Read, Write},
    path::PathBuf,
};

/// Map an IO error to an anyhow error
///
/// # Arguments
/// * `err` - The IO error
///
/// # Returns
/// * `anyhow::Error` - The mapped error
fn map_io_error(err: std::io::Error) -> anyhow::Error {
    match err.kind() {
        std::io::ErrorKind::NotFound => anyhow!("File not found"),
        std::io::ErrorKind::PermissionDenied => anyhow!("Permission denied"),
        _ => anyhow!("Failed to read/write file"),
    }
}

/// Get a password from the user
///
/// # Arguments
/// * `prompt` - The prompt to display to the user
///
/// # Returns
/// * `Result<String>` - The password
pub fn get_password(prompt: &str) -> Result<String> {
    let prompt = format!("{}: ", prompt);
    rpassword::prompt_password(prompt).context("Failed to get password")
}

/// Read stdin as bytes
///
/// # Returns
///
/// * `Result<Vec<u8>>` - The stdin contents
pub fn read_bytes_from_stdin() -> Result<Vec<u8>> {
    let mut input_bytes = Vec::new();
    io::stdin()
        .read_to_end(&mut input_bytes)
        .context("Failed to read from stdin")?;
    Ok(input_bytes)
}

/// Read a file as bytes
///
/// # Arguments
/// * `file_path` - The path to the file
///
/// # Returns
/// * `Result<Vec<u8>>` - The file contents
pub fn read_bytes_from_file(file_path: PathBuf) -> Result<Vec<u8>> {
    fs::read(file_path).map_err(map_io_error)
}

/// Write bytes to stdout
///
/// # Arguments
/// * `bytes` - The bytes to write
///
/// # Returns
/// * `Result<()>` - The result of the operation
pub fn write_bytes_to_stdout(bytes: Vec<u8>) -> Result<()> {
    match io::stdout().write(&bytes) {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!("Failed to write bytes to stdout")),
    }
}

/// Write bytes to a file
///
/// # Arguments
/// * `file_path` - The path to the file
/// * `bytes` - The bytes to write
///
/// # Returns
/// * `Result<()>` - The result of the operation
pub fn write_bytes_to_file(file_path: PathBuf, bytes: Vec<u8>) -> Result<()> {
    fs::write(file_path, bytes).map_err(map_io_error)
}

// pub fn append_extension(path: &mut PathBuf, extension: &str) {
//     let old_extension = path.extension();

//     match old_extension {
//         Some(ext) => {
//             let new_extension = format!("{}.{}", ext.to_str().unwrap(), extension);
//             path.set_extension(new_extension)
//         }
//         None => path.set_extension(extension),
//     };
// }

// pub fn write_stdout_as_string(bytes: Vec<u8>) -> Result<()> {
//     let string = String::from_utf8(bytes.clone()).context("Failed to encode bytes as string")?;
//     print!("{}", string);
//     Ok(())
// }
