use utoipa::ToSchema;
use rocket::serde::Serialize;

#[derive(Serialize, ToSchema)]
pub struct User(pub i32, pub String, pub String);