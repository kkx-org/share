pub mod session;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(session::init));
}
