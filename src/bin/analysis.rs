extern crate clap;
extern crate ambiguity_stats;

use clap::{Arg, App};
use ambiguity_stats::{n_pp_attachments, n_pp_objps, n_obj_frontings};
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
            .help("Sets the non-gold parser data file to use")
            .required(true)
            .index(2))
        .get_matches();

    let golddatafile = matches.value_of("INPUT_GOLD").unwrap();
    let nongolddatafile = matches.value_of("INPUT_NONGOLD").unwrap();
	let (golddata, nongolddata) = read_gng_data(golddatafile, nongolddatafile);

    let mut idx = 0;
    let mut overall_counts = 0;
    let mut errors = 0;
	for sent in &golddata {
        let (overall_count, error) = n_obj_frontings(&sent, &nongolddata.get(idx).expect("No sentence"));
        overall_counts += overall_count;
        errors += error;
        idx += 1;
    }

    println!("Number of fronted objects: {:?}", overall_counts);
    println!("Number of SUBJ-OBJ confusions: {:?}", errors);
    let ratio = errors as f32 / (overall_counts/100) as f32;
    println!("Error ratio: {:?}", ratio);
	println!("Done with analysis");
}