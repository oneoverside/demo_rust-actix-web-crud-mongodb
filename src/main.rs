use actix_web::{App, HttpServer};
use lib::presentation::controllers::posts::post_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(post_controller::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
