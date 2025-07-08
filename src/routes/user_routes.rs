use std::sync::Arc;

use actix_web::{HttpResponse, Responder, Scope, http::header::HeaderName, web};

use crate::user::{
    Error::{AlreadyExists, InvalidCredentials, InvalidInput},
    user_service::{CreateUserRequest, LoginRequest, UserService},
};

pub struct UserRoutes {
    user_service: Arc<UserService>,
}

impl UserRoutes {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    pub fn scope(data: web::Data<Self>) -> Scope {
        web::scope("/")
            .app_data(data.clone())
            .route("/register", web::post().to(Self::register))
            .route("/login", web::post().to(Self::login))
    }

    async fn register(data: web::Data<Self>, item: web::Json<CreateUserRequest>) -> impl Responder {
        match data.user_service.create_user(item.into_inner()).await {
            Ok(user_info) => HttpResponse::Created().json(user_info),
            Err(err) => match err {
                AlreadyExists(_) => HttpResponse::Conflict().finish(),
                InvalidInput(_) => HttpResponse::BadRequest().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            },
        }
    }

    async fn login(data: web::Data<Self>, item: web::Json<LoginRequest>) -> impl Responder {
        match data.user_service.login(item.into_inner()).await {
            Ok(token) => {
                HttpResponse::Ok()
                    .append_header(
                        (HeaderName::from_static("authorization"), 
                        format!("Bearer {}", token)
                    ))
                    .finish()
            }
            Err(err) => match err {
                InvalidCredentials(_) => HttpResponse::Unauthorized().finish(),
                InvalidInput(_) => HttpResponse::BadRequest().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            },
        }
    }
}
