extern crate ambiguity_stats;
extern crate clap;
extern crate conllx;

use ambiguity_stats::*;
use clap::{App, Arg};

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get statistics of ambguities occurring in parser data.")
        .arg(
            Arg::with_name("INPUT_GOLD")
                .help("Sets the gold data file to use")
                .required(true)
                .index(1),
        ).arg(
            Arg::with_name("INPUT_NONGOLD")
                .help("Sets the parser data file to use")
                .required(true)
                .index(2),
        ).get_matches();

    let golddatafile = matches.value_of("INPUT_GOLD").unwrap();
    let parserdatafile = matches.value_of("INPUT_NONGOLD").unwrap();
    let (golddata, parserdata) = read_gng_data(golddatafile, parserdatafile);

    let mut n_pps = 0;
    let mut n_pp_errs = 0;

    for (gold_sent, parser_sent) in golddata.iter().zip(parserdata.iter()) {
        let (n_pps_sent, n_pp_errs_sent) =
            get_ambiguity_counts(&gold_sent, &parser_sent, get_all_subj_obj_ambigs);
        n_pps += n_pps_sent;
        n_pp_errs += n_pp_errs_sent;
    }
    let acc = (n_pp_errs as f32) / ((n_pps as f32) / 100.0);
    println!(
        "Filename: {:?}\n# Overall count: {:?}\n# errors: {:?}\n% erroneous: {:?}",
        parserdatafile, n_pps, n_pp_errs, acc
    );
}
