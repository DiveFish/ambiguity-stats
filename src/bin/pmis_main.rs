extern crate clap;
extern crate ambiguity_stats;
extern crate conllx;

use clap::{Arg, App};
use conllx::Token;
use ambiguity_stats::read_sentences;
use ambiguity_stats::get_all_files;
use ambiguity_stats::get_ngram;
use ambiguity_stats::save_to_file;

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get PMI statistics of parser data.")
        .arg(Arg::with_name("INPUT_DIRECTORY")
            .help("Sets the data file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT_FILE")
            .help("Sets the output file name")
            .required(true)
            .index(2))
        .get_matches();

    let files = get_all_files(matches.value_of("INPUT_DIRECTORY").unwrap());
    let filename_template = matches.value_of("OUTPUT_FILE").unwrap();

    for file in &files {
        save_to_file(filename_template, get_ngram(& read_sentences(file), 3));
        println!("Done with file {}", file)
    }

}