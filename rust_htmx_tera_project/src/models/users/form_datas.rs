use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFormData {
    pub name: String,
    pub email: Option<String>,
}