use actix_web::{web, HttpResponse, Responder};
use tera::{Tera, Context};

pub async fn index(tera: web::Data<Tera>) -> HttpResponse {
    let context = Context::new();
    let rendered = tera.render("index.html", &context).unwrap_or_else(|_| "Error rendering template".to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn menu(tera: web::Data<Tera>) -> impl Responder {
    let context = Context::new();
    let rendered = tera.render("menu.html", &context).unwrap_or_else(|_| "Error rendering template".to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn home(tera: web::Data<Tera>) -> HttpResponse {
    let context = Context::new();
    let rendered = tera.render("pages/home/index.html", &context).unwrap_or_else(|_| "Error rendering template".to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn about(tera: web::Data<Tera>) -> HttpResponse {
    let context = Context::new();
    let rendered = tera.render("pages/about/index.html", &context).unwrap_or_else(|_| "Error rendering template".to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn contact(tera: web::Data<Tera>) -> HttpResponse {
    let context = Context::new();
    let rendered = tera.render("pages/contact/index.html", &context).unwrap_or_else(|_| "Error rendering template".to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn services(tera: web::Data<Tera>) -> HttpResponse {
    let context = Context::new();
    let rendered = tera.render("pages/services/index.html", &context).unwrap_or_else(|_| "Error rendering template".to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}


pub async fn add_item(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("message", "This content was loaded dynamically with HTMX!");
    let rendered = tera.render("pages/home/item_remove.html", &context).unwrap_or_else(|_| "Error rendering template".to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn remove_item(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("message", "This content was loaded dynamically with HTMX!");
    let rendered = tera.render("pages/home/item_add.html", &context).unwrap_or_else(|_| "Error rendering template".to_string());
    HttpResponse::Ok().content_type("text/html").body(rendered)
}