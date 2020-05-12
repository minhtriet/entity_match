extern crate serde;

use serde::{Deserialize, Serialize};
use serde_json::Error;
use reqwest::Client;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

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

    pub fn entities(client: Client, link: &str) {
        // Send sentence to `link`, return a list of entities span
        client.post("www.google.com");
    }
    // write function

    // Deserialize magic??
}

fn another(data: &str) -> Result<Article, Error> {
    let a: Article = serde_json::from_str(data)?;
    Ok(a)
}

fn run() -> Result<u64, Box<Error>> {
    let file = File::open("data/test").unwrap();
    let reader = BufReader::new(file);
    let mut count : u64 = 0;
    for line in reader.lines() {
        let article: Article = serde_json::from_str(&line.unwrap_or_default())?;
        count += 1;
        println!("{}", article.text)
    }
    Ok(count)
}

fn main() {
    match run() {
        Ok(e) => println!("{}", e),
        Err(e) => println!("{:#}", e)
    }
}

