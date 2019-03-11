extern crate ambiguity_stats;
extern crate clap;
extern crate conllx;

use ambiguity_stats::{
    get_all_files, get_deprel_bigrams, get_deprel_ngrams, get_graph_ngrams, get_ngrams,
    get_tree_ngrams, ngrams_to_file, read_sentences, sort_pmi_file,
};
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
        ).arg(
            Arg::with_name("OUTPUT_DIRECTORY")
                .help("Sets the output file name")
                .required(false)
                .index(2),
        ).arg(
            Arg::with_name("NGRAM_SIZE")
                .help("Sets the ngram size")
                .required(false)
                .index(3),
        ).get_matches();

    let input_files = get_all_files(matches.value_of("INPUT_DIRECTORY").unwrap());
    //let output_file_template = matches.value_of("OUTPUT_DIRECTORY").unwrap();
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
}

#[allow(dead_code)]
fn collect_ngram_trees(files: Vec<String>, ngram_size: usize) {
    for file in &files {
        get_graph_ngrams(&read_sentences(file), ngram_size, "SUBJ", "OBJA");
    }
}

fn collect_ngrams(files: Vec<String>, filename_template: &str, ngram_size: usize) {
    for file in &files {
        ngrams_to_file(filename_template, get_deprel_bigrams(&read_sentences(file))).unwrap();
        println!("Done with file {}", file)
    }
}
