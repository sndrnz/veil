use anyhow::{Context, Result};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

/// Encrypt bytes with a password
///
/// # Arguments
///
/// * `bytes` - The bytes to encrypt
/// * `password` - The password to encrypt with
///
/// # Returns
///
/// * `Result<Vec<u8>>` - The encrypted bytes
pub fn encrypt_bytes(bytes: Vec<u8>, password: String) -> Result<Vec<u8>> {
    let mc = new_magic_crypt!(password, 256);
    let encrypted_file = mc.encrypt_bytes_to_bytes(&bytes);

    Ok(encrypted_file)
}

/// Decrypt bytes with a password
///
/// # Arguments
///
/// * `bytes` - The bytes to decrypt
/// * `password` - The password to decrypt with
///
/// # Returns
///
/// * `Result<Vec<u8>>` - The decrypted bytes
pub fn decrypt_bytes(bytes: Vec<u8>, password: String) -> Result<Vec<u8>> {
    let mc = new_magic_crypt!(password, 256);

    let decrypted_file = mc
        .decrypt_bytes_to_bytes(&bytes)
        .context("Failed to decrypt file")?;

    Ok(decrypted_file)
}
