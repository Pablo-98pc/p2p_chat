// src/crypto.rs

use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
}; // Usaremos anyhow para simplificar errores

use base64::{engine::general_purpose, Engine as _};

// 1. Función para generar una clave aleatoria (32 bytes)
// Devuelve la clave en formato String (Base64) para que el usuario pueda copiarla y pasársela a su amigo.
pub fn generate_key() -> String {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    general_purpose::STANDARD.encode(key)
}

// 2. Función de Encriptación
// Recibe: El mensaje (texto) y la clave (texto Base64).
// Devuelve: El mensaje encriptado (texto Base64) o un Error.
pub fn encrypt(plain_text: &str, key_base64: &str) -> Result<String> {
    // Decodifica a string la key de base64
    let key_bytes = general_purpose::STANDARD.decode(key_base64)?;

    // Convierto la key en un tipo Key para generar el cipher
    let key = Key::from_slice(&key_bytes);

    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(key);

    let cyphertext = cipher
        .encrypt(&nonce, plain_text.as_bytes())
        .map_err(|_| anyhow::anyhow!("Encryption failure"))?;

    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&cyphertext);

    Ok(general_purpose::STANDARD.encode(&combined))
}

// 3. Función de Desencriptación
// Recibe: El mensaje encriptado (Base64) y la clave (Base64).
// Devuelve: El mensaje original o Error.
pub fn decrypt(encrypted_data_base64: &str, key_base64: &str) -> Result<String> {
    let encrypted_data = general_purpose::STANDARD.decode(encrypted_data_base64)?;
    let key_bytes = general_purpose::STANDARD.decode(key_base64)?;
    let key = Key::from_slice(&key_bytes);

    if encrypted_data.len() < 12 {
        return Err(anyhow::anyhow!("Data is too short to contain a nonce"));
    }

    let nonce_slice = &encrypted_data[0..12];
    let ciphertext_slice = &encrypted_data[12..];

    let nonce = Nonce::from_slice(nonce_slice);

    let cipher = ChaCha20Poly1305::new(key);

    let decrypted_bytes = cipher
        .decrypt(nonce, ciphertext_slice)
        .map_err(|_| anyhow::anyhow!("Decryption failed"))?;

    let decrypted_string = String::from_utf8(decrypted_bytes)?;

    Ok(decrypted_string)
}
