use actix_web::web;
pub mod session;
pub mod user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.configure(session::init).configure(user::init);
}
