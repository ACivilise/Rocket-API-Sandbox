use actix_web::{web, HttpResponse, Responder};
use tera::{Tera, Context};

pub async fn menu(tera: web::Data<Tera>) -> impl Responder {
    let context = Context::new();
    let rendered = tera.render("menu.html", &context).unwrap_or_else(|_| "Error rendering template".to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}