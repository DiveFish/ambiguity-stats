extern crate clap;
extern crate ambiguity_stats;

use clap::{Arg, App};
use ambiguity_stats::*;

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
    let mut attachments: usize = 0;
    let mut errors: usize = 0;
    let labels = ["OBJP", "PP"];
	for sent in &golddata {
        //let (overall_count, error) = get_ambiguity_counts(&sent, &parserdata.get(idx).expect("No sentence"), get_all_pp_ambigs);
        get_errors_by_deprels(&labels, &mut attachments, &mut errors, true, &sent, &parserdata.get(idx).expect("No sentence"));
        idx += 1;
    }
    println!("Number of occurrences: {:?}", attachments);
    println!("Number of errors: {:?}", errors);
    let ratio = errors as f32 / (attachments as f32/100.0);
    println!("Error ratio: {:?}", ratio);
	println!("Done with analysis");

    let mut all_attachments= 0;
    let mut all_combined_errors= 0;
    let mut all_label_errors= 0;
    let mut all_head_errors= 0;
    let mut idx = 0;
    for sent in &golddata {
        let (attachments, combined_errors, label_errors, head_errors) = get_las(&sent, &parserdata.get(idx).expect("No sentence"));
        all_attachments += attachments;
        all_combined_errors += combined_errors;
        all_label_errors += label_errors;
        all_head_errors += head_errors;
        idx += 1;
    }
    let las = (all_combined_errors + all_head_errors + all_label_errors) as f32 / all_attachments as f32;
    println!("\nAll attachments {:?}", all_attachments);
    println!("# of head+label errors {:?}", all_combined_errors);
    println!("Only label errors {:?}", all_label_errors);
    println!("Only head errors {:?}", all_head_errors);
    println!("LAS {:?}", las);
    let uas = all_head_errors as f32 / all_attachments as f32;
    println!("UAS {:?}", uas);
}