use reqwest;
use serde::Deserialize;
use serde_json::Value;
use serde_json::{self, json};

pub struct GoogleBooksAPI {
    pub query_body: String,
}

impl GoogleBooksAPI {

    pub fn new() -> Self {
        Self {query_body: String::from("https://www.googleapis.com/books/v1/volumes?q=")}
    }

    pub async fn search(&self, query: &String) -> Result<String, reqwest::Error> {
        let body = reqwest::get(format!("{}{}", &self.query_body, &query))
            .await?
            .text()
            .await?;
    
        Ok(body)
    }

    pub fn json_to_books(&self, json: String) -> Vec<Book> {
        let mut books: Vec<Book> = vec![];
        let collection: Value = serde_json::from_str(&json).expect("Failed to parse JSON");
        let items = &collection["items"].as_array();
        for item in items.expect("Failed to iterate over JSON objects") {
            books.push(Book::from(item.to_owned()));
        }
    
        books
    }
}

#[derive(Deserialize)]
pub struct Book {
    pub title: String,
    pub authors: Vec<String>,
    pub pages: u64,
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}, Author(s): {}, Pages: {}",
            self.title,
            self.authors.join(", "),
            self.pages
        )
    }
}

impl std::convert::From<Value> for Book {
    fn from(json: Value) -> Self {
        let title = json["volumeInfo"]["title"].as_str().expect("Could not convert title to string").to_owned();

        let authors: Vec<String> = json["volumeInfo"]["authors"].as_array().unwrap_or(&vec![json!("None")]).iter().map(|v| v.as_str().expect("Failed to convert to string").to_owned()).collect();
        let pages = json["volumeInfo"]["pageCount"].as_u64().unwrap_or_default();

        Self {
            title,
            authors,
            pages
        }
    }
}


