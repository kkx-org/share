use crate::utils::error::ApiErrorResponse;
use actix_web::{
    web::{FormConfig, JsonConfig, PathConfig, QueryConfig, ServiceConfig},
    HttpResponse,
};

pub fn init_errors(cfg: &mut ServiceConfig) {
    cfg.app_data(JsonConfig::default().error_handler(|err, _req| {
        let err_str = err.to_string();
        actix_web::error::InternalError::from_response(
            err,
            HttpResponse::BadRequest().json(ApiErrorResponse::Message {
                code: String::from("invalid_json_payload"),
                message: err_str,
            }),
        )
        .into()
    }))
    .app_data(QueryConfig::default().error_handler(|err, _req| {
        let err_str = err.to_string();
        actix_web::error::InternalError::from_response(
            err,
            HttpResponse::BadRequest().json(ApiErrorResponse::Message {
                code: String::from("invalid_query"),
                message: err_str,
            }),
        )
        .into()
    }))
    .app_data(PathConfig::default().error_handler(|err, _req| {
        let err_str = err.to_string();
        actix_web::error::InternalError::from_response(
            err,
            HttpResponse::NotFound().json(ApiErrorResponse::Message {
                code: String::from("invalid_path"),
                message: err_str,
            }),
        )
        .into()
    }))
    .app_data(FormConfig::default().error_handler(|err, _req| {
        let err_str = err.to_string();
        actix_web::error::InternalError::from_response(
            err,
            HttpResponse::BadRequest().json(ApiErrorResponse::Message {
                code: String::from("invalid_form_payload"),
                message: err_str,
            }),
        )
        .into()
    }));
}
