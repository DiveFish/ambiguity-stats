extern crate ambiguity_stats;
extern crate clap;
extern crate conllx;

use ambiguity_stats::*;
use clap::{App, Arg};
use std::fs::File;
use conllx::{WriteSentence, Writer};

pub fn main() {
    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get statistics of ambguities occurring in parser data.")
        .arg(
            Arg::with_name("TDZ")
                .help("Sets the TüBa-D/Z directory to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("TAZ")
                .help("Sets the taz directory to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output directory to use")
                .required(true)
                .index(3),
        )
        .get_matches();

    let tdz_dir = matches.value_of("TDZ").unwrap();
    let tdz_files = get_all_files(tdz_dir);
    let mut tdz_sents = Vec::new();
    for tdz_file in tdz_files {
        let sents = read_data(&tdz_file);
        for sent in sents {
            let mut sent_toks = Vec::with_capacity(sent.len());
            for tok in sent {
                sent_toks.push(tok.form().to_string());
            }
            tdz_sents.push(sent_toks);
        }
    }

    let output_filename = matches.value_of("OUTPUT").unwrap();
    let mut output_file = File::create(&output_filename).expect("Could not create file");

    let taz_dir = matches.value_of("TAZ").unwrap();
    let taz_files = get_all_files(taz_dir);

    let mut writer = Writer::new(Box::new(output_file));

    let mut i = 0;
    for taz_file in taz_files {
        let sents = read_data(&taz_file);
        for sent in sents {
            let mut sent_toks = Vec::with_capacity(sent.len());
            for tok in &sent {
                sent_toks.push(tok.form().to_string());
            }
            if !tdz_sents.contains(&sent_toks) {
                writer.write_sentence(&sent).unwrap();
            }
        }
        i += 1;
        eprintln!("Done with file {}: {}", i, taz_file);
    }

}
