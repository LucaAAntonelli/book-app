#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

mod new_requests;
mod requests;
pub use new_requests::HttpApp;
