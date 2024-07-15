use chrono::NaiveDate;
use sqlx::types::BigDecimal;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use goodreads_api::goodreads_api;
pub struct Book {
    title: String,
    authors: Vec<String>,
    pages: u64,
    acquisition_date: Option<NaiveDate>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    price_ebook: Option<BigDecimal>,
    price_paperback: Option<BigDecimal>
}

impl Book {
    pub fn new(book: &goodreads_api::GoodreadsBook, date: NaiveDate) -> Self {
        
        let title = book.title();
        let authors = book.authors();
        let pages = book.pages();
        
        println!("The book is {} by {:?}, bought on {}. It has {} pages", title, authors, date, &pages);
        Self {title: title.to_string(), authors: authors.to_vec(), pages, acquisition_date: date, start_date: Option::None, end_date: Option::None, price_ebook: Option::None, price_paperback: Option::None}
    }
}

impl From<goodreads_api::GoodreadsBook> for Book {
    fn from(value: goodreads_api::GoodreadsBook) -> Self {
        Self {
            title: value.title(),
            authors: value.authors(),
            pages: value.pages(),
            acquisition_date: None,
            start_date: None,
            end_date: None,
            price_ebook: None,
            price_paperback: None
        }
    }
}

pub struct DataBaseConnection {
    database_url: String,
    pool: Pool<sqlx::Postgres>,
}

impl DataBaseConnection {

    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        
        let pool = PgPoolOptions::new().max_connections(5).connect(database_url).await.unwrap();
        let database = Self {database_url: database_url.to_owned(), pool};
        
        Ok(database)
    }

    pub async fn insert_owned_book(&self, book: Book) -> Result<(), sqlx::Error> {
        
        sqlx::query!(
            "INSERT INTO owned_books (title, num_pages, acquisition_date) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            book.title,
            book.pages as i32,
            book.acquisition_date
        ).execute(&self.pool).await?;
    
        for author in &book.authors {
            sqlx::query!(
                "INSERT INTO authors (name) VALUES ($1) ON CONFLICT (name) DO NOTHING",
                author.to_owned()
            )
            .execute(&self.pool)
            .await?;
            sqlx::query!("INSERT INTO book_authors (book_id, author_id) VALUES ((SELECT book_id FROM owned_books WHERE title = $1), (SELECT author_id FROM authors WHERE name = $2)) ON CONFLICT DO NOTHING", book.title, author).execute(&self.pool).await?;
        }
    
        Ok(())
    }

    pub async fn start_new_book(&self, book_id: i32, start_date: NaiveDate) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO read_books (book_id, start_date) VALUES ($1, $2)", book_id, start_date)
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }

    pub async fn finished_book(&self, book_id: i32, end_date: NaiveDate) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE read_books SET end_date = $1 WHERE book_id = $2", end_date, book_id)
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }

    pub async fn all_authors(&self) -> Result<(), sqlx::Error> {
        let query_result = sqlx::query!("SELECT * FROM authors")
            .fetch_all(&self.pool)
            .await?;
        for row in query_result {
            println!("{:?}", row);
        }
    
        Ok(())
    }

    pub async fn alter_start_date(&self, book_id: i32, start_date: NaiveDate) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE read_books SET start_date = $1 WHERE book_id = $2", start_date, book_id)
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }
    
    pub async fn alter_end_date(&self, book_id: i32, end_date: NaiveDate) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE read_books SET end_date = $1 WHERE book_id = $2", end_date, book_id)
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }
}