#[derive(Debug)]
pub enum EncryptionError {
    AesGcmError(String),
    Base64DecodeError(String),
    Utf8Error(String),
    BadInput(String),
}

impl std::fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionError::AesGcmError(e) => write!(f, "AesGcmError: {}", e),
            EncryptionError::Base64DecodeError(e) => write!(f, "Base64DecodeError: {}", e),
            EncryptionError::Utf8Error(e) => write!(f, "Utf8Error: {}", e),
            EncryptionError::BadInput(e) => write!(f, "BadInput: {}", e),
        }
    }
}
