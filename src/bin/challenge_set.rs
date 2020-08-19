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

    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();

    //let (gold, parsed) = read_gng_data(input_file, output_file);
    //order_freq_ud(&gold, &parsed, "german", false, false);
    //definiteness_hdt(&gold, &parsed, "german", true, true, false); // ... language, las, binary, debug
    //negated_objs(&gold, &parsed, "UD", "german");

    let input = read_data(input_file);
    //filter_gold(input, &[&"opron".to_string(),&"invan".to_string(),&"psy".to_string()]);
    let (triples, properties) = extract_sent_parts(&input_file);
    // GERMAN
    let (templates, templates_aux, templates_pp, templates_aux_pp) = generate_templates_german();
    sentence_generator(&triples, &properties, &templates, &templates_aux, &templates_pp, &templates_aux_pp, "Deshalb", "Weil", output_file);
    // DUTCH
    //let (templates, templates_aux, templates_pp) = generate_templates_dutch();
    //sentence_generator(&triples, &properties, &templates, &templates_aux, templates_pp, "Daarom", "Omdat", output_file);

}

/// `Properties' includes the property combination, e.g. accusative and ambiguous sentences carry
/// the property acc-amb
fn extract_sent_parts(file: &str) -> (Vec<Vec<String>>, Vec<String>) {
    let mut sent_parts = Vec::new();
    let mut properties = Vec::new();

    let f = File::open(&file).expect("Could not open file.");
    for l in BufReader::new(f).lines() {
        let mut l = l.unwrap();
        l = l.trim().to_string();
        let line = l.split("\t").collect::<Vec<_>>();

        // Standard order: S V O (VAUX)(PP) when input is S V O or S V_aux O V or S V O PP or S V_aux O PP
        if line.len() == 7 {    // Sentence contains AUX and PP
            sent_parts.push(vec![line[0].to_owned(), line[2].to_owned(), line[5].to_owned(), line[4].to_owned(), line[3].to_owned(), line[6].to_owned()]);
            properties.push(line[1].to_owned());
        } else if line.len() == 6 {
            if line[5].split(" ").collect::<Vec<_>>().len() > 1 {  // Sentence contains PP
                sent_parts.push(vec![line[0].to_owned(), line[2].to_owned(), line[3].to_owned(), line[4].to_owned(), line[5].to_owned()]);
                properties.push(line[1].to_owned());
            } else {    // Sentence contains AUX
                sent_parts.push(vec ! [line[0].to_owned(), line[2].to_owned(), line[5].to_owned(), line[4].to_owned(), line[3].to_owned()]);
                properties.push(line[1].to_owned());
            }
        } else if line.len() == 5 {
            sent_parts.push(vec![line[0].to_owned(), line[2].to_owned(), line[3].to_owned(), line[4].to_owned()]);
            properties.push(line[1].to_owned());
        } else {
            eprintln!("Sentence length {} not supported.", line.len());
            eprintln!("{:?}", line);
        }
    }
    (sent_parts, properties)
}

fn generate_templates_german() -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>)  {
    let s = "S".to_owned();
    let v = "V".to_owned();
    let o = "O".to_owned();
    let v_aux = "VAUX".to_owned();
    let pp = "PP".to_owned();

    let mut templates = Vec::new();
    templates.push(vec![s.clone(), v.clone(), o.clone()]);
    templates.push(vec![o.clone(), v.clone(), s.clone()]);
    templates.push(vec!["Deshalb".to_owned(), v.clone(), s.clone(), o.clone()]);
    templates.push(vec!["Deshalb".to_owned(), v.clone(), o.clone(), s.clone()]);
    templates.push(vec![v.clone(), s.clone(), o.clone(), "?".to_owned()]);
    templates.push(vec![v.clone(), o.clone(), s.clone(), "?".to_owned()]);
    templates.push(vec!["Weil".to_owned(), s.clone(), o.clone(), v.clone()]);
    templates.push(vec!["Weil".to_owned(), o.clone(), s.clone(), v.clone()]);

    let mut templates_aux = Vec::new();
    templates_aux.push(vec![s.clone(), v_aux.clone(), o.clone(), v.clone()]);
    templates_aux.push(vec![o.clone(), v_aux.clone(), s.clone(), v.clone()]);
    templates_aux.push(vec!["Deshalb".to_owned(), v_aux.clone(), s.clone(), o.clone(), v.clone()]);
    templates_aux.push(vec!["Deshalb".to_owned(), v_aux.clone(), o.clone(), s.clone(), v.clone()]);
    templates_aux.push(vec![v_aux.clone(), s.clone(), o.clone(), v.clone(), "?".to_owned()]);
    templates_aux.push(vec![v_aux.clone(), o.clone(), s.clone(), v.clone(), "?".to_owned()]);
    templates_aux.push(vec!["Weil".to_owned(), s.clone(), o.clone(), v.clone(), v_aux.clone()]);
    templates_aux.push(vec!["Weil".to_owned(), o.clone(), s.clone(), v.clone(), v_aux.clone()]);

    let mut templates_pp = Vec::new();
    templates_pp.push(vec![s.clone(), v.clone(), pp.clone(), o.clone()]);
    templates_pp.push(vec![o.clone(), v.clone(), pp.clone(), s.clone()]);
    templates_pp.push(vec!["Deshalb".to_owned(), v.clone(), pp.clone(), s.clone(), o.clone()]);
    templates_pp.push(vec!["Deshalb".to_owned(), v.clone(), pp.clone(), o.clone(), s.clone()]);
    templates_pp.push(vec![v.clone(), pp.clone(), s.clone(), o.clone(), "?".to_owned()]);
    templates_pp.push(vec![v.clone(), pp.clone(), o.clone(), s.clone(), "?".to_owned()]);
    templates_pp.push(vec!["Weil".to_owned(), pp.clone(), s.clone(), o.clone(), v.clone()]);
    templates_pp.push(vec!["Weil".to_owned(), pp.clone(), o.clone(), s.clone(), v.clone()]);

    let mut templates_aux_pp = Vec::new();
    templates_aux_pp.push(vec![s.clone(), v_aux.clone(), pp.clone(), o.clone(), v.clone()]);
    templates_aux_pp.push(vec![o.clone(), v_aux.clone(), pp.clone(), s.clone(), v.clone()]);
    templates_aux_pp.push(vec!["Deshalb".to_owned(), v_aux.clone(), pp.clone(), s.clone(), o.clone(), v.clone()]);
    templates_aux_pp.push(vec!["Deshalb".to_owned(), v_aux.clone(), pp.clone(), o.clone(), s.clone(), v.clone()]);
    templates_aux_pp.push(vec![v_aux.clone(), pp.clone(), s.clone(), o.clone(), v.clone(), "?".to_owned()]);
    templates_aux_pp.push(vec![v_aux.clone(), pp.clone(), o.clone(), s.clone(), v.clone(), "?".to_owned()]);
    templates_aux_pp.push(vec!["Weil".to_owned(), pp.clone(), s.clone(), o.clone(), v.clone(), v_aux.clone()]);
    templates_aux_pp.push(vec!["Weil".to_owned(), pp.clone(), o.clone(), s.clone(), v.clone(), v_aux.clone()]);

    (templates, templates_aux, templates_pp, templates_aux_pp)
}

fn generate_templates_dutch() -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>)  {
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

    let mut templates_pp = Vec::new();
    let mut templates_aux_pp = Vec::new();
    //TODO: Implement

    (templates, templates_aux, templates_pp, templates_aux_pp)
}