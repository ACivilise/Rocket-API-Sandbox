use actix_web::{web, HttpResponse,HttpRequest};
use tera::{Tera};
use crate::handlers::{utilities::render_response};

pub async fn services(req: HttpRequest,tera: web::Data<Tera>) -> HttpResponse {
    const PARTIAL: &str = "pages/services/index.html";
    render_response(&req, tera, "index.html", PARTIAL)
}