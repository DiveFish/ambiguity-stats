extern crate clap;
extern crate conllx;

extern crate ambiguity_stats;

use clap::{App, Arg};
use conllx::Token;
use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

use ambiguity_stats::*;

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
                .required(false)
                .index(2),
        )
        .get_matches();

    let golddatafile = matches.value_of("INPUT_GOLD").unwrap();
    let parserdatafile = matches.value_of("INPUT_NONGOLD").unwrap();
    let (golddata, parserdata) = read_gng_data(golddatafile, parserdatafile);
    /*
    get_topofields(&golddata, true);

    let mut n_sent = 0;
    let mut n_token = 0;
    for gold_sent in &golddata {
        n_sent += 1;
        for _ in 0..gold_sent.len() {
            n_token += 1;
            //let gold_token = &gold_sent[i];
            //print!("{} ", gold_token.form());
        }
        //println!();
    }
    println!("#sents {:?}, #tokens {:?}", n_sent, n_token);


    let mut occurrences_total = 0;
    let mut errors_total = 0;

    for i in 0..golddata.len() {

        //let (overall_count, error) = get_ambiguity_counts(&sent, &parserdata.get(idx).expect("No sentence"), get_all_pp_ambigs);
        let (overall_occurrences, errors) = get_ambiguity_counts(&golddata[i], &parserdata[i], n_phrasalv_prep_ambig);

        occurrences_total += overall_occurrences;
        errors_total += errors;
    }
    println!("Ambiguity count: {},\terrors {}", occurrences_total, errors_total);

    //Get number of verbal, nominal and other heads per preposition in a UD treebank
    let mut preps: HashMap<String, Vec<usize>> = HashMap::new();
    for i in 0..golddata.len() {
        pp_preps_ud(&mut preps, &golddata[i]);
    }
    println!("Preposition; Frequency; Verbal heads; Nominal heads; Other heads");
    for (key, value) in preps.iter() {
        println!("{}; {}; {}; {}; {}", key, value[0], value[1], value[2], value[3]);
    }
    */


    // Get the distribution of PoS tags for the token following the preposition in the PP
    let mut prep_objs: HashMap<String, usize> = HashMap::new();
    for i in 0..golddata.len() {
        pp_objs_ud(&mut prep_objs, &golddata[i]);
    }
    println!("Preposition object; Frequency");
    for (key, value) in prep_objs.iter() {
        println!("{}; {}", key, value);
    }

    /*
    //Get number of errors, number of verbal, nominal and other heads per preposition in an HDT treebank
    for i in 0..golddata.len() {
        pp_preps(&mut preps, &golddata[i], &parserdata[i]);
    }
    // println!("Preposition; Frequency; Errors; Verbal heads; Nominal heads; Other heads");
    for (key, value) in preps.iter() {
        println!("{}; {}; {}; {}; {}; {}", key, value[0], value[1], value[2], value[3], value[4]);
    }

    // Get verbs involved in inversion along with the event frequency
    let mut inv_verbs: HashMap<String, usize> = HashMap::new();
    for i in 0..golddata.len() {
        inversion_verbs(&mut inv_verbs, &golddata[i], false);
    }

    for (key, val) in inv_verbs.iter() {
        if val > &1 {
            //println!("{} {}", key, val);
        }
    }
    */
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
