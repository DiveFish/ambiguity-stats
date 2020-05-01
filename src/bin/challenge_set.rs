extern crate ambiguity_stats;
extern crate clap;

use ambiguity_stats::*;
use clap::{App, Arg};
use std::fs::{File};
use std::io::{BufRead, BufReader};

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
    let output_file = matches.value_of("OUTPUT").unwrap();
    let (gold, parsed) = read_gng_data(intput_file, output_file);
    order_freq_hdt(&gold, &parsed, true, false);
    //definiteness_ud(&gold, &parsed, "dutch", true, false); // ... language, las, debug
    /*
    let (triples, properties) = extract_triples(&intput_file);
    let (templates, templates_aux) = generate_templates_dutch();
    sentence_generator(&triples, &properties, &templates, &templates_aux, "Daarom", "Omdat", output_file);
    */
}

/// `Properties' includes the property combination, e.g. accusative and ambiguous sentences carry
/// the property acc-amb
fn extract_triples(file: &str) -> (Vec<Vec<String>>, Vec<String>) {
    let mut triples = Vec::new();
    let mut properties = Vec::new();

    let f = File::open(&file).expect("Could not open file.");
    for l in BufReader::new(f).lines() {
        let mut l = l.unwrap();
        l = l.trim().to_string();
        let line = l.split("\t").collect::<Vec<_>>();
        // Standard order: S V O (VAUX) when input is S V O or S VAUX O V
        if line.len() == 6 {
            triples.push(vec![line[0].to_owned(), line[2].to_owned(), line[5].to_owned(), line[4].to_owned(), line[3].to_owned()]);
            properties.push(line[1].to_owned());
        } else if line.len() == 5 {
            triples.push(vec![line[0].to_owned(), line[2].to_owned(), line[3].to_owned(), line[4].to_owned()]);
            properties.push(line[1].to_owned());
        } else {
            eprintln!("Triple length {} not supported.", line.len());
            eprintln!("{:?}", line);
        }
    }
    (triples, properties)
}

fn generate_templates_german() -> (Vec<Vec<String>>, Vec<Vec<String>>)  {
    let mut templates = Vec::new();
    let s = "S".to_owned();
    let v = "V".to_owned();
    let o = "O".to_owned();
    templates.push(vec![s.clone(), v.clone(), o.clone()]);
    templates.push(vec![o.clone(), v.clone(), s.clone()]);
    templates.push(vec!["Deshalb".to_owned(), v.clone(), s.clone(), o.clone()]);
    templates.push(vec!["Deshalb".to_owned(), v.clone(), o.clone(), s.clone()]);
    templates.push(vec![v.clone(), s.clone(), o.clone(), "?".to_owned()]);
    templates.push(vec![v.clone(), o.clone(), s.clone(), "?".to_owned()]);
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
    templates_aux.push(vec![v_aux.clone(), s.clone(), o.clone(), v.clone(), "?".to_owned()]);
    templates_aux.push(vec![v_aux.clone(), o.clone(), s.clone(), v.clone(), "?".to_owned()]);
    templates_aux.push(vec!["Weil".to_owned(), s.clone(), o.clone(), v.clone(), v_aux.clone()]);
    templates_aux.push(vec!["Weil".to_owned(), o.clone(), s.clone(), v.clone(), v_aux.clone()]);

    (templates, templates_aux)
}

fn generate_templates_dutch() -> (Vec<Vec<String>>, Vec<Vec<String>>)  {
    let mut templates = Vec::new();
    let s = "S".to_owned();
    let v = "V".to_owned();
    let o = "O".to_owned();
    templates.push(vec![s.clone(), v.clone(), o.clone()]);
    templates.push(vec![o.clone(), v.clone(), s.clone()]);
    templates.push(vec!["Daarom".to_owned(), v.clone(), s.clone(), o.clone()]);
    templates.push(vec![v.clone(), s.clone(), o.clone(), "?".to_owned()]);
    templates.push(vec!["Omdat".to_owned(), s.clone(), o.clone(), v.clone()]);

    let mut templates_aux = Vec::new();
    let s = "S".to_owned();
    let v_aux = "VAUX".to_owned();
    let v = "V".to_owned();
    let o = "O".to_owned();
    templates_aux.push(vec![s.clone(), v_aux.clone(), o.clone(), v.clone()]);
    templates_aux.push(vec![o.clone(), v_aux.clone(), s.clone(), v.clone()]);
    templates_aux.push(vec!["Daarom".to_owned(), v_aux.clone(), s.clone(), o.clone(), v.clone()]);
    templates_aux.push(vec![v_aux.clone(), s.clone(), o.clone(), v.clone(), "?".to_owned()]);
    templates_aux.push(vec!["Omdat".to_owned(), s.clone(), o.clone(), v_aux.clone(), v.clone()]);

    (templates, templates_aux)
}