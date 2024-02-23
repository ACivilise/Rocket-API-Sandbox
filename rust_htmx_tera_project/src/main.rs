use actix_web::{web, App, HttpServer};
use tera::Tera;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(handlers::index))
            .route("/add_item", web::get().to(handlers::add_item))
            .route("/remove_item", web::delete().to(handlers::remove_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}