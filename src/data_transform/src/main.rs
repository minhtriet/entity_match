extern crate serde;

use serde::{Deserialize, Serialize};
use serde_json::Error;
use reqwest::Client;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

const NER_SERVER: &'static str = "127.0.0.1:5000/parse";

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    url: String,
    text: String,
    id: String,
}

// n entities -> select nC2 pair, generate from that
// [{sentence: , source: , entity[] [], sentence2: source: entity[]}, {sentence, source, entity}]
//

impl Article {
    // Deserialize magic??
}

fn another(data: &str) -> Result<Article, Error> {
    let a: Article = serde_json::from_str(data)?;
    Ok(a)
}

fn run() -> Result<u64, Box<Error>> {
    let file = File::open("data/test").unwrap();
    let client = reqwest::blocking::Client::new();
    let reader = BufReader::new(file);
    let mut count : u64 = 0;
    for line in reader.lines() {
        let article: Article = serde_json::from_str(&line.unwrap_or_default())?;
        client.post(NER_SERVER)
            .json(&format!("data={}", article.text))
            .send(); // TODO change to async
        count += 1;
    }
    Ok(count)
}

fn main() {
    match run() {
        Ok(e) => println!("{}", e),
        Err(e) => println!("{:#}", e)
    }
}

