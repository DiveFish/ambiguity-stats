extern crate ambiguity_stats;
extern crate clap;

use ambiguity_stats::*;
use clap::{App, Arg};
use std::path::Path;

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Convert 3-column pmi files into a single 4-column file")
        .arg(
            Arg::with_name("INPUT_DIR")
                .help("Sets the input directory of the pmi files")
                .required(true)
                .index(1),
        ).arg(
        Arg::with_name("OUTPUT_FILENAME")
            .help("Sets the name of the single output file")
            .required(true)
            .index(2),
    ).get_matches();


    let input_dir = matches.value_of("INPUT_DIR").unwrap();
    let output_filename = matches.value_of("OUTPUT_FILENAME").unwrap();
    eprintln!(">> Remember to REMOVE old single-pmi files (content will be appended) <<");
    compute_mi_to_dpar_pmis(Path::new(input_dir), output_filename);
}