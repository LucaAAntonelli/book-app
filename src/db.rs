use chrono::NaiveDate;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;

use crate::requests::Book;

pub async fn connect() -> Result<Pool<sqlx::Postgres>, sqlx::Error> {
    let database_url = "postgres://postgres:mysecretpassword@localhost/postgres";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

pub async fn insert_book(book: Book, pool: &Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
    println!("insert_book");
    sqlx::query!(
        "INSERT INTO Books (title, num_pages, acquisition_date) VALUES ($1, $2, $3)",
        book.title,
        book.pages as i32,
        NaiveDate::from_ymd_opt(2021, 01, 01)
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn all_books(pool: &Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
    println!("all_books");
    let query_result = sqlx::query!("SELECT * FROM Books").fetch_all(pool).await?;
    for row in query_result {
        println!("{:?}", row);
    }

    Ok(())
}
