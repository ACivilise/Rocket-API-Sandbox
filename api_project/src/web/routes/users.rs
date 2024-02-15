
use crate::domains;
use crate::models;
use rocket::serde::json::Json;

use models::users::user::User;
use domains::users::users::get_all_users;

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Obtenir la liste des utilisateurs", body = Vec<User>)
    )
)]
#[get("/")]
pub async fn get_users() -> Option<Json<Vec<User>>> {
    get_all_users().await.map(Json)
}