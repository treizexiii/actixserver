use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::products::products_repository::ProductRepository;

#[derive(Clone)]
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
            .route("", web::get().to(Self::list))
            .route("/{id}", web::get().to(Self::get))
            .route("", web::post().to(Self::create))
            .route("/{id}", web::delete().to(Self::delete))
            .route("/{id}", web::put().to(Self::update))
    }

    async fn list(data: web::Data<Self>) -> impl Responder {
        let products = data.products_repo.get_products().await;
        HttpResponse::Ok().json(products.unwrap())
    }

    async fn get(data: web::Data<Self>, id: web::Path<Uuid>) -> impl Responder {
        match data.products_repo.get_product_by_id(*id).await {
            Ok(product) => HttpResponse::Ok().json(product),
            Err(err) => match err {
                crate::products::Error::NotFound(_) => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            },
        }
    }

    async fn create(data: web::Data<Self>, item: web::Json<ProductRequest>) -> impl Responder {
        let new_product = item.into_inner();
        let product = data.products_repo.add_product(new_product.name, new_product.price).await;
        match product {
            Ok(product) => HttpResponse::Created().json(product),
            Err(err) => match err {
                crate::products::Error::AlreadyExists(_) => HttpResponse::Conflict().finish(),
                crate::products::Error::InvalidInput(_) => HttpResponse::BadRequest().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            },
        }
    }

    async fn update(data: web::Data<Self>, id: web::Path<Uuid>, item: web::Json<ProductRequest>) -> impl Responder {
        let updated_product = item.into_inner();
        match data.products_repo.update_product(*id, updated_product.name, updated_product.price).await {
            Ok(product) => HttpResponse::Ok().json(product),
            Err(err) => match err {
                crate::products::Error::NotFound(_) => HttpResponse::NotFound().finish(),
                crate::products::Error::InvalidInput(_) => HttpResponse::BadRequest().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            },
        }
    }

    async fn delete(data: web::Data<Self>, id: web::Path<Uuid>) -> impl Responder {
        match data.products_repo.delete_product(*id).await {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(err) => match err {
                crate::products::Error::NotFound(_) => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            },
        }
    }
}
