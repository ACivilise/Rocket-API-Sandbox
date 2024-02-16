
use crate::domains;
use crate::models;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;

use models::users::{entities::User, form_datas::UserFormData};
use domains::users::{queries::get_all_users, commands::create_user};

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Obtenir la liste des utilisateurs", body = Vec<User>)
    )
)]
#[get("/users")]
pub async fn get_users(db: &State<PgPool>) -> Option<Json<Vec<User>>> {
    get_all_users(db.inner()).await.ok().map(Json)
}


#[utoipa::path(
    post,
    path = "/users",
    request_body(content = UserFormData, description = "User to store in the database", content_type = "application/json"),
    responses(
        (status = 200, description = "Obtenir la liste des utilisateurs", body = Vec<User>)
    )
)]
#[post("/users", format = "json", data = "<user_data>")]
pub async fn insert_user(db: &rocket::State<PgPool>, user_data: rocket::serde::json::Json<UserFormData>) -> Result<(), String> {
    let result = create_user(db.inner(), &user_data.name, user_data.email.as_deref()).await;
    match result {
        Ok(user) => Ok(()),
        Err(e) => Err(format!("Failed to insert user: {}", e)),
    }
}