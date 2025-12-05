use openssl::rand::rand_bytes;
use openssl::symm::{Cipher, Crypter, Mode};
use rand::RngCore;
use std::fs;

/// Encrypts the given file using AES-CBC
pub fn encrypt(file_path: &str) {
    let plaintext = fs::read(file_path).expect("failed reading input file");

    // 32 byte random key
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);

    // 16 bytes for IV
    let mut iv = [0u8; 16];
    rand::rng().fill_bytes(&mut iv);

    let cipher = Cipher::aes_256_cbc();
    let mut crypter =
        Crypter::new(cipher, Mode::Encrypt, &key, Some(&iv)).expect("crypter failed to initialize");

    // block size is added as a buffer
    let mut ciphertext = vec![0; plaintext.len() + cipher.block_size()];

    // encrypts all complete blocks
    let mut count = crypter
        .update(&plaintext, &mut ciphertext)
        .expect("update failed");

    // encrypts the last incomplete block if it exists
    count += crypter
        .finalize(&mut ciphertext[count..])
        .expect("finalize failed");

    // trims vector
    ciphertext.truncate(count);

    fs::write(file_path, &ciphertext).expect("failed to write output");
}
