#![warn(clippy::all, rust_2018_idioms)]
use book_app::db::{DataBaseConnection, Book};
use calamine::{Reader, open_workbook, Xlsx, DataType};
use chrono::{Duration, NaiveDate};
#[tokio::main]
async fn main() {
    let excel_path = "/mnt/c/Users/lucaa/polybox/books_in_possession.xlsx";
    
    let mut workbook: Xlsx<_> = open_workbook(excel_path).expect("Error opening workbook");
    let db_uri = "postgres://postgres:mysecretpassword@mypostgres.crzu5du3w8kg.eu-north-1.rds.amazonaws.com:5432/bookdb";
    let database = DataBaseConnection::new(db_uri).await.unwrap();
    if let Ok(range) = workbook.worksheet_range("Tabelle1") {
        for row in range.rows(){
            let author = &row[0].as_string().unwrap();
            let title = &row[1].as_string().unwrap();
            if let DataType::DateTime(num_days) = row[2] {
                let acquisition_date = NaiveDate::from_ymd_opt(1899, 12, 30).unwrap() + Duration::days(num_days as i64); // Copmute date with Excel period date, add number of days found in Excel sheet
                let query = format!("{title} {author}");
                println!("{query}");
                let selected_book = Book::new(&query, acquisition_date).await;
                match database.insert_owned_book(selected_book).await {
                // TODO: Add functionality to notify whenever a book was already in the database
                Ok(_) => println!("Successfully written to database"),
                Err(e) => println!("Error while writing to database: {e}")
                }
            }  
        }
    }  
}