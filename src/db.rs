use crate::app::BookFromTable;
use chrono::NaiveDate;
use sqlx::types::BigDecimal;
use crate::requests::GoogleBooksAPI;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, query};
use std::io;

pub struct Book {
    title: String,
    authors: Vec<String>,
    pages: u64,
    acquisition_date: NaiveDate,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    price_ebook: Option<BigDecimal>,
    price_paperback: Option<BigDecimal>
}

impl Book {
    pub async fn new(query: &String, date: NaiveDate) -> Self {
        let api = GoogleBooksAPI::new();
        let query_result = api.search(query).await.unwrap();
        let books = api.json_to_books(query_result);
        for (idx, book) in books.iter().enumerate() {
            println!("{idx}:{book} ");
        }
        println!("Pick a book from the list: [0-9]");
        let mut selection = String::new();
        io::stdin().read_line(&mut selection).unwrap();
        let selection = selection.trim();
        let index = selection.parse::<usize>().unwrap();
        let chosen_book = books[index].clone();
       
        println!("The book is {} by {:?}, bought on {}. It has {} pages", chosen_book.title, chosen_book.authors, date, chosen_book.pages);
        Self {title: chosen_book.title, authors: chosen_book.authors, pages: chosen_book.pages, acquisition_date: date, start_date: Option::None, end_date: Option::None, price_ebook: Option::None, price_paperback: Option::None}
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

    pub async fn insert_book(&self, book: Book) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO Books (title, num_pages, acquisition_date, start_date, end_date, price_ebook, price_paperback) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (title, num_pages, acquisition_date) DO NOTHING",
            book.title,
            book.pages as i32,
            book.acquisition_date,
            book.start_date,
            book.end_date,
            book.price_ebook,
            book.price_paperback
        )
        .execute(&self.pool)
        .await?;
    
        for author in &book.authors {
            sqlx::query!(
                "INSERT INTO Authors (name) VALUES ($1) ON CONFLICT (name) DO NOTHING",
                author.to_owned()
            )
            .execute(&self.pool)
            .await?;
    
            sqlx::query!("INSERT INTO BookAuthors (book_id, author_id) VALUES ((SELECT book_id FROM Books WHERE title = $1), (SELECT author_id FROM Authors WHERE name = $2)) ON CONFLICT DO NOTHING", book.title, author).execute(&self.pool).await?;
        }
    
        Ok(())
    }

//     pub async fn all_authors(&self) -> Result<(), sqlx::Error> {
//         let query_result = sqlx::query!("SELECT * FROM Authors")
//             .fetch_all(&self.pool)
//             .await?;
//         for row in query_result {
//             println!("{:?}", row);
//         }
    
//         Ok(())
//     }

//     pub async fn alter_start_date(&self, book_id: i32, start_date: NaiveDate) -> Result<(), sqlx::Error> {
//         sqlx::query!("UPDATE Books SET start_date = $1 WHERE book_id = $2", start_date, book_id)
//         .execute(&self.pool)
//         .await?;
    
//         Ok(())
//     }
    
//     pub async fn alter_end_date(&self, book_id: i32, end_date: NaiveDate) -> Result<(), sqlx::Error> {
//         sqlx::query!("UPDATE Books SET end_date = $1 WHERE book_id = $2", end_date, book_id)
//         .execute(&self.pool)
//         .await?;
    
//         Ok(())
//     }
}




// pub async fn all_books(pool: &Pool<sqlx::Postgres>) -> Result<BookFromTable, sqlx::Error> {
//     let query_result = sqlx::query!("SELECT * FROM Books").fetch_all(pool).await?;
//     let result: BookFromTable = query_result;
//     Ok(result)
// }



// pub async fn all_references(pool: &Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
//     let query_result = sqlx::query!("SELECT * FROM BookAuthors")
//         .fetch_all(pool)
//         .await?;
//     for row in query_result {
//         println!("{:?}", row);
//     }
//     Ok(())
// }



// pub async fn all_read_books(pool: &Pool<sqlx::Postgres>) -> Result<Vec<BookFromTable>, sqlx::Error> {
//     let result = sqlx::query_as!(BookFromTable, "
//     SELECT 
//         b.title AS title, 
//         string_agg(a.name, ', ') AS authors,
//         b.num_pages AS num_pages,
//         b.acquisition_date AS acquisition_date,
//         b.start_date AS start_date,
//         b.end_date AS end_date,
//         b.price_ebook AS price_ebook,
//         b.price_paperback AS price_paperback
//     FROM 
//         Books b
//     JOIN 
//         BookAuthors ba ON b.book_id = ba.book_id
//     JOIN 
//         Authors a ON ba.author_id = a.author_id
//     GROUP BY 
//         b.book_id;").fetch_all(pool).await?;

   
//     Ok(result)
// } 