use chrono::NaiveDate;
use ::goodreads_api::goodreads_api::GoodreadsBook;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use goodreads_api::goodreads_api;
use log::info;
pub struct Book {
    title: String,
    authors: Vec<Author>,
    num_pages: i32,
    acquisition_date: Option<NaiveDate>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

pub struct Author {
    first_name: String,
    last_name: String
}

impl From<String> for Author {
    fn from(value: String) -> Self {
        let mut parts = value.split_whitespace();
        let first_name = parts.next().expect("No first name found!").to_owned();
        let last_name = parts.next().unwrap_or("").to_owned();

        Self{ first_name, last_name}
    }
}



impl From<goodreads_api::GoodreadsBook> for Book {
    fn from(value: goodreads_api::GoodreadsBook) -> Self {
        Self {
            title: value.title(),
            authors: value.authors().iter().map(|x| Author::from(x.to_owned())).collect::<Vec<Author>>(),
            num_pages: value.pages(),
            acquisition_date: None,
            start_date: None,
            end_date: None,
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
        let book = Book::from(input_book);
        sqlx::query!(
            "INSERT INTO owned_books (title, num_pages, acquisition_date) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            book.title,
            book.num_pages as i32,
            book.acquisition_date
        ).execute(&self.0).await?;
        info!("Successfully inserted book");
    
        for author in &book.authors {
            sqlx::query!(
                "INSERT INTO authors (first_name, last_name) VALUES ($1, $2) ON CONFLICT (first_name, last_name) DO NOTHING",
                author.first_name,
                author.last_name
            )
            .execute(&self.0)
            .await?;
            sqlx::query!(
                "INSERT INTO book_authors (book_id, author_id) VALUES (
                    (SELECT book_id FROM owned_books WHERE title = $1), 
                    (SELECT author_id FROM authors WHERE first_name = $2 AND last_name = $3)) ON CONFLICT DO NOTHING", 
                book.title, author.first_name, author.last_name)
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

    pub async fn all_authors(&self) -> Result<(), sqlx::Error> {
        let query_result = sqlx::query!("SELECT * FROM authors")
            .fetch_all(&self.0)
            .await?;
        for row in query_result {
            println!("{:?}", row);
        }
    
        Ok(())
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

    pub async fn get_all_owned_books(&self) -> Result<Vec<Book>, sqlx::Error> {
        sqlx::query_as!(Book, "SELECT title, num_pages, acquisition_date FROM owned_books").fetch_all(&self.0).await
    }
}
