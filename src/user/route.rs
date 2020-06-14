use actix_web::web;
use super::handler;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users")
            .route(web::get().to(handler::get_users))
            .route(web::post().to(handler::add_user))
            .route(web::put().to(handler::update_user)),
        )
        .service(web::resource("/users/{id}")
            .route(web::get().to(handler::get_user_by_id))
            .route(web::delete().to(handler::delete_user)),
        );
}