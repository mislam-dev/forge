use super::enc_errors::EncryptionError;

pub trait EncryptionServiceTraits {
    fn encrypt(&self, plaintext: &[u8]) -> Result<String, EncryptionError>;
    fn decrypt(&self, ciphertext: &str) -> Result<String, EncryptionError>;
}
