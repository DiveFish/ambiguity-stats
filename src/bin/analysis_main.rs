extern crate clap;
extern crate ambiguity_stats;
extern crate conllx;

use clap::{Arg, App};
use ambiguity_stats::*;
use std::collections::HashMap;
use conllx::Token;

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

    /*
    for gold_sent in &golddata {
        for i in 0..gold_sent.len() {
            let gold_token = &gold_sent[i];
            print!("{} ", gold_token.form());
        }
        println!();
    }
    */
    //get_topofields(golddata.as_slice());
    for i in 0..golddata.len() {
        get_ambiguity_counts(&golddata[i], &parserdata[i], n_pp_objps_ambig);
    }
}

/*
pub fn errors(golddata: &[Vec<Token>], parserdata: &[Vec<Token>]) {   //Check if &[[Token]] works


    // Dependency labels
    let labels = ["ADV", "APP", "ATTR", "AUX", "AVZ", "CJ", "DET", "ETH", "EXPL", "GMOD",
        "GRAD", "KOM", "KON", "KONJ", "NEB", "NP2", "OBJA", "OBJA2", "OBJC", "OBJD", "OBJG", "OBJI",
        "OBJP", "PAR", "PART", "PN", "PP", "PRED", "REL", "S", "SUBJ", "SUBJC", "VOK", "ZEIT",
        "-PUNCT-", "-UNKNOWN-", "_", "ROOT", "gmod-app", "koord"];

    /*
    // PoS tags
    let labels = ["ADJA", "ADJD", "ADV", "APPR", "APPRART", "APPO", "APZR", "ART", "CARD",
        "FM", "ITJ", "KOUI", "KOUS", "KON", "KOKOM", "NN", "NE", "PDS", "PDAT", "PIS", "PIAT",
        "PIDAT", "PPER", "PPOSS", "PPOSAT", "PRELS", "PRELAT", "PRF", "PROP", "PWS", "PWAT", "PWAV", "PAV",
        "PTKZU", "PTKNEG", "PTKVZ", "PTKANT", "PTKA", "TRUNC", "VVFIN", "VVIMP", "VVINF", "VVIZU",
        "VVPP", "VAFIN", "VAIMP", "VAINF", "VAPP", "VMFIN", "VMINF", "VMPP", "XY", "$,", "$.", "$("];
    */

    let attachments = 178430;

    for label in labels.iter() {

        let mut idx = 0;
        let mut all_attachments = 0;
        let mut all_combined_errors = 0;
        let mut all_head_errors = 0;
        let mut all_label_errors = 0;
        let mut all_wrong_labels: HashMap<String, usize> = HashMap::new();

        for sent in &golddata {

            //let (overall_count, error) = get_ambiguity_counts(&sent, &parserdata.get(idx).expect("No sentence"), get_all_pp_ambigs);
            let (attachments, combined_errors, head_errors, label_errors, wrong_labels) = get_errors_by_labels(&label, &sent, &parserdata.get(idx).expect("No sentence"));

            all_attachments += attachments;
            all_combined_errors += combined_errors;
            all_head_errors += head_errors;
            all_label_errors += label_errors;
            idx += 1;

            for (label, freq) in wrong_labels.iter() {
                *all_wrong_labels.entry(label.clone()).or_insert(0) += freq;
            }
        }
        let las = 1.0 - (all_combined_errors + all_head_errors + all_label_errors) as f32 / all_attachments as f32;
        let uas = 1.0 - ((all_combined_errors + all_head_errors) as f32 / all_attachments as f32);
        let all_errors = all_combined_errors + all_head_errors + all_label_errors;
        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", label, all_attachments, all_combined_errors, all_head_errors, all_label_errors, all_errors, las, uas);
        //print!("{}", label);
        let mut wrong_label_vec: Vec<_> = all_wrong_labels.iter().collect();
        wrong_label_vec.sort_by(|a, b| b.1.cmp(&a.1));
        let mut i: usize = 0;
        while i < 10 && wrong_label_vec.len() > i {
            //print!("\t{:?} ", wrong_label_vec[i]);
            i += 1;
        }
        let mut count: usize = 0;
        for (_, freq) in wrong_label_vec.iter() {
            count += *freq;
        }
        //println!("\n{}", count);
        //println!();
    }
}
*/