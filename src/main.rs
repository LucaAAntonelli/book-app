use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=mysecretpassword dbname=postgres",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection failed: {}", e);
        }
    });

    Ok(())
}
