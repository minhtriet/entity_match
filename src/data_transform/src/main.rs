extern crate serde;

use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    url: String,
    text: String,
    id: String,
}

fn another() -> Result<Article, Error> {
    let data = r#"
        { "url": "John Doe", "text": "43", "id": "+44 1234567" }"#;
    let a: Article = serde_json::from_str(data)?;
    Ok(a)
}

fn main() {
    let article = another().unwrap_or_else(|error| {
        panic!("invalid JSON");
    });
    println!("{}", article.url);
}

