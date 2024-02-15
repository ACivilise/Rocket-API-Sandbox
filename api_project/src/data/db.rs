use tokio_postgres::{NoTls, Error, Client};

pub async fn connect() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect("your_database_url", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}