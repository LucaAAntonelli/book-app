use std::io;
mod db;
mod requests;

use crate::requests::Book;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut query = String::new();
    println!("Enter book to search");
    io::stdin()
        .read_line(&mut query)
        .expect("Failed to read line");
    let value = requests::search(query)
        .await
        .expect("Query returned an error");
    let books = requests::json_to_books(value);

    println!("Choose which book to add:");
    let mut choice = String::new();
    for (i, book) in books.iter().enumerate() {
        println!("{i}: {book}");
    }
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let index = choice.trim().parse::<usize>().expect("Invalid input!");

    let pool = db::connect().await?;

    db::insert_book(&books[index], &pool).await?;
    db::all_authors(&pool).await?;
    db::all_books(&pool).await?;

    Ok(())
}
