use crate::data;
use crate::models;
use tokio_postgres::Error;

pub async fn get_all_users() -> Result<Vec<models::users::user::User>, Error> {
    let client = data::db::connect().await?;

    let rows = client
        .query("SELECT id, name, email FROM users", &[])
        .await?;

    let mut users = Vec::new();

    for row in rows {
        let user = models::users::user::User(row.get(0), row.get(1), row.get(2));

        users.push(user);
    }

    Ok(users)
}