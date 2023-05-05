pub use crate::cli::{Cli, Format};
use anyhow::{anyhow, Context, Result};
use base64_light::*;
use std::{fs, path::PathBuf};

/// Map an IO error to an anyhow error
///
/// # Arguments
///
/// * `err` - The IO error
///
/// # Returns
///
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
///
/// * `prompt` - The prompt to display to the user
///
/// # Returns
///
/// * `Result<String>` - The password
pub fn get_password(prompt: &str) -> Result<String> {
    let prompt = format!("{}: ", prompt);
    rpassword::prompt_password(prompt).context("Failed to get password")
}

/// Read a file as bytes
///
/// # Arguments
///
/// * `file_path` - The path to the file
///
/// # Returns
///
/// * `Result<Vec<u8>>` - The file contents
pub fn read_file_as_bytes(file_path: PathBuf) -> Result<Vec<u8>> {
    fs::read(file_path).map_err(map_io_error)
}

/// Read a file as base64
///
/// # Arguments
///
/// * `file_path` - The path to the file
///
/// # Returns
///
/// * `Result<Vec<u8>>` - The file contents
pub fn read_file_as_base64(file_path: PathBuf) -> Result<Vec<u8>> {
    let content = fs::read_to_string(file_path).map_err(map_io_error)?;
    Ok(base64_decode(content.as_str()))
}

/// Write bytes to a file
///
/// # Arguments
///
/// * `file_path` - The path to the file
/// * `bytes` - The bytes to write
///
/// # Returns
///
/// * `Result<()>` - The result of the operation
pub fn write_file_as_bytes(file_path: PathBuf, bytes: Vec<u8>) -> Result<()> {
    fs::write(file_path, bytes).map_err(map_io_error)
}

/// Write bytes to a file as base64
///
/// # Arguments
///
/// * `file_path` - The path to the file
/// * `bytes` - The bytes to write
///
/// # Returns
///
/// * `Result<()>` - The result of the operation
pub fn write_file_as_base64(file_path: PathBuf, bytes: Vec<u8>) -> Result<()> {
    let content = base64_encode_bytes(&bytes);
    fs::write(file_path, content).map_err(map_io_error)
}

/// Append an extension to a path
///
/// # Arguments
///
/// * `path` - The path to append to
/// * `extension` - The extension to append
pub fn append_extension(path: &mut PathBuf, extension: &str) {
    let old_extension = path.extension();

    match old_extension {
        Some(ext) => {
            let new_extension = format!("{}.{}", ext.to_str().unwrap(), extension);
            path.set_extension(new_extension)
        }
        None => path.set_extension(extension),
    };
}
