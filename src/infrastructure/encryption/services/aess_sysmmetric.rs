use super::super::enc_errors::EncryptionError;
use super::super::traits::EncryptionServiceTraits;
use aes_gcm::{
    Aes256Gcm, Key, KeyInit,
    aead::{Aead, Nonce},
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::{Rng, rng};

const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;

pub struct AESSymmetricEncryptionService {
    pub cipher: Aes256Gcm,
}

impl AESSymmetricEncryptionService {
    pub fn new(key: String) -> Result<Self, EncryptionError> {
        let k = Key::<Aes256Gcm>::try_from(key.as_bytes())
            .map_err(|e| return EncryptionError::AesGcmError(e.to_string()))?;
        let cipher = Aes256Gcm::new(&k);
        Ok(Self { cipher })
    }
}

impl EncryptionServiceTraits for AESSymmetricEncryptionService {
    fn encrypt(&self, plaintext: &[u8]) -> Result<String, EncryptionError> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rng().fill(&mut nonce_bytes);

        let nonce = Nonce::<Aes256Gcm>::try_from(nonce_bytes.as_slice()).map_err(|e| {
            return EncryptionError::AesGcmError(e.to_string());
        })?;

        let ciphertext = self.cipher.encrypt(&nonce, plaintext).map_err(|e| {
            return EncryptionError::AesGcmError(e.to_string());
        })?;

        let mut payload = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        payload.extend_from_slice(&nonce_bytes);
        payload.extend_from_slice(&ciphertext);

        let str = STANDARD.encode(payload);

        Ok(str)
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, EncryptionError> {
        let raw_bytes = STANDARD.decode(ciphertext).map_err(|e| {
            return EncryptionError::Base64DecodeError(e.to_string());
        })?;

        if raw_bytes.len() < NONCE_LEN + TAG_LEN {
            return Err(EncryptionError::BadInput(format!("Ciphertext too short")));
        }

        let (nonce_bytes, ciphertext) = raw_bytes.split_at(NONCE_LEN);
        let nonce = Nonce::<Aes256Gcm>::try_from(nonce_bytes).map_err(|e| {
            return EncryptionError::AesGcmError(e.to_string());
        })?;
        let decrypted_bytes = self.cipher.decrypt(&nonce, ciphertext).map_err(|e| {
            return EncryptionError::AesGcmError(e.to_string());
        })?;

        let result = String::from_utf8(decrypted_bytes).map_err(|e| {
            return EncryptionError::Utf8Error(e.to_string());
        })?;

        Ok(result)
    }
}
