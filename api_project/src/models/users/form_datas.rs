
use utoipa::ToSchema;
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserFormData {
    pub name: String,
    pub email: Option<String>,
}