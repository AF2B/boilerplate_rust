use actix_web::{web, App, HttpServer};

mod routes;
mod controllers {
    pub mod users_controller;
}
mod models {
    pub mod user;
}
mod views {
    pub mod user_view;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(routes::init_routes))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
