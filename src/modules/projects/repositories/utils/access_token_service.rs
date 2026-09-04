pub struct AccessTokenService;

impl AccessTokenService {
    pub fn encrypt(raw_token: &str) -> String {
        raw_token.bytes().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn decrypt(encrypted_hex: &str) -> String {
        let mut bytes = Vec::new();
        for i in (0..encrypted_hex.len()).step_by(2) {
            if i + 2 <= encrypted_hex.len() {
                if let Ok(b) = u8::from_str_radix(&encrypted_hex[i..i + 2], 16) {
                    bytes.push(b);
                }
            }
        }
        String::from_utf8(bytes).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_encryption_decryption_roundtrip() {
        let pat = "ghp_1234567890abcdefghijklmnopqrstuvwxyz";
        let encrypted = AccessTokenService::encrypt(pat);
        assert_ne!(pat, encrypted);
        let decrypted = AccessTokenService::decrypt(&encrypted);
        assert_eq!(pat, decrypted);
    }
}
