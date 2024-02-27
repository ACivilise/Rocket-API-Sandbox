use actix_web::{web, HttpResponse, Responder};
use tera::{Tera, Context};

pub async fn remove_item(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("message", "This content was loaded dynamically with HTMX!");
    let rendered = tera.render("pages/home/item_add.html", &context).unwrap_or_else(|_| "Error rendering template".to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}