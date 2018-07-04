extern crate clap;
extern crate ambiguity_stats;
extern crate conllx;

use clap::{Arg, App};
use ambiguity_stats::{read_sentences, get_all_files, get_ngrams, sort_pmi_file, get_deprel_ngrams,
                      get_tree_ngrams, ngrams_to_file};

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get PMI statistics of parser data.")
        .arg(Arg::with_name("INPUT_DIRECTORY")
            .help("Sets the data file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT_DIRECTORY")
            .help("Sets the output file name")
            .required(true)
            .index(2))
        .arg(Arg::with_name("NGRAM_SIZE")
            .help("Sets the ngram size")
            .required(true)
            .index(3))
        .get_matches();

    let files = get_all_files(matches.value_of("INPUT_DIRECTORY").unwrap());
    let filename_template = matches.value_of("OUTPUT_DIRECTORY").unwrap();
    //collect_ngrams(files, filename_template, matches.value_of("NGRAM_SIZE").unwrap().parse::<usize>().unwrap());

    collect_ngram_trees(files, matches.value_of("NGRAM_SIZE").unwrap().parse::<usize>().unwrap());

    // Make sure pmis of ngrams have been retrieved via mi program!
    //sort_pmi_file("/Users/patricia/RustProjects/results/taz/2018.07/pmi-ranks-2rel/pmi_OBJP-PN.txt", 3,
    //            "/Users/patricia/RustProjects/results/taz/2018.07/pmi-ranks-2rel/pmi_OBJP-PN-sorted.txt").unwrap();

}

fn collect_ngram_trees(files: Vec<String>, ngram_size: usize) {
    for file in &files {
        get_tree_ngrams(& read_sentences(file), ngram_size);
    }
}

#[allow(dead_code)]
fn collect_ngrams(files: Vec<String>, filename_template: &str, ngram_size: usize) {
    for file in &files {
        ngrams_to_file(filename_template,
                       get_deprel_ngrams(& read_sentences(file), ngram_size)).unwrap();
        println!("Done with file {}", file)
    }
}