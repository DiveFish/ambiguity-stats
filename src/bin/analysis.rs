extern crate clap;
extern crate ambiguity_stats;

use clap::{Arg, App};
use ambiguity_stats::n_incorrect_pp_attachments;
use ambiguity_stats::read_gng_data;

pub fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("DiveFish")
        .about("Get statistics of ambguities occurring in conll sample data.")
        .arg(Arg::with_name("INPUT_GOLD")
            .help("Sets the gold data file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("INPUT_NONGOLD")
            .help("Sets the non-gold parser data file to use")
            .required(true)
            .index(2))
        .get_matches();

    let golddatafile = matches.value_of("INPUT_GOLD").unwrap();
    let nongolddatafile = matches.value_of("INPUT_NONGOLD").unwrap();

	let (golddata, nongolddata) = read_gng_data(golddatafile, nongolddatafile);

    let mut idx = 0;
    let mut errors = 0;
	for sent in &golddata {
        errors += n_incorrect_pp_attachments(&sent, &nongolddata.get(idx).expect("No Token"));
        idx += 1;
    }

    println!("Number of errors: {:?}", errors);
	println!("Done with analysis");
}