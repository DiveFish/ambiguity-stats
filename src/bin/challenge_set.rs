extern crate ambiguity_stats;
extern crate clap;

use ambiguity_stats::*;
use clap::{App, Arg};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

pub fn main() {

    let matches = App::new("ambiguity-stats")
        .version("1.0")
        .author("DiveFish")
        .about("Get statistics of inversion occurring in parser data.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = matches.value_of("INPUT").unwrap();
    let triples = extract_triples(&file);
    let templates = generate_templates();
    sentence_generator(&triples, &templates);
}

fn extract_triples(file: &str) -> Vec<Vec<String>> {
    let mut triples = Vec::new();

    let f = File::open(&file).expect("Could not open file.");
    for l in BufReader::new(f).lines() {
        let l = l.unwrap();
        let line = l.split(" ").collect::<Vec<_>>();
        triples.push(vec![line[0].to_owned(), line[1].to_owned(), line[2].to_owned()]);
    }
    triples
}

fn generate_templates() -> Vec<Vec<String>> {
    let mut templates = Vec::new();
    let s = "S".to_owned();
    let v = "V".to_owned();
    let o = "O".to_owned();
    templates.push(vec![s.clone(), v.clone(), o.clone()]);
    templates.push(vec![o.clone(), v.clone(), s.clone()]);
    templates.push(vec![v.clone(), s.clone(), o.clone()]);
    templates.push(vec![v.clone(), o.clone(), s.clone()]);
    templates.push(vec!["Weil".to_owned(), s.clone(), o.clone(), v.clone()]);
    templates.push(vec!["Weil".to_owned(), o.clone(), s.clone(), v.clone()]);
    templates
}