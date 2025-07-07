mod products;
mod product_routes;

mod user;

use std::sync::Arc;

use actix_web::{web, App, HttpServer};

use products::products_repository::MemoryProductsRepository;
use product_routes::ProductRoutes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let products_repository = Arc::new(MemoryProductsRepository::new());
    let products_api = web::Data::new(ProductRoutes::new(products_repository));

    HttpServer::new(move || {
        App::new()
            .service(ProductRoutes::scope(products_api.clone()))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
