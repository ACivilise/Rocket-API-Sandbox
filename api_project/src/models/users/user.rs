
use serde::Serialize;

#[derive(Serialize)]
pub struct User(pub i32, pub String, pub String);