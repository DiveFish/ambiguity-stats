use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::Result;

/// Generic function for different kinds of ngram readers
/// Save word list in files, one file per key.
pub fn ngrams_to_file<'a>(file_name_template: &'a str, rel_map: HashMap<String, Vec<String>>) -> Result<()> {

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