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
        )
        .arg(
            Arg::with_name("INPUT_NONGOLD")
                .help("Sets the parser data file to use")
                .required(true)
                .index(2),
        )
        .get_matches();

    let golddatafile = matches.value_of("INPUT_GOLD").unwrap();
    let parserdatafile = matches.value_of("INPUT_NONGOLD").unwrap();
    let (golddata, parserdata) = read_gng_data(golddatafile, parserdatafile);

    let las_no_heads = las_no_heads(&parserdata, &golddata);
    // Attachment scores
    /*
    let (las, uas) = las_uas(&parserdata, &golddata);
    println!("LAS {:?} and UAS {:?}", las, uas);
    let per_sent_las = per_sent_uas(&parserdata, &golddata);
    for las in per_sent_las {
        println!("{}", las);
    }
    */
}
