mod errors;
mod user;
mod user_repository;
pub mod user_service;

use std::sync::Arc;

pub use errors::{Result, Error};

use crate::user::user_service::UserService;

pub fn add_users() -> UserService {
    let user_repository = Arc::new(user_repository::MemoryUserRepository::new());
    UserService::new(user_repository)
}
