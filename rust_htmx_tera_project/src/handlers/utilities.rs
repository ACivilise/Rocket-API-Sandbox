use actix_web::{web, HttpRequest, HttpResponse};
use tera::{Tera, Context};

pub fn is_hx_request(req: &HttpRequest) -> bool {
    matches!(req.headers().get("HX-Request"), Some(header) if header == "true")
}

pub fn render_response(
    req: &HttpRequest, 
    tera: web::Data<Tera>, 
    full_layout: &str, 
    partial_path: &str
) -> HttpResponse {
    let mut context = Context::new();
    
    let partial_content = tera.render(partial_path, &context)
                               .unwrap_or_else(|_| "Error loading partial".to_string());

    let content = if is_hx_request(req) {
        partial_content
    } else {
        context.insert("partial_content", &partial_content);
        tera.render(full_layout, &context)
            .unwrap_or_else(|_| "Error rendering template".to_string())
    };

    HttpResponse::Ok().content_type("text/html").body(content)
}