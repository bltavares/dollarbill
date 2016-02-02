extern crate markov;

use std::io::Read;
use std::fs::File;
use markov::Chain;

fn read_file_or_die(path: &str) -> String {
    let mut file = File::open(path).expect(&format!("The file {} was not found", path));
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect(&format!("Unable to read content of the file {} into the buffer", path));
    buffer
}

const PHRASE_TERMINATOR: &'static str = ".";

fn main() {
    let content = read_file_or_die("dump");

    let mut chain = Chain::new();
    for line in content.lines() {
        let words: Vec<String> = line.split(PHRASE_TERMINATOR)
                                     .flat_map(|x| x.split_whitespace())
                                     .map(|x| x.to_owned())
                                     .collect();
        chain.feed(words);
    }
    let words: Vec<String> = chain.str_iter_for(100).collect();
    println!("{}", words.join(" "));
}
