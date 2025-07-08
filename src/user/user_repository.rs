use std::sync::{Arc, Mutex};

use argon2::password_hash::rand_core::OsRng;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

use crate::user::user::User;
use crate::user::{
    Error::{InvalidCredentials, NotFound},
    Result, errors,
};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn add_user(&self, username: String, email: String, password: String) -> Result<User>;
    async fn get_user_by_username(&self, username: String) -> Result<User>;
    async fn control_user(&self, username: String, password: String) -> Result<User>;
}

#[derive(Clone)]
pub struct MemoryUserRepository {
    users: Arc<Mutex<Vec<User>>>,
}

#[async_trait::async_trait]
impl UserRepository for MemoryUserRepository {
    async fn add_user(&self, username: String, email: String, password: String) -> Result<User> {
        let mut users: std::sync::MutexGuard<'_, Vec<User>> = self.users.lock().unwrap();

        if username.is_empty() || email.is_empty() {
            return Err(crate::user::Error::InvalidInput(
                "Username and email cannot be empty".to_string(),
            ));
        }
        if password.len() < 8 {
            return Err(crate::user::Error::InvalidInput(
                "Password must be at least 8 characters long".to_string(),
            ));
        }
        if users.iter().any(|u| u.username == username) {
            return Err(crate::user::Error::AlreadyExists(format!(
                "User with username '{}' already exists",
                username
            )));
        }

        let id = users.last().map_or(0, |u| u.id) + 1;
        match Self::hash_password(&password) {
            Ok(hashed_password) => {
                let user = User::new(id, username.clone(), email.clone(), hashed_password);
                users.push(user.clone());
                Ok(user)
            }
            Err(e) => Err(e),
        }
    }

    async fn get_user_by_username(&self, username: String) -> Result<User> {
        let users = self.users.lock().unwrap();
        users
            .iter()
            .find(|&u| u.username == username)
            .cloned()
            .ok_or_else(|| NotFound(format!("User with username '{}' not found", username)))
    }

    async fn control_user(&self, username: String, password: String) -> Result<User> {
        let users = self.users.lock().unwrap();
        let user = users
            .iter()
            .find(|&u| u.username == username)
            .cloned()
            .ok_or_else(|| {
                InvalidCredentials(format!("User with username '{}' not found", username))
            })?;

        match Self::verify_password(&password, user.password.clone()) {
            Ok(true) => Ok(user),
            Ok(false) => Err(InvalidCredentials(
                "Invalid username or password".to_string(),
            )),
            Err(e) => Err(e),
        }
    }
}

impl MemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => Ok(hash.to_string()),
            Err(_) => Err(crate::user::Error::HashingError(
                "Failed to hash password".to_string(),
            )),
        }
    }

    fn verify_password(password: &str, origin: String) -> Result<bool> {
        match PasswordHash::new(&origin) {
            Ok(hash) => {
                let argon2 = Argon2::default();
                match argon2.verify_password(password.as_bytes(), &hash) {
                    Ok(_) => return Ok(true),
                    Err(_) => return Ok(false),
                }
            }
            Err(_) => {
                return Err(errors::Error::HashingError(
                    "Invalid password hash".to_string(),
                ));
            }
        };
    }
}
