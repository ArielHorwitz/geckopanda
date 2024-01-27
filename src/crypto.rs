use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{anyhow, Context, Result};
use rand::{thread_rng, Rng};
use sha2::Digest;

const NONCE_SIZE: usize = 12;

/// Encrypt data
pub fn encrypt(plaintext: &[u8], key: &str) -> Result<Vec<u8>> {
    let cipher = get_cipher(key).context("genereate key")?;
    let nonce_data: [u8; NONCE_SIZE] = thread_rng().gen();
    let mut ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_data), plaintext)
        .map_err(|e| anyhow!("encryption failure: {e}"))?;
    ciphertext.extend_from_slice(&nonce_data);
    Ok(ciphertext)
}

/// Decrypt data
pub fn decrypt(ciphertext: &[u8], key: &str) -> Result<Vec<u8>> {
    let cipher = get_cipher(key)?;
    let split_at = ciphertext.len().saturating_sub(NONCE_SIZE);
    let (ciphertext, nonce_data) = ciphertext.split_at(split_at);
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce_data), ciphertext)
        .map_err(|e| anyhow!("decryption failure: {e}"))?;
    Ok(plaintext)
}

fn get_cipher(key: &str) -> Result<Aes256Gcm> {
    let fixed_key = sha2::Sha256::digest(key.as_bytes());
    let cipher = Aes256Gcm::new_from_slice(&fixed_key).context("aes from slice")?;
    Ok(cipher)
}
