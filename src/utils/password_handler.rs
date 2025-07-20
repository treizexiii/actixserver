use argon2::password_hash::rand_core::OsRng;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

pub fn hash_password(password: &str) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Some(hash.to_string()),
        Err(_) => None
    }
}

pub fn verify_password(password: &str, origin: String) -> Option<bool> {
    match PasswordHash::new(&origin) {
        Ok(hash) => {
            let argon2 = Argon2::default();
            match argon2.verify_password(password.as_bytes(), &hash) {
                Ok(_) => return Some(true),
                Err(_) => return Some(false),
            }
        }
        Err(_) => {
            return Some(false);
        }
    };
}
