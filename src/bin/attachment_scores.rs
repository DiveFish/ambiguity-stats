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
        .arg(
            Arg::with_name("PARSER")
                .help("Sets the parser model name to use")
                .required(false)
                .index(3),
        )
        .arg(
            Arg::with_name("FEATURES")
                .help("Sets the number of features to combine")
                .required(false)
                .index(4),
        )
        .get_matches();

    let golddatafile = matches.value_of("INPUT_GOLD").unwrap();
    let parserdatafile = matches.value_of("INPUT_NONGOLD").unwrap();
    let parser = matches.value_of("PARSER").unwrap();
    let (golddata, parserdata) = read_gng_data(golddatafile, parserdatafile);

    // Correct subject/object fit indicated by 1, swap by 0, other label given
    /*
    println!("Parser\tSent\tS fit\tO fit\tS gold\tO gold\tS parser\tO parser\tOrder\tProp1\tProp2");
    let las_no_heads = las_no_heads_feats(&parserdata, &golddata, &parser);
    eprintln!("In {}\nSubject/object accuracy: {}\n", parserdatafile, las_no_heads);
    prop_scores(&parserdata, &golddata, &parser);
    */
    let num_features = matches.value_of("FEATURES").unwrap().parse::<usize>().unwrap();
    prop_scores_combined(&parserdata, &golddata, &parser, num_features);

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
