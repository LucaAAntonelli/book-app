use std::io;
mod db;
mod requests;

use crate::requests::Book;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // let mut query = String::new();
    // println!("Enter book to search");
    // io::stdin()
    //     .read_line(&mut query)
    //     .expect("Failed to read line");
    // let value = requests::search(query)
    //     .await
    //     .expect("Query returned an error");
    // //  println!("The query returned the following result:\n{value}");
    // let books = requests::json_to_books(value);
    // for book in books {
    //     println!("{book}");
    // }

    let pool = db::connect().await?;

    let book = Book {
        title: String::from("Test"),
        authors: vec![String::from("Max Mustermann")],
        pages: 7,
    };

    db::insert_book(book, &pool).await?;
    db::all_books(&pool).await?;

    Ok(())
}
