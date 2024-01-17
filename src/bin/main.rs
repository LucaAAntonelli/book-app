use std::io;
use book_app::db::DataBase;
use book_app::requests::GoogleBooksAPI;
#[tokio::main]
async fn main() {
    let db_uri = "postgres://postgres:mysecretpassword@mypostgres.crzu5du3w8kg.eu-north-1.rds.amazonaws.com:5432/bookdb";
    let database = DataBase::new(db_uri).await.unwrap();
    let mut buffer = String::new();
    let api = GoogleBooksAPI::new();
    println!("Enter book");
    io::stdin().read_line(&mut buffer).unwrap();
    let response = api.search(&buffer).await.unwrap();
    let books = api.json_to_books(response);
    for book in books {
        println!("{book}");
    }
}