extern crate conllx;

use conllx::Token;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::Result;

/// A utility class to read in tokens with direct dependency relation.
/// Save token pairs with same relation into same file,
/// different relations into different files.

pub fn get_deprel_ngrams(sentences: &Vec<Vec<Token>>, max_depth: usize) -> HashMap<String, Vec<String>> {

    let mut rel_map: HashMap<String, Vec<String>> = HashMap::new();

    for sentence in sentences {
        for token in sentence {

            let mut depth: usize = 0;
            let mut cur_token = token.clone();

            let mut ngram = "".to_string();
            if cur_token.pos().unwrap().starts_with('N') {
                ngram.push_str(&cur_token.form());
            } else {
                ngram.push_str(&cur_token.form().to_lowercase());
            }

            let mut deprels = "".to_string();

            while depth < max_depth {

                let mut head_idx = 0;
                match cur_token.head() {
                    None => break,
                    Some(head) => if head == 0 {
                        ngram.push_str(" ROOT");
                        deprels.push_str("_ROOT");
                        depth += 1;
                        continue;
                    } else {
                        deprels.push_str(&cur_token.head_rel().unwrap());
                        deprels.push_str("_");
                        cur_token = sentence[head - 1].clone();
                    }
                }

                ngram.push_str(" ");
                if cur_token.pos().unwrap().starts_with('N') {
                    ngram.push_str(&cur_token.form());
                } else {
                    ngram.push_str(&cur_token.form().to_lowercase());
                }

                depth += 1;
            }

            if depth == max_depth {
                ngram.push_str("\n");
                deprels = deprels.chars().filter(|&c| !deprels.contains("-")).collect();

                let n = ngram.clone();
                println!("{}", &n);
                println!("{}", &deprels);
                println!("\n\n");
                if rel_map.contains_key(&deprels) {
                    rel_map.get_mut(&deprels).unwrap().push(n);
                } else {
                    rel_map.insert(deprels.clone(), vec![n]);
                }
            }
        }
    }

    rel_map
}

/// Save word list in files, one file per key
pub fn deprel_ngrams_to_file<'a>(file_name_template: &'a str, rel_map: HashMap<String, Vec<String>>) -> Result<()> {

    for (key, value) in rel_map.iter() {

        let filename = format!("{}{}.txt", file_name_template, key);   //TODO: rather use push?

        if fs::metadata(&filename).is_ok() {
            let mut file = OpenOptions::new().append(true).open(filename).unwrap();
            for ngram in value.iter() {
                file.write_all(ngram.as_bytes()).unwrap();
            }
        } else {
            let mut file = File::create(&filename)?;
            for ngram in value.iter() {
                file.write_all(ngram.as_bytes()).unwrap();
            }
        }

    }
    Ok(())
}