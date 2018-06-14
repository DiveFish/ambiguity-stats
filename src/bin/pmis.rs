extern crate clap;
extern crate ambiguity_stats;

use clap::{Arg, App};
use ambiguity_stats::read_sentences;
use ambiguity_stats::get_ngram;
use ambiguity_stats::save_to_file;
use std::collections::HashMap;

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get PMI statistics of parser data.")
        .arg(Arg::with_name("INPUT_DATA")
            .help("Sets the data file to use")
            .required(true)
            .index(1))
        .get_matches();

    let datafile = matches.value_of("INPUT_GOLD").unwrap();

    let text = read_sentences(datafile);

    save_to_file("pmi-ngrams", get_ngram(&text, 2));
}