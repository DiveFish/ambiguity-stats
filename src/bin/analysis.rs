extern crate clap;
extern crate ambiguity_stats;

use clap::{Arg, App};
use ambiguity_stats::*;
use ambiguity_stats::read_gng_data;

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get statistics of ambguities occurring in parser data.")
        .arg(Arg::with_name("INPUT_GOLD")
            .help("Sets the gold data file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("INPUT_NONGOLD")
            .help("Sets the parser data file to use")
            .required(true)
            .index(2))
        .get_matches();

    let golddatafile = matches.value_of("INPUT_GOLD").unwrap();
    let parserdatafile = matches.value_of("INPUT_NONGOLD").unwrap();
	let (golddata, parserdata) = read_gng_data(golddatafile, parserdatafile);

    let mut idx = 0;
    let mut overall_counts = 0;
    let mut errors = 0;
	for sent in &golddata {
        let (overall_count, error) = get_ambiguity_counts(&sent, &parserdata.get(idx).expect("No sentence"), n_adj_adv_ambig);
        overall_counts += overall_count;
        errors += error;
        idx += 1;
    }
    println!("Number of occurrences: {:?}", overall_counts);
    println!("Number of errors: {:?}", errors);
    let ratio = errors as f32 / (overall_counts as f32/100.0);
    println!("Error ratio: {:?}", ratio);
	println!("Done with analysis");

    let mut all_correct_labels= 0;
    let mut all_correct_heads= 0;
    let mut all_label_errors= 0;
    let mut all_head_errors= 0;
    idx = 0;
    for sent in &golddata {
        let (correct_labels, correct_heads, label_errors, head_errors) = get_error_counts(&sent, &parserdata.get(idx).expect("No sentence"));
        all_correct_labels += correct_labels;
        all_correct_heads += correct_heads;
        all_label_errors += label_errors;
        all_head_errors += head_errors;
        idx += 1;
    }
    println!("\nCorrect labels {:?}", all_correct_labels);
    println!("Correct heads {:?}", all_correct_heads);
    println!("Label errors {:?}", all_label_errors);
    println!("Head errors {:?}", all_head_errors);
}