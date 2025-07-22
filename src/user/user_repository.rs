use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use crate::user::{
    Error::{InvalidCredentials, NotFound},
    Result, errors,
};
use crate::{
    user::user::User,
    utils::password_handler::{hash_password, verify_password},
};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn add_user(&self, username: String, email: String, password: String) -> Result<User>;
    async fn get_user_by_username(&self, username: String) -> Result<User>;
    async fn control_user(&self, username: String, password: String) -> Result<User>;
    async fn get_all_users(&self) -> Result<Vec<User>>;
    async fn get_user_by_id(&self, id: u32) -> Result<User>;
}


/////////// MemoryUserRepository /////////////////////////////////////////////////////////////////////////////////

static USERS: Lazy<Arc<Mutex<Vec<User>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(Vec::new()))
});

#[derive(Clone)]
pub struct MemoryUserRepository {
    users: Arc<Mutex<Vec<User>>>,
}

impl MemoryUserRepository {
    pub fn new() -> Self {
        MemoryUserRepository {
            users: USERS.clone(),
        }
    }
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
        match hash_password(&password) {
            Some(hashed_password) => {
                let user = User {
                    id,
                    username: username.clone(),
                    email: email.clone(),
                    password: hashed_password,
                };
                users.push(user.clone());
                Ok(user)
            }
            None => Err(crate::user::Error::HashingError(
                "Failed to hash password".to_string(),
            )),
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

        match verify_password(&password, user.password.clone()) {
            Some(true) => Ok(user),
            Some(false) => Err(InvalidCredentials("Invalid password".to_string())),
            None => Err(errors::Error::HashingError(
                "Password verification failed".to_string(),
            )),
        }
    }

    async fn get_all_users(&self) -> Result<Vec<User>> {
        let users = self.users.lock().unwrap();
        Ok(users.clone())
    }

    async fn get_user_by_id(&self, id: u32) -> Result<User> {
        let users = self.users.lock().unwrap();
        users
            .iter()
            .find(|&u| u.id == id)
            .cloned()
            .ok_or_else(|| NotFound(format!("User with ID '{}' not found", id)))
    }
}
