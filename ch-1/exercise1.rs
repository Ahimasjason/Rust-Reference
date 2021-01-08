use std::env;
use std::fs::File;
//use std::io::prelude::BufRead;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug)]
struct WordCounter(HashMap<String, u16>);

impl WordCounter {
    fn new() -> Self {
        WordCounter(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(&self) {
        for (key, value) in self.0.iter() {
            println!("{} : {}", key, value);
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 1 {
        panic!("Provide enough arguments {:?}", arguments);
    }

    let file_name = &arguments[1];

    let file = File::open(file_name).expect("Unknown File");

    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let l = line.expect("Could not read");
        if l != "" {
            let words = l.split(" ");
            for w in words {
                if w == "" {
                    continue;
                } else {
                    word_counter.increment(w);
                }
            }
        }
    }
    word_counter.display();
}
