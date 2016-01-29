extern crate markov;

use std::io::Read;
use std::fs::File;
use markov::Chain;

fn read_file_or_die(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

fn main() {
    let content = read_file_or_die("dump");

    let mut chain = Chain::new();
    for line in content.lines() {
        for word in line.split_whitespace() {
            chain.feed_str(&word);
        }
    }
    let words : Vec<String> = (1..100).map(|_| chain.generate_str()).collect();
    println!("{}", words.join(" "));
}
