extern crate ambiguity_stats;
extern crate clap;
extern crate conllx;

use ambiguity_stats::*;

use std::io::{BufWriter, Write};

use clap::{App, Arg};
use std::fs::File;

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Compare different parser results.")
        .arg(
            Arg::with_name("INPUT_DEP")
                .help("Sets the dependency-embedding-based parser file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("INPUT_PMI")
                .help("Sets the PMI-based parser file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("INPUT_GOLD")
                .help("Sets the gold standard file")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::with_name("OUTPUT_ERRS_SHARED")
                .help("Sets the output file for shared errors")
                .required(true)
                .index(4),
        )
        .arg(
            Arg::with_name("OUTPUT_ERRS_DEP")
                .help("Sets the output file for dependency-embedding-based errors")
                .required(true)
                .index(5),
        )
        .arg(
            Arg::with_name("OUTPUT_ERRS_PMI")
                .help("Sets the output file for PMI-based errors")
                .required(true)
                .index(6),
        )
        .get_matches();

    let dep_file = matches.value_of("INPUT_DEP").unwrap();
    let pmi_file = matches.value_of("INPUT_PMI").unwrap();
    let gold_file = matches.value_of("INPUT_GOLD").unwrap();

    let dep_sents = read_sentences(dep_file);
    let pmi_sents = read_sentences(pmi_file);
    let gold_sents = read_sentences(gold_file);

    let (errs_shared, dep_errs, pmi_errs) = comp_inv_err_sents(&dep_sents, &pmi_sents, &gold_sents);

    let errs_shared_out_file = matches.value_of("OUTPUT_ERRS_SHARED").unwrap();
    let dep_errs_out_file = matches.value_of("OUTPUT_ERRS_DEP").unwrap();
    let pmi_errs_out_file = matches.value_of("OUTPUT_ERRS_PMI").unwrap();

    let mut shared_errs_writer =
        BufWriter::new(File::create(errs_shared_out_file).expect("Unable to create file"));
    let mut dep_errs_writer =
        BufWriter::new(File::create(dep_errs_out_file).expect("Unable to create file"));
    let mut pmi_errs_writer =
        BufWriter::new(File::create(pmi_errs_out_file).expect("Unable to create file"));

    for idx in 0..errs_shared.len() {
        if errs_shared[idx].len() <= 10 {
            writeln!(shared_errs_writer, "{:?}\n", errs_shared[idx]);
        }
    }

    for idx in 0..dep_errs.len() {
        if dep_errs[idx].len() <= 10 {
            writeln!(dep_errs_writer, "{:?}\n", dep_errs[idx]);
        }
    }

    for idx in 0..pmi_errs.len() {
        if pmi_errs[idx].len() <= 10 {
            writeln!(pmi_errs_writer, "{:?}\n", pmi_errs[idx]);
        }
    }
}
