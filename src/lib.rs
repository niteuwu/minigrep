use std::process;
use std::fs::File as f;
use std::io::{prelude::*, BufReader};
use ansi_term::Colour::Green;


pub fn get_match((x, q): (BufReader<f>, &str)) {
    let lines = x.lines().map(|l| l.unwrap());
    for line in lines {
        let words: Vec<_> = line.split(' ').collect();
        if words.iter().any(|&i| i == q) {
            for word in words {
                if word == q {
                    print!("{} ", Green.paint(word));
                } else {
                    print!("{} ", word);
                }
            }
        }
        println!("");
    }
}

pub struct File {
    pub filename: String,
    pub query: String,
}

impl File {
    pub fn new(args: Vec<String>) -> File {
        if args.len() < 3 {
            println!("Not enough arguments");
            process::exit(0);
        }

        File {
            filename: args[1].clone(),
            query: args[2].clone(),
        }
    }

    pub fn reads(&self) -> (BufReader<f>, &str) {
        let file = f::open(&self.filename).expect("File doesn't exist");
        (BufReader::new(file), &self.query)
    }
}
