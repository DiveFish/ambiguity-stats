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
