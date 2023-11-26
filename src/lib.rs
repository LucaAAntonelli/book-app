#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

mod db;
mod requests;
// pub use db::all_books;
