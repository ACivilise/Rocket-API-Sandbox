use actix_web::{web, HttpResponse, Responder};
use tera::{Tera, Context};

pub async fn add_item(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("message", "This content was loaded dynamically with HTMX!");
    
    // Render the template with the context
    let rendered = tera.render("item_remove.html", &context)
                        .unwrap_or_else(|_| "Error rendering template".to_string());

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn remove_item(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("message", "This content was loaded dynamically with HTMX!");
    
    // Render the template with the context
    let rendered = tera.render("item_add.html", &context)
                        .unwrap_or_else(|_| "Error rendering template".to_string());

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn index(tera: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("name", "World");
    let rendered = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}