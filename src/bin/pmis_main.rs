extern crate ambiguity_stats;
extern crate clap;
extern crate conllx;

use ambiguity_stats::*;
use clap::{App, Arg};

/// The compute-mi program takes non-deduplicated lists of ngrams and calculates the PMIs for
/// all ngrams in the list. The ngram lists are generated in 'ambiguity-stats' before they
/// are passed to compute-mi.
pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get PMI statistics of parser data.")
        .arg(
            Arg::with_name("INPUT_DIRECTORY")
                .help("Sets the data file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT_DIRECTORY")
                .help("Sets the output file name")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::with_name("NGRAM_SIZE")
                .help("Sets the ngram size")
                .required(false)
                .index(3),
        )
        .get_matches();

    /*
    let input_files = get_all_files(matches.value_of("INPUT_DIRECTORY").unwrap());

    for file in input_files {
        let content = read_sentences(&file);
        let mut n_sentences = content.len();
        let mut n_tokens = 0;
        for sentence in content {
            n_tokens += sentence.len();
        }
        println!("Sentences {:?} - tokens {:?}", n_sentences, n_tokens);
    }

    //collect_ngrams(input_files, output_file_template, matches.value_of("NGRAM_SIZE").unwrap().parse::<usize>().unwrap());

    //collect_ngram_trees(files, matches.value_of("NGRAM_SIZE").unwrap().parse::<usize>().unwrap());

    // Sort ngrams by PMI
    // Make sure pmis of ngrams have been retrieved via mi program before sorting the ngram-pmi lists!

    //sort_pmi_file("/Users/patricia/RustProjects/results/taz/2018.07/pmi-ranks-2rel/pmi_OBJP-PN.txt", 3,
    //            "/Users/patricia/RustProjects/results/taz/2018.07/pmi-ranks-2rel/pmi_OBJP-PN-sorted.txt").unwrap();
    */

    let input_file = matches.value_of("INPUT_DIRECTORY").unwrap();

    let focus_words = vec![
        "vereinigt".to_string(),
        "Sansibar".to_string(),
    ];

    let context_words = vec![
        "mit".to_string(),
        "mit".to_string(),
    ];

    let deprels = vec![
        "OBJP".to_string(),
        "OBJP".to_string(),
    ];

    get_pmi(&focus_words, &context_words, &deprels, input_file);
}

fn collect_ngrams(files: Vec<String>, filename_template: &str, ngram_size: usize) {
    for file in &files {
        ngrams_to_file(
            filename_template,
            readers::get_deprel_bigrams(&read_sentences(file)),
        )
        .unwrap();
        println!("Done with file {}", file)
    }
}
