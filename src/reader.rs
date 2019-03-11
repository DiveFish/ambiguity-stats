use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::Result;

/// Generic function for different kinds of ngram readers
/// Save word list in files, one file per key.
pub fn ngrams_to_files<'a>(
    file_name_template: &'a str,
    rel_map: HashMap<String, Vec<String>>,
) -> Result<()> {
    for (key, value) in rel_map.iter() {
        let filename = format!("{}{}.txt", file_name_template, key); //TODO: rather use push?

        if fs::metadata(&filename).is_ok() {
            let mut file = OpenOptions::new().append(true).open(filename).unwrap();
            for ngram in value.iter() {
                file.write_all(ngram.as_bytes());
            }
        } else {
            let mut file = File::create(&filename)?;
            for ngram in value.iter() {
                file.write_all(ngram.as_bytes());
            }
        }
    }
    Ok(())
}

/// Save word list in single file.
pub fn ngrams_to_file<'a>(
    file_name_template: &'a str,
    rel_map: HashMap<String, Vec<String>>,
) -> Result<()> {
    let filename = format!("{}d.txt", file_name_template);

    for (_, value) in rel_map.iter() {
        if fs::metadata(&filename).is_ok() {
            let mut file = OpenOptions::new().append(true).open(&filename).unwrap();
            for ngram in value.iter() {
                let mut ngram = ngram.split_whitespace();
                let ngram1_out = ngram.next().unwrap();
                let ngram2_out = ngram.next().unwrap();
                let ngram_out = format!("{}\t{}", ngram1_out, ngram2_out);
                file.write_all(ngram_out.as_bytes())
                    .expect("Unable to write to file");
            }
        } else {
            let mut file = File::create(&filename)?;
            for ngram in value.iter() {
                let mut ngram = ngram.split_whitespace();
                let ngram1_out = ngram.next().unwrap();
                let ngram2_out = ngram.next().unwrap();
                let ngram_out = format!("{}\t{}", ngram1_out, ngram2_out);
                file.write_all(ngram_out.as_bytes())
                    .expect("Unable to write to file");
            }
        }
    }
    Ok(())
}
