extern crate conllx;

use conllx::{ReadSentence, Reader, Token};
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// Read gold and non-gold data files
pub fn read_gng_data(
    golddata_file: &str,
    nongolddata_file: &str,
) -> (Vec<Vec<Token>>, Vec<Vec<Token>>) {
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
    if filename.ends_with(".conll.gz") {
        let reader = File::open(filename).expect("Couldn't open file");
        let boxed_reader = BufReader::new(GzDecoder::new(reader).expect("Couldn't unzip .gz file"));
        Reader::new(boxed_reader)
            .sentences()
            .map(|s| s.unwrap())
            .collect()
    } else if filename.ends_with(".conll") {
        let reader = File::open(filename).expect("Couldn't open file");
        Reader::new(BufReader::new(reader))
            .sentences()
            .map(|s| s.unwrap())
            .collect()
    } else {
        Vec::new()
    }
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
                if filename.ends_with("conll") || filename.ends_with("conll.gz") {
                    files.push(filename);
                }
            }
        }
    } else if dir.is_file() {
        let filename = dir.to_str().unwrap().clone().to_string();
        if filename.ends_with("conll") || filename.ends_with("conll.gz") {
            files.push(filename);
        }
    }
}

pub fn compute_mi_to_dpar_pmis(input_dir: &Path, output_filename: &str) -> io::Result<()> {
    if fs::metadata(&output_filename).is_ok() {
        println!("Appending to file ({})", output_filename);
        let mut file = OpenOptions::new().append(true).open(output_filename).unwrap();
        if input_dir.is_dir() {
            for entry in fs::read_dir(input_dir).unwrap() {
                let path = entry.unwrap().path();
                if path.to_str().unwrap().clone().to_string().ends_with("nsc") {
                    if path.is_file() {
                        let f = File::open(&path)?;
                        for l in BufReader::new(f).lines() {
                            let l = l.unwrap();
                            let line = l.split("\t").collect::<Vec<_>>();
                            let (w1, w2, deprel, pmi) = (
                                line[0].to_string(),
                                line[1].to_string(),
                                path.file_stem().unwrap().to_string_lossy().to_string(),
                                line[2].to_string(),
                            );
                            writeln!(file, "{}\t{}\t{}\t{}", w1, w2, deprel, pmi);
                        }
                    }
                }
            }
        }
    } else {
        println!("Creating new file \"{}\"", output_filename);
        let mut file = File::create(&output_filename)?;
        if input_dir.is_dir() {
            for entry in fs::read_dir(input_dir).unwrap() {
                let path = entry.unwrap().path();
                if path.to_str().unwrap().clone().to_string().ends_with("nsc") {
                    if path.is_file() {
                        let f = File::open(&path)?;
                        for l in BufReader::new(f).lines() {
                            let l = l.unwrap();
                            let line = l.split("\t").collect::<Vec<_>>();
                            let (w1, w2, deprel, pmi) = (
                                line[0].to_string(),
                                line[1].to_string(),
                                path.file_stem().unwrap().to_string_lossy().to_string(),
                                line[2].to_string(),
                            );
                            writeln!(file, "{}\t{}\t{}\t{}", w1, w2, deprel, pmi);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
