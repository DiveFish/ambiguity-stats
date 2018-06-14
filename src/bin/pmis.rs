extern crate clap;
extern crate ambiguity_stats;

use clap::{Arg, App};
use ambiguity_stats::read_sentences;
use ambiguity_stats::get_ngram;
use ambiguity_stats::save_to_file;

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get PMI statistics of parser data.")
        .arg(Arg::with_name("INPUT_DATA")
            .help("Sets the data file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT_FILE")
            .help("Sets the output file name")
            .required(true)
            .index(2))
        .get_matches();

    let text = read_sentences(matches.value_of("INPUT_DATA").unwrap());

    save_to_file(matches.value_of("OUTPUT_FILE").unwrap(), get_ngram(&text, 3));
}