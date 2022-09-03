use std::fs::File;
use std::io;
use std::io::{BufReader, prelude::*};
use regex::Regex;
use clap::{arg, Command};

fn main() {
    let args = Command::new("grep-lite").version("0.1").author("mjh").about("searches for patterns")
                    .arg(arg!(--pattern <VALUE>))
                    .arg(arg!(--input_file <VALUE>))
                    .get_matches();
    
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input_file = args.value_of("input_file").unwrap_or("-");

    if input_file == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);

    } else {
        let f = File::open(input_file).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}: {}", i + 1, line),
            None => (),
        }
    }
}
