use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use keyring::Entry;
use rand::RngCore;

const SERVICE_NAME: &str = "FileFlow";
const KEY_NAME: &str = "encryption_key";

/// Manages encryption and decryption of sensitive data
pub struct EncryptionManager;

impl EncryptionManager {
    /// Gets or creates the encryption key from the system keyring
    fn get_or_create_key() -> Result<Vec<u8>, String> {
        let entry = Entry::new(SERVICE_NAME, KEY_NAME)
            .map_err(|e| format!("Failed to access keyring: {}", e))?;

        // Try to retrieve existing key
        match entry.get_password() {
            Ok(key_b64) => {
                // Decode the base64 key
                STANDARD
                    .decode(key_b64)
                    .map_err(|e| format!("Failed to decode key: {}", e))
            }
            Err(_) => {
                // Generate new 256-bit key
                let mut key = vec![0u8; 32];
                OsRng.fill_bytes(&mut key);

                // Store it in the keyring
                let key_b64 = STANDARD.encode(&key);
                entry
                    .set_password(&key_b64)
                    .map_err(|e| format!("Failed to store key in keyring: {}", e))?;

                Ok(key)
            }
        }
    }

    /// Encrypts a password using AES-256-GCM
    pub fn encrypt_password(password: &str) -> Result<String, String> {
        if password.is_empty() {
            return Ok(String::new());
        }

        // Get encryption key
        let key = Self::get_or_create_key()?;

        // Create cipher
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| format!("Failed to create cipher: {}", e))?;

        // Generate random nonce (96 bits for GCM)
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, password.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        // Combine nonce + ciphertext and encode as base64
        let mut combined = nonce_bytes.to_vec();
        combined.extend_from_slice(&ciphertext);

        Ok(STANDARD.encode(combined))
    }

    /// Decrypts a password that was encrypted with encrypt_password
    pub fn decrypt_password(encrypted: &str) -> Result<String, String> {
        if encrypted.is_empty() {
            return Ok(String::new());
        }

        // Decode from base64
        let combined = STANDARD
            .decode(encrypted)
            .map_err(|e| format!("Failed to decode encrypted data: {}", e))?;

        // Extract nonce and ciphertext
        if combined.len() < 12 {
            return Err("Invalid encrypted data: too short".to_string());
        }

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Get decryption key
        let key = Self::get_or_create_key()?;

        // Create cipher
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| format!("Failed to create cipher: {}", e))?;

        // Decrypt
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;

        // Convert to string
        String::from_utf8(plaintext)
            .map_err(|e| format!("Failed to convert decrypted data to string: {}", e))
    }

    /// Checks if a string is encrypted (base64 format with sufficient length)
    pub fn is_encrypted(data: &str) -> bool {
        if data.is_empty() {
            return false;
        }

        // Try to decode as base64
        if let Ok(decoded) = STANDARD.decode(data) {
            // Encrypted data should be at least nonce (12 bytes) + some ciphertext
            decoded.len() >= 12
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let password = "my_secret_password";

        let encrypted = EncryptionManager::encrypt_password(password).unwrap();
        assert_ne!(encrypted, password);
        assert!(EncryptionManager::is_encrypted(&encrypted));

        let decrypted = EncryptionManager::decrypt_password(&encrypted).unwrap();
        assert_eq!(decrypted, password);
    }

    #[test]
    fn test_empty_password() {
        let encrypted = EncryptionManager::encrypt_password("").unwrap();
        assert_eq!(encrypted, "");

        let decrypted = EncryptionManager::decrypt_password("").unwrap();
        assert_eq!(decrypted, "");
    }

    #[test]
    fn test_is_encrypted() {
        assert!(!EncryptionManager::is_encrypted("plain_text"));
        assert!(!EncryptionManager::is_encrypted(""));

        let encrypted = EncryptionManager::encrypt_password("test").unwrap();
        assert!(EncryptionManager::is_encrypted(&encrypted));
    }

    #[test]
    fn test_multiple_encryptions_different() {
        let password = "same_password";

        let encrypted1 = EncryptionManager::encrypt_password(password).unwrap();
        let encrypted2 = EncryptionManager::encrypt_password(password).unwrap();

        // Different nonces should produce different ciphertexts
        assert_ne!(encrypted1, encrypted2);

        // But both should decrypt to the same value
        assert_eq!(
            EncryptionManager::decrypt_password(&encrypted1).unwrap(),
            password
        );
        assert_eq!(
            EncryptionManager::decrypt_password(&encrypted2).unwrap(),
            password
        );
    }
}
