use std::cmp::Ordering::Less;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write};
use std::path::Path;

//Todo @Daniël: How to make a separate function pmis_to_file() with the Vec<&string, &f32> as input? <- lifetimes

pub fn sort_pmi_file(input_file: &str, ngram_size: usize, output_file: &str) -> Result<()> {
    let path = Path::new(input_file);
    let file = match File::open(input_file) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };
    let mut map: HashMap<String, f32> = HashMap::new();
    for reader in BufReader::new(file).lines() {
        let mut line = reader.unwrap();
        let mut split_line = line.split_whitespace();
        let mut ngram = "".to_owned();
        for i in 0..ngram_size + 1 as usize {
            if i < ngram_size {
                if i == 0 {
                    ngram.push_str(&split_line.next().unwrap());
                } else {
                    ngram.push_str(" ");
                    ngram.push_str(&split_line.next().unwrap());
                }
            } else if i == ngram_size {
                let pmi = split_line.next().unwrap();
                if pmi.parse::<f32>().is_ok() {
                    map.insert(ngram.clone(), pmi.parse::<f32>().unwrap());
                }
            }
        }
    }
    let mut map_vec: Vec<(&String, &f32)> = map.iter().collect();
    map_vec.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap_or(Less));

    // Split the following into separate method
    let mut file = File::create(&output_file)?;
    for entry in map_vec.iter() {
        let mut line = entry.0.to_string();
        line.push_str(" ");
        line.push_str(&entry.1.to_string());
        line.push_str("\n");
        file.write_all(line.as_bytes()).unwrap();
    }
    Ok(())
}

pub fn get_pmi(
    focus_words: &[String],
    context_words: &[String],
    deprels: &[String],
    input_file: &str,
) -> Result<()> {
    assert_eq!(focus_words.len(), context_words.len());
    assert_eq!(focus_words.len(), deprels.len());

    let path = Path::new(input_file);
    let file = match File::open(input_file) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    let mut association_strengths: HashMap<(String, String, String), f32> = HashMap::new();
    for l in BufReader::new(file).lines() {
        let l = l.unwrap();
        let line = l.split("\t").collect::<Vec<_>>();
        association_strengths.insert(
            (
                line[0].to_string(),
                line[1].to_string(),
                line[2].to_string(),
            ),
            line[3].parse::<f32>().unwrap(),
        );
    }

    for idx in 0..focus_words.len() {
        let head = &focus_words[idx];
        let dependent = &context_words[idx];
        let deprel = &deprels[idx];
        let dep_triple = (head.to_string(), dependent.to_string(), deprel.to_string());
        let association_strength = match association_strengths.get(&dep_triple) {
            Some(association_strength) => *association_strength,
            None => -1f32,
        };
        println!(
            "PMI of {} {} {}: {}",
            head, dependent, deprel, association_strength
        );
    }

    Ok(())
}
