mod products;
mod user;
mod web;
mod utils;

use std::sync::Arc;
use actix_web::{web::Data, App, HttpServer};
use products::products_repository::MemoryProductsRepository;
use web::product_routes::ProductRoutes;

use crate::{web::user_routes::UserRoutes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let products_repository = Arc::new(MemoryProductsRepository::new());
    let products_api = Data::new(ProductRoutes::new(products_repository));

    let users_api = Data::new(UserRoutes::new(
        Arc::new(user::add_users())
    ));

    HttpServer::new(move || {
        App::new()
            .service(ProductRoutes::scope(products_api.clone()))
            .service(UserRoutes::scope(users_api.clone()))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
