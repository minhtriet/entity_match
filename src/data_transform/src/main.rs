extern crate serde;

use serde::{Deserialize, Serialize};

use serde_json::json;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::time::Duration;

const NER_SERVER: &'static str = "http://127.0.0.1:5000/parse";

#[derive(Serialize, Deserialize)]
struct Article {
    url: String,
    text: String,
    id: String,
}

#[derive(Serialize, Deserialize)]
struct NER {
    begin: u32,
    end: u32,
}

// n entities -> select nC2 pair, generate from that
// [{sentence: , source: , entity[] [], sentence2: source: entity[]}, {sentence, source, entity}]
//

impl Article {
    // Deserialize magic??
}


fn run() -> Result<u64, Box<Error>> {
    let file = File::open("data/test").unwrap();
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(100))
        .build()?;
    let reader = BufReader::new(file);
    let mut count : u64 = 0;
    for line in reader.lines() {
        let article: Article = serde_json::from_str(&line.unwrap_or_default())?;
        let ners: Vec<NER> = client.post(NER_SERVER)
            .json(&json!({"data": article.text}))
            .send()?.json()?; // TODO change to async
        for item in ners.iter() {
            println!("{} {}", item.begin, item.end);
        }
        count = count + 1;
    }
    Ok(count)
}

fn main() {
    match run() {
        Ok(e) => println!("{}", e),
        Err(e) => println!("{:#}", e)
    }
}

