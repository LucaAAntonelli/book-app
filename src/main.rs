use std::io;
mod db;
mod requests;

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
    //  println!("The query returned the following result:\n{value}");
    let books = requests::json_to_books(value);
    for book in books {
        println!("{book}");
    }

    let pool = db::connect().await?;

    let query_result = sqlx::query!("SELECT * FROM Books").fetch_all(&pool).await?;

    for row in query_result {
        let title = row.title;
        let id = row.book_id;
        println!("ID={id}, Title={title}");
    }

    Ok(())
}
