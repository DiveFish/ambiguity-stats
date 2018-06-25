extern crate conllx;
extern crate flate2;

use conllx::{Token, Reader, ReadSentence};
use std::fs::{self, File};
use std::io::{BufReader};
use std::path::Path;
use self::flate2::bufread::GzDecoder;

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

/// Get all files from a directory the name of which is provided as string.
/// Return list of all files in directory incl. subdirectories.
pub fn get_all_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    let dir = Path::new(path);
    get_files(dir, &mut files);
    files
}

/// Get all files from a directory, also recursively if necessary.
fn get_files(dir: &Path, files: &mut Vec<String>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                get_files(&path, files);
            } else {
                let filename = path.to_str().unwrap().clone().to_string();
                if filename.ends_with("conll") {
                    files.push(filename);
                }
            }
        }
    }
}