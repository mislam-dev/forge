# Infrastructure Plan: Encryption

> **Plan Type:** Infrastructure
> **Priority:** P0 — Blocker
> **Status:** Not Started
> **Last Updated:** 2026-08-13

---

## 1. Overview

The Forge Platform requires **AES-256-GCM at-rest encryption** for two categories of sensitive data:

1. **Git Personal Access Tokens (PAT)** — stored in `project_repositories.access_token_encrypted`
2. **Secret environment variable values** — stored in `project_environment_variables.value_encrypted`

The encryption service lives in `src/infrastructure/encryption/`.

Both the Repository sub-module and the Environment Variables sub-module depend on this service before they can write encrypted values to the database.

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/infrastructure/encryption/mod.rs` | Exists — empty stub |
| AES-256-GCM implementation | Not implemented |
| Key management / master secret | Not implemented |
| Encryption service exposed to modules | Not implemented |

---

## 3. Dependencies

### Depends On
- Foundation (Cargo.toml)
- `MASTER_ENCRYPTION_KEY` environment variable (32-byte secret)

### Used By
- Repository sub-module (PAT encryption/decryption)
- Environment Variables sub-module (env var value encryption)
- Build Worker (env var decryption for injection, in-memory only — never logged)

---

## 4. Required Cargo Dependencies

```toml
[dependencies]
# AES-256-GCM encryption
aes-gcm = "0.10"
# Cryptographic primitives
rand = "0.8"
# Base64 encoding for ciphertext storage
base64 = "0.22"
```

---

## 5. Encryption Scheme

Per `docs/system/04-security/security-architecture.md`:

**Algorithm:** AES-256-GCM

**Process:**
1. Load master secret key from environment (`MASTER_ENCRYPTION_KEY`, 32 bytes / 256 bits)
2. Derive scoped key: `HKDF(master_key, salt=project_id, info="forge-encryption")`
3. Generate random 12-byte IV (nonce) per encryption operation
4. Encrypt plaintext with AES-256-GCM
5. Encode to storage format: `base64(IV || Ciphertext || AuthTag)`
6. Store encoded string in database column

**Decryption:**
1. Decode from base64
2. Split into IV, ciphertext, auth tag
3. Derive scoped key using same project_id salt
4. Decrypt and authenticate

---

## 6. Key Management

- Master key loaded from `MASTER_ENCRYPTION_KEY` environment variable (never hardcoded)
- Master key must be exactly 32 bytes (256 bits), provided as hex or base64 encoded string
- Key rotation is a **future enhancement** — not required for MVP
- Decryption is only authorized for:
  - Project Owner/System Admin via `GET /projects/:id/env-vars/decrypt` (API)
  - Build Worker via internal service token (runtime injection only, never logged)

---

## 7. API Contract

```rust
// Conceptual interface
pub struct EncryptionService {
    master_key: [u8; 32],
}

impl EncryptionService {
    pub fn new(master_key_hex: &str) -> Result<Self, EncryptionError>;
    pub fn encrypt(&self, plaintext: &str, project_id: Uuid) -> Result<String, EncryptionError>;
    pub fn decrypt(&self, ciphertext_b64: &str, project_id: Uuid) -> Result<String, EncryptionError>;
}
```

---

## 8. Security Rules

- **Never** log plaintext values before or after encryption
- **Never** log the master key or derived keys
- **Never** return encrypted values in plaintext via any public API endpoint — return `"••••••••"` for secret values
- PAT tokens must never appear in logs, even in base64 form
- Encryption/decryption must happen in-memory — no temporary files

---

## 9. Implementation Tasks

### Cargo Setup
- [ ] Add `aes-gcm`, `rand`, `base64` to Cargo.toml

### EncryptionService
- [ ] Implement `EncryptionService::new(master_key_hex)` — validate key length
- [ ] Implement `encrypt(plaintext, project_id)` — HKDF derive key, random IV, AES-256-GCM, base64 encode
- [ ] Implement `decrypt(ciphertext_b64, project_id)` — base64 decode, derive key, AES-256-GCM decrypt + auth
- [ ] Return typed `EncryptionError` for: invalid key, invalid ciphertext, authentication failure

### AppState Integration
- [ ] Load `MASTER_ENCRYPTION_KEY` from environment on startup
- [ ] Panic on startup if key is missing or invalid length
- [ ] Expose `EncryptionService` via `AppState`

### Testing
- [ ] Unit test: encrypt/decrypt round trip returns original plaintext
- [ ] Unit test: different project_ids produce different ciphertexts for same plaintext
- [ ] Unit test: tampered ciphertext fails authentication (auth tag verification)
- [ ] Unit test: invalid master key length fails at startup
- [ ] Unit test: encrypted value is never equal to plaintext

---

## 10. Definition of Done

- [ ] `EncryptionService` implemented with AES-256-GCM
- [ ] Encrypt/decrypt round trip verified
- [ ] Different project salts produce different ciphertexts
- [ ] Auth tag verification rejects tampered data
- [ ] `AppState` exposes `EncryptionService`
- [ ] Invalid key causes startup panic with clear error message
- [ ] No secrets appear in logs during tests

---

## 11. Estimated Effort

**Small (< 1 day)**

AES-256-GCM via `aes-gcm` crate is well-documented. The main work is the HKDF key derivation and base64 encoding scheme.

---

## 12. Recommendations

**Required:**
- Key derivation must use the project_id as a salt to ensure per-project key separation.
- The 12-byte IV must be randomly generated for every encryption call.
- The authentication tag must be verified on every decryption (GCM provides this automatically).

**Recommended:**
- Store ciphertext in a versioned format (e.g., `v1:base64...`) to allow future algorithm migration.
- Use `zeroize` crate to zero out sensitive byte arrays when they go out of scope.

**Future Enhancement:**
- Key rotation support (store key version alongside ciphertext)
- Integration with external secrets manager (HashiCorp Vault, AWS KMS)
