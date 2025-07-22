use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::{Error, user_repository::UserRepository};

#[derive(Clone)]
pub struct UserService {
    repository: Arc<dyn UserRepository>,
    tokens: Arc<Mutex<HashMap<Uuid, UserInfo>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserService {
            repository,
            tokens: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn register_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<UserInfo, Error> {
        if request.username.is_empty() || request.email.is_empty() || request.password.is_empty() {
            return Err(Error::InvalidInput("Username, email, and password cannot be empty".to_string()));
        }
        if request.password.len() < 8 {
            return Err(Error::InvalidInput("Password must be at least 8 characters long".to_string()));
        }
        let user = self.repository.add_user(request.username, request.email, request.password).await?;
        Ok(UserInfo {
            username: user.username,
            email: user.email,
            last_login: None,
        })
    }

    pub async fn login(&self, request : LoginRequest) -> Result<String, Error> {
        if request.username.is_empty() || request.password.is_empty() {
            return Err(Error::InvalidCredentials("Invalid username or password".to_string()));
        }
        let user = self.repository.control_user(request.username, request.password).await?;
        let token = Uuid::new_v4();
        let mut tokens = self.tokens.lock().unwrap();
        let last_login = Some(Utc::now());
        tokens.insert(
            token, 
            UserInfo {
                username: user.username.clone(),
                email: user.email.clone(),
                last_login,
            },
        );
        Ok(token.to_string())
    }

    pub async fn get_user_info(&self, username: String) -> Result<UserInfo, Error> {
        let user = self.repository.get_user_by_username(username).await?;
        Ok(UserInfo {
            username: user.username,
            email: user.email,
            last_login: None,
        })
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserInfo>, Error> {
        let users = self.repository.get_all_users().await?;
        Ok(users.into_iter().map(|user| UserInfo {
            username: user.username,
            email: user.email,
            last_login: None,
        }).collect())
    }
    
    pub async fn get_user_by_id(&self, id: u32) -> Result<UserInfo, Error> {
        let user = self.repository.get_user_by_id(id).await?;
        Ok(UserInfo {
            username: user.username,
            email: user.email,
            last_login: None,
        })
    }
}
