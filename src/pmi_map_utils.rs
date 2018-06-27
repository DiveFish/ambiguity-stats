use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Result, Write};
use std::prelude::*;
use std::path::Path;
use std::error::Error;
use std::cmp::Ordering::Less;

pub fn read_pmi_file(input_file: &str, ngram_size: usize) -> HashMap<String, f32> {

    let path = Path::new(input_file);
    let file = match File::open(input_file) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(),
                           why.description()),
        Ok(file) => file,
    };
    let mut map:HashMap<String, f32> = HashMap::new();
    for reader in BufReader::new(file).lines() {
        let mut line = reader.unwrap();
        let mut split_line = line.split_whitespace();
        let mut ngram = "".to_owned();
        for i in 0..ngram_size+1 as usize {
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
    sort_map(&mut map);
    map
}

pub fn sort_map(map: &mut HashMap<String, f32>) -> Vec<(&String, &f32)> {

    let mut map_vec: Vec<(&String, &f32)> = map.iter().collect();
    map_vec.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Less));
    map_vec
}

pub fn pmi_to_file(map: HashMap<String, f32>, output_file: &str) -> Result<()> {

    for entry in map.iter() {
        let mut file = File::create(&output_file)?;
        let mut line = entry.0.to_string();
        line.push_str(" ");
        line.push_str(&entry.1.to_string());
        file.write_all(line.as_bytes());
    }
    Ok(())
}