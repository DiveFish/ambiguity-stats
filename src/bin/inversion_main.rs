extern crate ambiguity_stats;
extern crate clap;
extern crate conllx;

use std::collections::HashMap;

use ambiguity_stats::*;
use clap::{App, Arg};

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get statistics of inversion occurring in parser data.")
        .arg(
            Arg::with_name("INPUT_DIR")
                .help("Sets the data file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("INPUT_DIR2")
                .help("Sets the second data file to use")
                .required(false)
                .index(2),
        )
        .get_matches();

    let dir1 = matches.value_of("INPUT_DIR").unwrap();
    //let dir2 = matches.value_of("INPUT_DIR2").unwrap();
    let files = get_all_files(dir1);
    /*
    let verbs_file = "/home/patricia/Dokumente/Promotion/Ergebnisse/Inversion-verbs/lemmas-lowercased/content-verb-heads/europarl-verbs.txt";
    let gn_reflexives = get_all_files("/home/patricia/Dokumente/Promotion/Daten/GermaNet/Verbframes/reflexives");
    let gn_verb_groups = get_all_files("/home/patricia/Dokumente/Promotion/Daten/GermaNet/Verbframes/groups");
    let gn_expletives = get_all_files("/home/patricia/Dokumente/Promotion/Daten/GermaNet/Verbframes/expletives");
    gn_metadata(&gn_reflexives, &gn_verb_groups, &gn_expletives, &verbs_file);
    */
    get_svo_triplets(&files);
}

fn get_svo_triplets(files: &[String]) {
    for file in files {
        eprintln!("{}", file);
        let text = read_data(&file);
        for sent in text.iter() {
            svo_triples(sent, true);
        }
    }
}

fn get_inversion_verbs(files: &[String]) {
    let mut verb_freqs = HashMap::new();

    for file in files {
        let text = read_data(file);
        let mut idx = 0;
        for sent in text.iter() {
            let (verbs, inv_verbs) = inversion_verbs_content(sent);
            for (verb, inversion_verb) in verbs.iter().zip(inv_verbs.iter()) {
                let (freq, inv_freq) = verb_freqs.entry(verb.clone()).or_insert((0, 0));
                *freq += 1;
                if inversion_verb != "UNKNOWN" {
                    *inv_freq += 1;
                }
            }
        }
    }

    println!("Verb\tVerb frequency\tInversion frequency\tInversion ratio\tWeighted inversion ratio\tVerb length");
    for (verb, (freq, inv_freq)) in verb_freqs.iter() {
        println!("{}\t{}\t{}\t{}\t\t{}", verb, freq, inv_freq, *inv_freq as f32/ *freq as f32, verb.chars().count());
    }
}