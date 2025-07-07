use std::sync::Arc;

use actix_web::{web, Responder, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::products::products_repository::ProductRepository;

pub struct ProductRoutes {
    products_repo: Arc<dyn ProductRepository>,
}

#[derive(Deserialize, Serialize)]
pub struct ProductRequest {
    name: String,
    price: f64,
}

impl ProductRoutes {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self {
            products_repo: repository,
        }
    }

    pub fn scope(data: web::Data<Self>) -> Scope {
        web::scope("/products")
            .app_data(data.clone())
            .route("", actix_web::web::get().to(Self::list))
            .route("/{id}", actix_web::web::get().to(Self::get))
            .route("", actix_web::web::post().to(Self::create))
            .route("/{id}", actix_web::web::delete().to(Self::delete))
            .route("/{id}", actix_web::web::put().to(Self::update))
    }

    pub async fn list(data: web::Data<Self>) -> impl Responder {
        let products = data.products_repo.get_products().await;
        actix_web::HttpResponse::Ok().json(products.unwrap())
    }

    pub async fn get(data: web::Data<Self>, id: web::Path<Uuid>) -> impl Responder {
        match data.products_repo.get_product_by_id(*id).await {
            Ok(product) => actix_web::HttpResponse::Ok().json(product),
            Err(err) => match err {
                crate::products::Error::NotFound(_) => actix_web::HttpResponse::NotFound().finish(),
                _ => actix_web::HttpResponse::InternalServerError().finish(),
            },
        }
    }

    pub async fn create(data: web::Data<Self>, item: web::Json<ProductRequest>) -> impl Responder {
        let new_product = item.into_inner();
        let product = data.products_repo.add_product(new_product.name, new_product.price).await;
        match product {
            Ok(product) => actix_web::HttpResponse::Created().json(product),
            Err(err) => match err {
                crate::products::Error::AlreadyExists(_) => actix_web::HttpResponse::Conflict().finish(),
                crate::products::Error::InvalidInput(_) => actix_web::HttpResponse::BadRequest().finish(),
                _ => actix_web::HttpResponse::InternalServerError().finish(),
            },
        }
    }

    pub async fn update(data: web::Data<Self>, id: web::Path<Uuid>, item: web::Json<ProductRequest>) -> impl Responder {
        let updated_product = item.into_inner();
        match data.products_repo.update_product(*id, updated_product.name, updated_product.price).await {
            Ok(product) => actix_web::HttpResponse::Ok().json(product),
            Err(err) => match err {
                crate::products::Error::NotFound(_) => actix_web::HttpResponse::NotFound().finish(),
                crate::products::Error::InvalidInput(_) => actix_web::HttpResponse::BadRequest().finish(),
                _ => actix_web::HttpResponse::InternalServerError().finish(),
            },
        }
    }

    pub async fn delete(data: web::Data<Self>, id: web::Path<Uuid>) -> impl Responder {
        match data.products_repo.delete_product(*id).await {
            Ok(_) => actix_web::HttpResponse::NoContent().finish(),
            Err(err) => match err {
                crate::products::Error::NotFound(_) => actix_web::HttpResponse::NotFound().finish(),
                _ => actix_web::HttpResponse::InternalServerError().finish(),
            },
        }
    }
}