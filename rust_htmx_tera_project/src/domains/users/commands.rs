use sqlx::PgPool;
use crate::models::users::entities::User;

// Assuming `name` and `email` are the only fields you need to insert, and `id` is auto-generated.
pub async fn create_user(db: &PgPool, name: &str, email: Option<&str>) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        name,
        email
    )
    .fetch_one(db)
    .await?;
    
    Ok(user)
}