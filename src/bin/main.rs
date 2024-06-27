#![warn(clippy::all, rust_2018_idioms)]
use book_app::db::{DataBaseConnection, Book};
use calamine::{Reader, open_workbook, Xlsx, DataType};
use chrono::{Duration, NaiveDate};
use dotenv::dotenv;
use std::env;
use std::io;
use goodreads_api::goodreads_api;
use log::{info, error};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init(); // Initialize the logger
    
    let excel_path = "/mnt/c/Users/lucaa/ownCloud/books_in_possession.xlsx";
    info!("Opening workbook at {}", excel_path);
    let mut workbook: Xlsx<_> = open_workbook(excel_path).expect("Error opening workbook");
    
    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("Database URL: {}", db_uri);
    let database = DataBaseConnection::new(&db_uri).await.unwrap();
    
    if let Ok(range) = workbook.worksheet_range("Tabelle1") {
        for row in range.rows() {
            let author = &row[0].as_string().unwrap();
            let title = &row[1].as_string().unwrap();
            if let DataType::DateTime(num_days) = row[2] {
                let acquisition_date = NaiveDate::from_ymd_opt(1899, 12, 30).unwrap() + Duration::days(num_days as i64);
                let query = format!("{title} {author}");
                info!("Query: {}", query);
                
                let mut buffer = String::new();
                let books = goodreads_api::GoodreadsBook::search(&query);
                
                info!("Books found: {:?}", books);
                println!("Select a book via index:");
                for (idx, book) in books.iter().enumerate() {
                    println!("{idx}: {book}");
                }
                
                let stdin = io::stdin();
                stdin.read_line(&mut buffer).unwrap();
                let selected_book = &books[buffer.trim().parse::<usize>().unwrap()];
                let database_book = Book::new(selected_book, acquisition_date);
                
                match database.insert_owned_book(database_book).await {
                    Ok(_) => info!("Successfully written to database"),
                    Err(e) => error!("Error while writing to database: {}", e)
                }
            }
        }
    }
}
