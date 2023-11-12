use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
pub async fn connect() -> Result<Pool<sqlx::Postgres>, sqlx::Error> {
    let database_url = "postgres://postgres:mysecretpassword@localhost/postgres";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

// async fn insert() -> Result<(), sqlx::Error> {}
