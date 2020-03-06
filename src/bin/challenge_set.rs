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
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file to use")
                .required(false)
                .index(2),
        )
        .get_matches();

    let intput_file = matches.value_of("INPUT").unwrap();

    let triples = extract_triples(&intput_file);
    let (templates, templates_aux) = generate_templates();
    sentence_generator(&triples, &templates, &templates_aux);
}

fn extract_triples(file: &str) -> Vec<Vec<String>> {
    let mut triples = Vec::new();

    let f = File::open(&file).expect("Could not open file.");
    for l in BufReader::new(f).lines() {
        let mut l = l.unwrap();
        l = l.trim().to_string();
        let line = l.split("\t").collect::<Vec<_>>();
        // Standard order: S V O (VMOD) when input is S V O or S VMOD O V
        if line.len() == 5 {
            triples.push(vec![line[0].to_owned(), line[1].to_owned(), line[4].to_owned(), line[3].to_owned(), line[2].to_owned()]);
        } else if line.len() == 4 {
            triples.push(vec![line[0].to_owned(), line[1].to_owned(), line[2].to_owned(), line[3].to_owned()]);
        } else {
            eprintln!("Triple length {} not supported.", line.len());
            eprintln!("{:?}", line);
        }
    }
    triples
}

fn generate_templates() -> (Vec<Vec<String>>, Vec<Vec<String>>)  {
    let mut templates = Vec::new();
    let s = "S".to_owned();
    let v = "V".to_owned();
    let o = "O".to_owned();
    templates.push(vec![s.clone(), v.clone(), o.clone()]);
    templates.push(vec![o.clone(), v.clone(), s.clone()]);
    templates.push(vec!["Deshalb".to_owned(), v.clone(), s.clone(), o.clone()]);
    templates.push(vec!["Deshalb".to_owned(), v.clone(), o.clone(), s.clone()]);
    templates.push(vec!["Weil".to_owned(), s.clone(), o.clone(), v.clone()]);
    templates.push(vec!["Weil".to_owned(), o.clone(), s.clone(), v.clone()]);

    let mut templates_aux = Vec::new();
    let s = "S".to_owned();
    let v_aux = "VAUX".to_owned();
    let v = "V".to_owned();
    let o = "O".to_owned();
    templates_aux.push(vec![s.clone(), v_aux.clone(), o.clone(), v.clone()]);
    templates_aux.push(vec![o.clone(), v_aux.clone(), s.clone(), v.clone()]);
    templates_aux.push(vec!["Deshalb".to_owned(), v_aux.clone(), s.clone(), o.clone(), v.clone()]);
    templates_aux.push(vec!["Deshalb".to_owned(), v_aux.clone(), o.clone(), s.clone(), v.clone()]);
    templates_aux.push(vec!["Weil".to_owned(), s.clone(), o.clone(), v.clone(), v_aux.clone()]);
    templates_aux.push(vec!["Weil".to_owned(), o.clone(), s.clone(), v.clone(), v_aux.clone()]);

    (templates, templates_aux)
}