/// Statistical tools for the NYTimes Wordle game.

use std::fs::File;
use std::io::{self, BufRead};


fn load_words(filename: &str) -> Result<Vec<String>, io::Error> {
    let f = File::open(filename)?;
    let reader = io::BufReader::new(f);
    let mut words = Vec::<String>::new();
    for line in reader.lines() {
        words.push(line?);
    }
    Ok(words)
}

fn main() {
    println!("{:?}", load_words("solutions.txt").expect("error"));
}
