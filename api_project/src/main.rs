#[macro_use] extern crate rocket;

use rocket::{routes, Build, Rocket};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod web;
mod domains;
mod data;
mod models;
mod commons;

use web::routes::index;
use web::routes::users;


#[rocket::launch]
fn rocket() -> Rocket<Build> {
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(
        paths(
            index::index,
            users::get_users
        ),
        components(
            schemas(models::users::user::User)
        ),
    )]
    struct ApiDoc;

    rocket::build()
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/", RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .mount("/", Redoc::with_url("/redoc", ApiDoc::openapi()))
        .mount("/", routes![index::index])
        .mount("/users", routes![users::get_users])
}


