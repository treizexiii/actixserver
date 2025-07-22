// use actix_web::{
//     dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}};
    
// pub struct Authorization;

// impl <S, B> Transform <S, ServiceRequest> for Authorization
// where 
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Transform = AuthorizationMiddleware<S>;
//     type Future = Ready<Result<Self::Transform, Self::Error>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ready(Ok(AuthorizationMiddleware { service }))
//     }
// }

// pub struct AuthorizationMiddleware<S> {
//     service: S,
// }

// impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = S::Future;

//     forward_ready!(service);

//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         // Here you can add your authorization logic
//         // For example, checking headers or tokens

//         let auth_header = req.headers().get("Authorization");
//         if let Some(auth_value) = auth_header {
//             if let Ok(token) = auth_value.to_str() {
//                 // Validate the token (this is just a placeholder, implement your logic)

                
//                 if token == "valid_token" {
//                     // Token is valid, proceed with the request
//                     return self.service.call(req);
//                 }
//             }
//         }
//         // If the token is invalid or not present, return an error response
//         let response = req.error_response(
//             actix_web::HttpResponse::Unauthorized().body("Unauthorized")
//         );
//         return futures::future::ok(response);
//     }
// }
