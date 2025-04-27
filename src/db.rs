use chrono::NaiveDate;
use ::goodreads_api::goodreads_api::GoodreadsBook;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use goodreads_api::goodreads_api;
use log::info;

pub struct OwnedBook {
    title: String,
    num_pages: i32,
    acquisition_date: Option<NaiveDate>,
    url: String,
    cover_url: String
}

pub struct ReadBook {
    book_id: u32,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>

}




impl From<goodreads_api::GoodreadsBook> for OwnedBook {
    fn from(value: goodreads_api::GoodreadsBook) -> Self {
        Self {
            title: value.title(),
            num_pages: value.pages() as i32,
            acquisition_date: None,
            url: value.url(),
            cover_url: value.cover_image().unwrap_or("".to_string())
        }
    }
}


pub struct DataBaseConnection(Pool<sqlx::Postgres>);


impl DataBaseConnection {

    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        info!("Creating new SQL database connection pool");
        let pool = PgPoolOptions::new().max_connections(5).connect(database_url).await.unwrap();
        info!("Successfully created SQL database connection pool");
        Ok(DataBaseConnection(pool))
    }

    pub async fn insert_owned_book(&self, input_book: GoodreadsBook) -> Result<(), sqlx::Error> {
        info!("Inserting new book into database");
        let book = OwnedBook::from(input_book.clone());
        sqlx::query!(
            "INSERT INTO owned_books (title, num_pages, acquisition_date) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            book.title,
            book.num_pages as i32,
            book.acquisition_date
        ).execute(&self.0).await?;
        info!("Successfully inserted book");
    
        for author in &input_book.authors() {
            sqlx::query!(
                "INSERT INTO authors (given_names, last_name) VALUES ($1, $2) ON CONFLICT (given_names, last_name) DO NOTHING",
                author.given_names,
                author.last_name
            )
            .execute(&self.0)
            .await?;
            sqlx::query!(
                "INSERT INTO book_authors (book_id, author_id) VALUES (
                    (SELECT book_id FROM owned_books WHERE title = $1), 
                    (SELECT author_id FROM authors WHERE given_names = $2 AND last_name = $3)) ON CONFLICT DO NOTHING", 
                book.title, author.given_names, author.last_name)
                .execute(&self.0).
                await?;
        }
        info!("Successfully inserted authors");
    
        Ok(())
    }

    pub async fn start_new_book(&self, book_id: i32, start_date: NaiveDate) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO read_books (book_id, start_date) VALUES ($1, $2)", book_id, start_date)
        .execute(&self.0)
        .await?;
    
        Ok(())
    }

    pub async fn finished_book(&self, book_id: i32, end_date: NaiveDate) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE read_books SET end_date = $1 WHERE book_id = $2", end_date, book_id)
        .execute(&self.0)
        .await?;
    
        Ok(())
    }

    pub async fn all_authors(&self) -> Result<Vec<goodreads_api::Author>, sqlx::Error> {
        let query_result = sqlx::query_as!(goodreads_api::Author, "SELECT given_names, last_name FROM authors")
            .fetch_all(&self.0)
            .await?;
        for row in &query_result {
            println!("{:?}", row);
        }
    
        Ok(query_result)
    }

    pub async fn alter_start_date(&self, book_id: i32, start_date: NaiveDate) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE read_books SET start_date = $1 WHERE book_id = $2", start_date, book_id)
        .execute(&self.0)
        .await?;
    
        Ok(())
    }
    
    pub async fn alter_end_date(&self, book_id: i32, end_date: NaiveDate) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE read_books SET end_date = $1 WHERE book_id = $2", end_date, book_id)
        .execute(&self.0)
        .await?;
    
        Ok(())
    }

    pub async fn get_all_owned_books(&self) -> Result<Vec<OwnedBook>, sqlx::Error> {
        let result = sqlx::query_as!(OwnedBook, "SELECT title, num_pages, acquisition_date, url, cover_url FROM owned_books")
            .fetch_all(&self.0)
            .await?;

        Ok(result)
    }
}
