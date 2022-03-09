use actix_web::{delete, http::StatusCode, put, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response<T, E> {
    data: Option<T>,
    error: Option<E>,
}

#[delete("/")]
async fn delete() -> HttpResponse {
    HttpResponse::Ok()
        .status(StatusCode::OK)
        .body("PETTHEPEPEGA")
}

#[put("/")]
async fn create() -> HttpResponse {
    HttpResponse::Ok()
        .status(StatusCode::OK)
        .body("PETTHEPEPEGA")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/session").service(create));
}
