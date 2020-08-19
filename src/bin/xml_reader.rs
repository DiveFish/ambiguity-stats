extern crate ambiguity_stats;
extern crate clap;

use ambiguity_stats::*;
use clap::{App, Arg};
use std::fs::{File};
use std::io::{BufRead, BufReader};

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get statistics of ambiguities occurring in parser data.")
        .arg(
            Arg::with_name("INPUT_DIR")
                .help("Sets the gold data file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let dir = matches.value_of("INPUT_DIR").unwrap();
    let files = get_all_files(dir);

    for filename in files {
        let filename_copy = filename.clone();
        let filename_split = filename_copy.split("/").collect::<Vec<_>>();
        let description = filename_split[filename_split.len()-1];

        let file = File::open(filename).expect("Could not open file");
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            println!("{} > {}", description, line);
        }
    }

}