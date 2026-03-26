#![forbid(unsafe_code)]

// TODO: your code goes here.
//
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut h = HashSet::<String>::new();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        h.insert(line);
    }

    let file = File::open(&args[2]).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if h.contains(&line) {
            println!("{}", line);
            h.remove(&line);
        }
    }
}
