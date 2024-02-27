use actix_web::{web, App, HttpServer};
use tera::Tera;

mod handlers;
mod models;
mod domains;

use handlers::{
    about::{index::about},
    contact::{index::contact},
    home::{index::home, add_item::add_item, remove_item::remove_item},
    menu::{index::menu},
    services::{index::services}
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/about", web::get().to(about))
            .route("/contact", web::get().to(contact))
            .route("/", web::get().to(home))
            .route("/home", web::get().to(home))
            .route("/add_item", web::get().to(add_item))
            .route("/remove_item", web::delete().to(remove_item))
            .route("/menu", web::get().to(menu))
            .route("/services", web::get().to(services))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}