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
    let (golddata, parserdata) = read_gng_data(golddatafile, parserdatafile);

    // Correct subject/object fit indicated by 1, swap by 0, other label given

    let parser = matches.value_of("PARSER").unwrap();

    println!("Parser\tSent\tS fit\tO fit\tS gold\tO gold\tS parser\tO parser\tOrder\tProp1\tProp2");
    let las_feats = las_feats(&golddata, &parserdata, &parser, false); // ... no_amb
    eprintln!("In {}\nSubject/object accuracy: {}\n", parserdatafile, las_feats);
    //prop_scores(&golddata, &parserdata, &parser, false, false); // ... no_amb, sent_las

    //let num_features = matches.value_of("FEATURES").unwrap().parse::<usize>().unwrap();
    //prop_scores_combined(&golddata, &parserdata, &parser, num_features);

    // Attachment scores
    /*
    let (las, uas) = las_uas(&golddata, &parserdata);
    println!("LAS {:?} and UAS {:?}", las, uas);

    let per_sent_las = per_sent_uas(&golddata, &parserdata);
    for las in per_sent_las {
        println!("{}", las);
    }
    */
}
