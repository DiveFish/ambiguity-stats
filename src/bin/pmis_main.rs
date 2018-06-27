extern crate clap;
extern crate ambiguity_stats;
extern crate conllx;

use clap::{Arg, App};
use conllx::Token;
use ambiguity_stats::{read_sentences, get_all_files, get_ngram, ngrams_to_file,
                      pmi_to_file, read_pmi_file};

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

    pmi_to_file(read_pmi_file("/Users/patricia/RustProjects/results/taz/2018.06/pmi-2/OBJP_PN.txt", 2),
                "/Users/patricia/RustProjects/results/taz/2018.06/pmi-2/OBJP_PN-sorted.txt");

}

fn retrieve_pmis(files: Vec<String>, filename_template: &str) {
    for file in &files {
        ngrams_to_file(filename_template,
                       get_ngram(& read_sentences(file), 2));
        println!("Done with file {}", file)
    }
}

fn process_pmi_files() {

}