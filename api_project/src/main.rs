#[macro_use] extern crate rocket;

use sqlx::postgres::PgPoolOptions;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;
use web::routes::{root, users};
use std::env;

mod web;
mod models;
mod domains;

use models::users::{entities::User, form_datas::UserFormData};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // env_logger::init();

    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    #[derive(OpenApi)]
    #[openapi(
        paths(
            root::index,
            users::get_users,
            users::insert_user
        ),
        components(
            schemas(User, UserFormData)
        ),
    )]
    struct ApiDoc;

    let rocket = rocket::build()
        .manage(db_pool)
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/", RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .mount("/", Redoc::with_url("/redoc", ApiDoc::openapi()))
        .mount("/", routes![root::index])
        .mount("/", routes![users::get_users])
        .mount("/", routes![users::insert_user]);

    rocket.launch().await?;

    Ok(())
}
