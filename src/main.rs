extern crate markov;
extern crate language_detector;

use std::io::Read;
use std::fs::File;
use markov::Chain;
use language_detector::English;

fn read_file_or_die(path: &str) -> String {
    let mut file = File::open(path).expect(&format!("The file {} was not found", path));
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect(&format!("Unable to read content of the file {} into the buffer",
                         path));
    buffer
}

const PHRASE_TERMINATOR: &'static str = ".";

fn main() {
    let content = read_file_or_die("dump");

    let mut chain = Chain::new();
    let detector = English::new();

    let filter_english = std::env::var("FILTER_ENGLISH").is_ok();
    let only_english = std::env::var("ONLY_ENGLISH").is_ok();


    for line in content.lines().filter(|x| {
        if filter_english {
            !detector.is_english(x)
        } else if only_english {
            detector.is_english(x)
        } else {
            true
        }
    }) {
        let words: Vec<String> = line.split(PHRASE_TERMINATOR)
                                     .flat_map(|x| x.split_whitespace())
                                     .map(|x| x.to_owned())
                                     .collect();
        chain.feed(words);
    }
    let words: Vec<String> = chain.str_iter_for(100).collect();
    println!("{}", words.join(" "));
}
