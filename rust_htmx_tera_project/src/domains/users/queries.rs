use crate::models::users::entities::User;
use sqlx::PgPool;

pub async fn get_all_users(db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(db)
        .await?;    
    Ok(users)
}