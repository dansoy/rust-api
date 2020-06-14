use actix_web::web;
use super::handler;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register")
            .route(web::post().to(handler::register)),
        )
        .service(web::resource("/auth")
            .route(web::post().to(handler::login))
            .route(web::delete().to(handler::logout))
            .route(web::get().to(handler::get_me)),
        );
}