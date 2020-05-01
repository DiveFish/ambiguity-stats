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
                .required(true)
                .index(2),
        )
        .get_matches();

    let input = matches.value_of("INPUT_GOLD").unwrap();
    let _output = matches.value_of("INPUT_NONGOLD").unwrap();

    let mut freq_map: HashMap<String, usize> = HashMap::new();
    let files = get_all_files(input);
    for file in files {
        let sents = read_data(&file);
        for sent in sents {
            for token in sent {
                let entry = freq_map.entry(token.form().to_string()).or_insert(0);
                *entry += 1;
            }
        }
    }

    let mut freq_vec: Vec<_> = freq_map.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

    for i in 0..100 {
        println!("{:?}", freq_vec[i]);
    }
    /*
    let golddatafile = matches.value_of("INPUT_GOLD").unwrap();
    let parserdatafile = matches.value_of("INPUT_NONGOLD").unwrap();
    let (golddata, parserdata) = read_gng_data(golddatafile, parserdatafile);

    let mut n_sent = 0;
    let mut n_token = 0;
    for gold_sent in &golddata {
        n_sent += 1;
        for _ in 0..gold_sent.len() {
            n_token += 1;
        }
    }
    println!("#sents {:?}, #tokens {:?}", n_sent, n_token);

    //get_topofields(golddata.as_slice());

    let mut occurrences_total = 0;
    let mut errors_total = 0;

    for i in 0..golddata.len() {

        //let (overall_count, error) = get_ambiguity_counts(&sent, &parserdata.get(idx).expect("No sentence"), get_all_pp_ambigs);
        let (overall_occurrences, errors) = get_ambiguity_counts(&golddata[i], &parserdata[i], n_phrasalv_prep_ambig);

        occurrences_total += overall_occurrences;
        errors_total += errors;
    }
    println!("Ambiguity count: {},\terrors {}", occurrences_total, errors_total);


    //Get number of errors, number of verbal, nominal and other heads per preposition
    let mut preps: HashMap<String, Vec<usize>> = HashMap::new();
    for i in 0..golddata.len() {
        pp_preps_ud(&mut preps, &golddata[i]);
        //pp_preps(&mut preps, &golddata[i], &parserdata[i]);
    }

    println!("Preposition; Frequency; Errors; Verbal heads; Nominal heads; Other heads");
    for (key, value) in preps.iter() {
        println!("{}; {}; {}; {}; {}; {}", key, value[0], value[1], value[2], value[3], value[4]);
    }

    */
}