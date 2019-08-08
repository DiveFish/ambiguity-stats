extern crate ambiguity_stats;
extern crate clap;

use ambiguity_stats::*;
use clap::{App, Arg};
use std::fs::File;
use std::io::Write;
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
        )
        .arg(
            Arg::with_name("INPUT_DIR_2")
                .help("Sets the input directory of the second file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("OUTPUT_FILE")
                .help("Sets the name of the single output file")
                .required(false)
                .index(3),
        )
        .get_matches();

    let input_dir1 = matches.value_of("INPUT_DIR").unwrap();
    let input_dir2 = matches.value_of("INPUT_DIR_2").unwrap();
    eprintln!(">> Remember to \n- REMOVE old single-pmi files (content will be appended)\n- Change BOTH ends_with(pnsc/nsc) in compute_mi_to_dpar_pmis method accordingly \n<<");
    bigram_pmi_to_dpar_pmis(Path::new(input_dir1), input_dir2, "psc");

    /*
    let output_filename = matches.value_of("OUTPUT_FILE").unwrap();
    let assoc_map = combine_tri_bigram_files(
        File::open(input_dir1).expect("Cannot read input file 1"),
        File::open(input_dir2).expect("Cannot read input file 1"),
    )
    .expect("Could not create association map");

    println!("Creating new file \"{}\"", output_filename);
    let mut file = File::create(&output_filename).expect("Could not create output file");
    for ((w1, w2, w3, d1, d2), association) in &assoc_map {
        writeln!(file, "{}\t{}\t{}\t{}\t{}\t{}", w1, w2, w3, d1, d2, association);
    }
    */
}
