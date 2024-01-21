use std::io;
use book_app::db::DataBase;
use book_app::requests::GoogleBooksAPI;
use chrono::NaiveDate;
#[tokio::main]
async fn main() {
    let db_uri = "postgres://postgres:mysecretpassword@mypostgres.crzu5du3w8kg.eu-north-1.rds.amazonaws.com:5432/bookdb";
    let _database = DataBase::new(db_uri).await.unwrap();
    let mut buffer = String::new();
    let api = GoogleBooksAPI::new();
    println!("Enter book");
    io::stdin().read_line(&mut buffer).unwrap();
    let response = api.search(&buffer).await.unwrap();
    let books = api.json_to_books(response);
    for (idx, book) in books.iter().enumerate() {
        println!("{idx}: {book}");
    }
    let mut selection = String::new();
    println!("Pick a book from the list: [0-9]");
    io::stdin().read_line(&mut selection).unwrap();
    let selection = selection.trim();
    let index = selection.parse::<usize>().unwrap();
    let chosen_book = books[index].clone();
    let mut date_str = String::new();
    println!("Enter acquisition date as YYYY-MM-DD");
    io::stdin().read_line(&mut date_str).unwrap();
    let date_str = date_str.trim();
    let date_as_vec = date_str.split("-").collect::<Vec<&str>>();
    let year: i32 = date_as_vec[0].parse().unwrap();
    let month: u32 = date_as_vec[1].parse().unwrap();
    let day: u32 = date_as_vec[2].parse().unwrap();
    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    println!("The book is {} by {:?}, bought on {}. It has {} pages", chosen_book.title, chosen_book.authors, date, chosen_book.pages);
}