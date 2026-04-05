use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::errors::AppError;

type HmacSha256 = Hmac<Sha256>;

fn argon2_instance() -> Argon2<'static> {
    let params = Params::new(65536, 3, 4, Some(32)).expect("Invalid Argon2 params");
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2_instance();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hash error: {}", e)))?;
    Ok(hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("Password hash parse error: {}", e)))?;
    let argon2 = argon2_instance();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn hash_email(email: &str, secret: &str) -> String {
    let normalized = email.trim().to_lowercase();
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(normalized.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_and_verify() {
        let password = "Test1234!";
        let hash = hash_password(password).unwrap();
        assert!(hash.starts_with("$argon2id$"));
        assert!(verify_password(&hash, password).unwrap());
        assert!(!verify_password(&hash, "WrongPass1!").unwrap());
    }

    #[test]
    fn test_hash_uniqueness() {
        let h1 = hash_password("Test1234!").unwrap();
        let h2 = hash_password("Test1234!").unwrap();
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_email_deterministic() {
        let secret = "test-secret-key-32bytes-minimum!!";
        let h1 = hash_email("test@example.com", secret);
        let h2 = hash_email("test@example.com", secret);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }

    #[test]
    fn test_hash_email_normalization() {
        let secret = "test-secret-key-32bytes-minimum!!";
        let h1 = hash_email("Test@Example.COM", secret);
        let h2 = hash_email("  test@example.com  ", secret);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_email_different_secret() {
        let h1 = hash_email("test@example.com", "secret1");
        let h2 = hash_email("test@example.com", "secret2");
        assert_ne!(h1, h2);
    }
}
