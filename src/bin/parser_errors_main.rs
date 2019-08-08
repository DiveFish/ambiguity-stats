extern crate ambiguity_stats;
extern crate clap;
extern crate conllx;

use ambiguity_stats::*;
use clap::{App, Arg};
use conllx::Token;
use std::collections::HashMap;

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
    errors(&golddata, &parserdata);
}

pub fn errors(golddata: &[Vec<Token>], parserdata: &[Vec<Token>]) {
    //Check if &[[Token]] works

    // Dependency labels
    let labels = [
        "-PUNCT-",
        "-UNKNOWN-",
        "ADV",
        "APP",
        "ATTR",
        "AUX",
        "AVZ",
        "CJ",
        "DET",
        "EXPL",
        "GMOD",
        "gmod-app",
        "GRAD",
        "KOM",
        "KON",
        "KONJ",
        "koord",
        "NEB",
        "OBJA",
        "OBJC",
        "OBJD",
        "OBJG",
        "OBJI",
        "OBJP",
        "PAR",
        "PART",
        "PN",
        "PP",
        "PRED",
        "REL",
        "ROOT",
        "S",
        "SUBJ",
        "SUBJC",
        "ZEIT",
    ];

    //println!("Label -- head-label errors -- head errors -- label errors");
    for label in labels.iter() {
        let mut all_attachments = 0;
        let mut all_combined_errors = 0;
        let mut all_head_errors = 0;
        let mut all_label_errors = 0;
        let mut all_wrong_labels: HashMap<String, usize> = HashMap::new();
        let mut token_cnt = 0;
        let mut sent_cnt = 0;

        for (gold_sent, parser_sent) in golddata.iter().zip(parserdata.iter()) {
            let (attachments, combined_errors, head_errors, label_errors, wrong_labels) =
                get_errors_by_labels(&label, gold_sent, parser_sent);
            all_attachments += attachments;
            all_combined_errors += combined_errors;
            all_head_errors += head_errors;
            all_label_errors += label_errors;
            for (label, freq) in wrong_labels.iter() {
                *all_wrong_labels.entry(label.clone()).or_insert(0) += freq;
            }
            token_cnt += gold_sent.len();
            sent_cnt += 1;
        }

        let las = (1.0
            - (all_combined_errors + all_head_errors + all_label_errors) as f32
                / all_attachments as f32)
            * 100.0;
        let uas = (1.0 - ((all_combined_errors + all_head_errors) as f32 / all_attachments as f32))
            * 100.0;
        //eprintln!("{:?}\n{:?}", las, uas);

        let error_sum = all_combined_errors + all_head_errors + all_label_errors;

        println!(
            "{}\t{:?}\n\t{:?}\n\t{:?}",
            label, all_combined_errors, all_head_errors, all_label_errors
        );
        //println!("# sents: {}, # of tokens: {}", sent_cnt, token_cnt);

        let mut wrong_label_vec: Vec<_> = all_wrong_labels.iter().collect();
        wrong_label_vec.sort_by(|a, b| b.1.cmp(&a.1));

        let mut i: usize = 0;
        print!("\t");
        while i < 5 && wrong_label_vec.len() > i {
            print!("{:?};", wrong_label_vec[i]);
            i += 1;
        }
        println!();

        // Get total number of errors per label
        /*
        let mut count: usize = 0;
        for (_, freq) in wrong_label_vec.iter() {
            count += *freq;
        }
        println!("{}", count);
        */
    }
}
