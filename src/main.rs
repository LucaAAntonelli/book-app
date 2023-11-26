// use std::io;
// mod db;
// mod requests;

// #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {
//     let mut query = String::new();
//     println!("Enter book to search");
//     io::stdin()
//         .read_line(&mut query)
//         .expect("Failed to read line");
//     let value = requests::search(query)
//         .await
//         .expect("Query returned an error");
//     let books = requests::json_to_books(value);

//     println!("Choose which book to add:");
//     let mut choice = String::new();
//     for (i, book) in books.iter().enumerate() {
//         println!("{i}: {book}");
//     }
//     io::stdin()
//         .read_line(&mut choice)
//         .expect("Failed to read line");
//     let index = choice.trim().parse::<usize>().expect("Invalid input!");

//     let pool = db::connect().await?;

//     db::insert_book(&books[index], &pool).await?;
//     db::all_authors(&pool).await?;
//     db::all_books(&pool).await?;
//     db::all_references(&pool).await?;

//     Ok(())
// }

#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use book_app::TemplateApp;
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "Book Tracker",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    )
}
