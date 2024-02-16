use utoipa::ToSchema;
use rocket::serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, ToSchema, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
}