use std::sync::{Arc, Mutex};
use async_trait::async_trait;

use uuid::Uuid;

use crate::products::Error::{NotFound, InvalidInput, AlreadyExists};
use crate::products::Result;

use crate::products::product::Product;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn add_product(&self, name: String, price: f64) -> Result<Product>;
    async fn get_products(&self) -> Result<Vec<Product>>;
    async fn get_product_by_id(&self, id: Uuid) -> Result<Product>;
    async fn update_product(&self, id: Uuid, name: String, price: f64) -> Result<Product>;
    async fn delete_product(&self, id: Uuid) -> Result<()>;
}

#[derive(Clone)]
pub struct MemoryProductsRepository {
    products: Arc<Mutex<Vec<Product>>>,
}

impl MemoryProductsRepository {
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
        }
    }    
}

#[async_trait::async_trait]
impl ProductRepository for  MemoryProductsRepository {
    async fn add_product(&self, name: String, price: f64) -> Result<Product> {
        if name.is_empty() || price <= 0.0 {
            return Err(InvalidInput("Name cannot be empty and price must be greater than zero".to_string()));
        }
        let product = Product::new(name, price);
        let mut products = self.products.lock().unwrap();

        if products.iter().any(|p| p.name == product.name) {
            return Err(AlreadyExists(format!("Product with name '{}' already exists", product.name)));
        }

        products.push(product);
        
        Ok(products.last().cloned().unwrap())
    }

    async fn get_products(&self) -> Result<Vec<Product>> {
        let products = self.products.lock().unwrap();
        Ok(products.clone())
    }

    async fn get_product_by_id(&self, id: Uuid) -> Result<Product> {
        let products = self.products.lock().unwrap();
        let product = products.iter().find(|&p| p.id == id).cloned();

        match product {
            Some(p) => Ok(p),
            None => {
                Err(NotFound(format!("Product with id {} not found", id)))
            }
        }
    }

    async fn update_product(&self, id: Uuid, name: String, price: f64) -> Result<Product> {
        let mut products = self.products.lock().unwrap();
        match products.iter_mut().find(|p| p.id == id) {
            Some(product) => {
                if name.is_empty() || price <= 0.0 {
                    return Err(InvalidInput("Name cannot be empty and price must be greater than zero".to_string()));
                }
                product.name = name;
                product.price = price;
                Ok(product.clone())
            },
            None => Err(NotFound(format!("Product with id {} not found", id))),
        }
    }

    async fn delete_product(&self, id: Uuid) -> Result<()> {
        let mut products = self.products.lock().unwrap();
        if let Some(pos) = products.iter().position(|p| p.id == id) {
            products.remove(pos);
            Ok(())
        } else {
            Err(NotFound(format!("Product with id {} not found", id)))
        }
    }
}
