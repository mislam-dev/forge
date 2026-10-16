use crate::infrastructure::encryption::{
    AESSymmetricEncryptionService, EncryptionError, EncryptionServiceTraits,
};

pub struct AccessTokenService;

impl AccessTokenService {
    pub fn encrypt(raw_token: &str, key: &str) -> Result<String, EncryptionError> {
        let service = AESSymmetricEncryptionService::new(key.to_string())?;
        service.encrypt(raw_token.as_bytes())
    }

    pub fn decrypt(encrypted_hex: &str, key: &str) -> Result<String, EncryptionError> {
        let service = AESSymmetricEncryptionService::new(key.to_string())?;
        service.decrypt(encrypted_hex)
    }
}
