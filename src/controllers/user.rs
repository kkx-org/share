use crate::{models::user::CreateUserDto, services, utils::error::CustomError};
use actix_web::{http::StatusCode, put, web, HttpResponse};
use sqlx::PgPool;

#[put("/")]
async fn register(
    data: web::Json<CreateUserDto>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, CustomError> {
    services::user::register(data.0, &*db).await?;

    Ok(HttpResponse::Ok().status(StatusCode::OK).finish())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(register));
}
