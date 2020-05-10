extern crate serde;

use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    url: String,
    text: String,
    id: String,
}

fn another(data: &str) -> Result<Article, Error> {
    let a: Article = serde_json::from_str(data)?;
    Ok(a)
}

fn main() {
     let file = File::open("data/test").unwrap();
     let reader = BufReader::new(file);
     for line in reader.lines() {
         let json = another(&line.unwrap()).unwrap_or_else(|error| {
             panic!("invalid JSON");
         });
         println!("{}", json.text)
     }
}

