use aes_gcm::aead::consts::U12;
use aes_gcm::aead::{Aead, KeyInit, Nonce, OsRng};
use aes_gcm::aes::Aes256;
use aes_gcm::{AeadCore, AesGcm};
use aes_gcm::{Aes256Gcm, Key};
use base64::engine::general_purpose;
use base64::Engine;
use persistence::Env;
use std::sync::Arc;
// Or `Aes128Gcm`

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CryptoConfig<'a> {
    pub app_key_main: &'a str,
}

impl CryptoConfig<'_> {
    pub fn new(env: &Env) -> CryptoConfig {
        CryptoConfig {
            app_key_main: &env.app_key_main,
        }
    }
}

pub struct Crypto<'a> {
    pub config: CryptoConfig<'a>,
}

impl<'a> Crypto<'a> {
    pub fn new(env: &'a Env) -> Crypto<'a> {
        Crypto {
            config: CryptoConfig::new(env),
        }
    }

    pub fn new_arc(env: &'a Env) -> Arc<dyn Encrypt + Send + Sync + 'a> {
        Arc::new(Crypto::new(env))
    }
}

#[async_trait::async_trait]
#[allow(dead_code)]
pub trait Encrypt {
    async fn encrypt(&self, data: &str) -> anyhow::Result<String>;
    async fn decrypt(&self, data: &str) -> anyhow::Result<String>;
    async fn decrypt_oy(&self, data: &str) -> anyhow::Result<String>;
}

#[async_trait::async_trait]
impl Encrypt for Crypto<'_> {
    async fn encrypt(&self, data: &str) -> anyhow::Result<String> {
        let key = Key::<Aes256Gcm>::from_slice(self.config.app_key_main.as_bytes());
        let cipher = Aes256Gcm::new(key);

        let nonce: Nonce<AesGcm<Aes256, U12>> = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let ciphertext = cipher
            .encrypt(&nonce, data.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption error: {:?}", e))?;

        let nonce = general_purpose::URL_SAFE_NO_PAD.encode(nonce);
        let ciphertext = general_purpose::URL_SAFE_NO_PAD.encode(ciphertext);
        Ok(format!("{}{}", nonce, ciphertext))
    }

    async fn decrypt(&self, data: &str) -> anyhow::Result<String> {
        let (nonce_str, ciphertext_str) = data.split_at(16); // 12 bytes encoded in base64 is 16 characters
        let nonce = general_purpose::URL_SAFE_NO_PAD.decode(nonce_str)?;
        let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce);
        let ciphertext = general_purpose::URL_SAFE_NO_PAD.decode(ciphertext_str)?;
        let key = Key::<Aes256Gcm>::from_slice(self.config.app_key_main.as_bytes());
        let cipher = Aes256Gcm::new(key);

        let plaintext = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?;

        Ok(String::from_utf8(plaintext)?)
    }
    async fn decrypt_oy(&self, data: &str) -> anyhow::Result<String> {
        let parts: Vec<&str> = data.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid encrypted data format"));
        }
        let nonce = general_purpose::URL_SAFE_NO_PAD.decode(parts[0])?;
        let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce);
        let ciphertext = general_purpose::URL_SAFE_NO_PAD.decode(parts[1])?;
        let key = Key::<Aes256Gcm>::from_slice(self.config.app_key_main.as_bytes());
        let cipher = Aes256Gcm::new(key);

        let plaintext = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?;

        Ok(String::from_utf8(plaintext)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_encrypt_decrypt_oy() {
        dotenv::dotenv().ok();
        let env = Box::leak(Box::new(Env::new()));
        let crypto = Crypto::new(env);
        let data = "Hello, world!";
        let encrypted = match crypto.encrypt(data).await {
            Ok(encrypted) => encrypted,
            Err(e) => {
                println!("Error: {}", e);
                panic!("Error: {}", e);
            }
        };

        let decrypted = match crypto.decrypt(&encrypted).await {
            Ok(decrypted) => decrypted,
            Err(e) => {
                println!("Error: {}", e);
                panic!("Error: {}", e);
            }
        };
        println!("decrypted: {}", decrypted);
        println!("encrypted: {}", encrypted);
        assert_eq!(data, decrypted);
    }
}
