use std::fs::File;
use std::io::BufReader;

extern crate conllx;
use conllx::{Token, Reader, ReadSentence};

/// Read gold and non-gold data files
pub fn read_gng_data(golddata_file: &str, nongolddata_file: &str) -> (Vec<Vec<Token>>, Vec<Vec<Token>>) {
	let golddata = read_sentences(golddata_file);
	let nongolddata = read_sentences(nongolddata_file);
	(golddata, nongolddata)
}

/// Read single file
pub fn read_data(datafile: &str) -> Vec<Vec<Token>> {
	read_sentences(datafile)
}

// Taken from conllx-rs repo, tests.rs file
pub fn read_sentences(filename: &str) -> Vec<Vec<Token>> {
    Reader::new(BufReader::new(File::open(filename).unwrap()))
        .sentences()
        .map(|s| s.unwrap())
        .collect()
}