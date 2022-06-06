/// Statistical tools for the NYTimes Wordle game.

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BTreeMap;


fn load_words(filename: &str) -> Result<Vec<String>, io::Error> {
    let f = File::open(filename)?;
    let reader = io::BufReader::new(f);
    let mut words = Vec::<String>::new();
    for line in reader.lines() {
        words.push(line?);
    }
    Ok(words)
}

fn letter_freq(words: &Vec<String>) -> BTreeMap<char, usize> {
    let mut freq = BTreeMap::<char, usize>::new();

    for word in words {
        for c in word.chars() {
            freq.insert(c, freq.get(&c).unwrap_or(&0) + 1);
        }
    }

    freq
}

#[test]
fn test_letter_freq() {
   let freqs = letter_freq(&vec!["irate".to_string(), "abate".to_string()]);
   assert_eq!(freqs, BTreeMap::from([
       ('a', 3),
       ('b', 1),
       ('e', 2),
       ('i', 1),
       ('r', 1),
       ('t', 2),
   ]));
}

fn main() {
    let words = load_words("solutions.txt").expect("load_words");
    print!("{:?}", letter_freq(&words));
}
