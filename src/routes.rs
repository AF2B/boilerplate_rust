use actix_web::web;

use crate::controllers::users_controller::{get_all_users};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_all_users))
            // .route("", web::post().to(users_controller::create_user))
            // .route("/{id}", web::get().to(users_controller::get_user_by_id))
            // .route("/{id}", web::put().to(users_controller::update_user))
            // .route("/{id}", web::delete().to(users_controller::delete_user))
    );
}
