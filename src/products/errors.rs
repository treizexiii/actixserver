use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    NotFound(String),
    AlreadyExists(String),
    InvalidInput(String),
}