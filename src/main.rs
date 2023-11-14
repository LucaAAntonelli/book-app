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

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
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
        Box::new(|cc| Box::new(book_app::TemplateApp::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(book_tracking_app::TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
