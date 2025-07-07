
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String
}

impl User {
    pub fn new(id: u32, username: String, email: String, password: String) -> Self {
        Self {
            id, // ID will typically be set by the database
            username,
            email,
            password
        }
    }
}
