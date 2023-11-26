use crate::app::BookFromTable;
use crate::requests::Book;
use chrono::NaiveDate;
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

pub async fn insert_book(book: &Book, pool: &Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO Books (title, num_pages, acquisition_date) VALUES ($1, $2, $3)",
        book.title,
        book.pages as i32,
        NaiveDate::from_ymd_opt(2021, 01, 01)
    )
    .execute(pool)
    .await?;

    for author in &book.authors {
        sqlx::query!(
            "INSERT INTO Authors (name) VALUES ($1) ON CONFLICT (name) DO NOTHING",
            author.to_owned()
        )
        .execute(pool)
        .await?;

        sqlx::query!("INSERT INTO BookAuthors (book_id, author_id) VALUES ((SELECT book_id FROM Books WHERE title = $1), (SELECT author_id FROM Authors WHERE name = $2)) ON CONFLICT DO NOTHING", book.title, author).execute(pool).await?;
    }

    Ok(())
}

// pub async fn all_books(pool: &Pool<sqlx::Postgres>) -> Result<BookFromTable, sqlx::Error> {
//     let query_result = sqlx::query!("SELECT * FROM Books").fetch_all(pool).await?;
//     let result: BookFromTable = query_result;
//     Ok(result)
// }

pub async fn all_authors(pool: &Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
    let query_result = sqlx::query!("SELECT * FROM Authors")
        .fetch_all(pool)
        .await?;
    for row in query_result {
        println!("{:?}", row);
    }

    Ok(())
}

pub async fn all_references(pool: &Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
    let query_result = sqlx::query!("SELECT * FROM BookAuthors")
        .fetch_all(pool)
        .await?;
    for row in query_result {
        println!("{:?}", row);
    }
    Ok(())
}

pub async fn alter_start_date(
    pool: &Pool<sqlx::Postgres>,
    book_id: i32,
    start_date: NaiveDate,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE Books SET start_date = $1 WHERE book_id = $2",
        start_date,
        book_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn alter_end_date(
    pool: &Pool<sqlx::Postgres>,
    book_id: i32,
    end_date: NaiveDate,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE Books SET end_date = $1 WHERE book_id = $2",
        end_date,
        book_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
