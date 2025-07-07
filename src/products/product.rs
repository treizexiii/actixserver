use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
}

impl Product  {
    pub fn new(name: String, price: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            price,
        }
    }
}
