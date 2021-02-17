extern crate ambiguity_stats;
extern crate clap;
extern crate conllx;

use clap::{App, Arg};
use std::collections::HashMap;

use ambiguity_stats::*;

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get statistics of ambiguities occurring in parser data.")
        .arg(
            Arg::with_name("INPUT_GOLD")
                .help("Sets the gold data file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("INPUT_PARSED")
                .help("Sets the parser data file to use")
                .required(true)
                .index(2),
        )
        .get_matches();

    let file_gold = matches.value_of("INPUT_GOLD").unwrap();
    let file_parsed = matches.value_of("INPUT_PARSED").unwrap();
    let (text_gold, text_parsed) = read_gng_data(file_gold, file_parsed);

    let mut preps: HashMap<String, Vec<usize>> = HashMap::new();
    prep_errs_ud(&mut preps, &text_gold, &text_parsed, &"obl", false, false);

    let mut prep_las = HashMap::new();
    for (key, val) in preps.iter() {
        if val[0] > 10 {
            if val[1] == 0 {
                prep_las.entry(key).or_insert(1.0);
            } else {
                prep_las.entry(key).or_insert(1.00 - (val[1] as f32 / val[0] as f32));
            }
        }
    }

    let mut prep_las_vec: Vec<_> = prep_las.iter().collect();
    prep_las_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let max = if prep_las_vec.len() > 20 {
        20
    } else {
        prep_las_vec.len()
    };

    for i in 0..max {
        println!("{:?}", prep_las_vec[i]);
    }

    /*
    let (precision, recall, f1_score) = prec_rec_f1(&text_gold, &text_parsed, true,pp_ud_acc_comps);
    println!(
        "Filename: {:?}\nPrecision: {:?}\nRecall: {:?}\nF1 score: {:?}",
        file_parsed, precision, recall, f1_score
    );

    // Get error rates for ambiguous structures
    let mut n_ambigs = 0;
    let mut n_ambigs_errs = 0;

    for (sent_gold, sent_parsed) in text_gold.iter().zip(text_parsed.iter()) {
        let (n_ambigs_sent, n_ambigs_errs_sent) =
            get_ambiguity_counts(&sent_gold, &sent_parsed, true, pp_gng_ambigs_ud); //inversion_ambigs_ud
        n_ambigs += n_ambigs_sent;
        n_ambigs_errs += n_ambigs_errs_sent;
    }

    let acc = (n_ambigs_errs as f32) / ((n_ambigs as f32) / 100.0);
    println!(
        "Filename: {:?}\n# Overall count: {:?}\n# errors: {:?}\n% erroneous: {:?}",
        file_parsed, n_ambigs, n_ambigs_errs, acc
    );

    // Check which errors are made by both model a and model b
    let err_sents_a = read_err_sents(file_gold).expect("Cannot read error input (model a)");
    let err_sents_b = read_err_sents(file_gold).expect("Cannot read error input (model b)");
    let matching_err_sents = matching_sents(err_sents_a, err_sents_b);
    for sent in matching_err_sents.iter() {
        println!("{}", sent);
    }
    */

}