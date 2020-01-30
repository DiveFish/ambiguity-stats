extern crate conllx;
extern crate xml;

use conllx::{ReadSentence, Reader, Token};
use flate2::read::GzDecoder;
use xml::reader::{EventReader, XmlEvent};

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
    } else if filename.ends_with(".conll") || filename.ends_with(".tsv") {
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
                files.push(filename);
            }
        }
    } else if dir.is_file() {
        let filename = dir.to_str().unwrap().clone().to_string();
        files.push(filename);
    }
}

/// Get all files from a directory, also recursively if necessary.
fn get_treebank_files(dir: &Path, files: &mut Vec<String>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                get_files(&path, files);
            } else {
                let filename = path.to_str().unwrap().clone().to_string();
                if filename.ends_with("conll") || filename.ends_with("tsv") || filename.ends_with("conll.gz") {
                    files.push(filename);
                }
            }
        }
    } else if dir.is_file() {
        let filename = dir.to_str().unwrap().clone().to_string();
        if filename.ends_with("conll") || filename.ends_with("tsv") || filename.ends_with("conll.gz") {
            files.push(filename);
        }
    }
}

pub fn bigram_pmi_to_dpar_pmis(
    input_dir: &Path,
    output_filename: &str,
    file_ending: &str,
) -> io::Result<()> {
    if fs::metadata(&output_filename).is_ok() {
        println!("Appending to file ({})", output_filename);
        let mut file = OpenOptions::new()
            .append(true)
            .open(output_filename)
            .unwrap();
        if input_dir.is_dir() {
            for entry in fs::read_dir(input_dir).unwrap() {
                let path = entry.unwrap().path();
                if path
                    .to_str()
                    .unwrap()
                    .clone()
                    .to_string()
                    .ends_with(file_ending)
                {
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
                if path
                    .to_str()
                    .unwrap()
                    .clone()
                    .to_string()
                    .ends_with(file_ending)
                {
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

pub fn bigram_UD_pmi_to_dpar_pmis(
    input_dir: &Path,
    output_filename: &str,
    file_ending: &str,
) -> io::Result<()> {
    if fs::metadata(&output_filename).is_ok() {
        println!("Appending to file ({})", output_filename);
        let mut file = OpenOptions::new()
            .append(true)
            .open(output_filename)
            .unwrap();
        if input_dir.is_dir() {
            for entry in fs::read_dir(input_dir).unwrap() {
                let path = entry.unwrap().path();
                if path
                    .to_str()
                    .unwrap()
                    .clone()
                    .to_string()
                    .ends_with(file_ending)
                    {
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
                                writeln!(file, "{}\t{}\t{}\t{}", w1, w2, deprel.replace("/", ":"), pmi);
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
                if path
                    .to_str()
                    .unwrap()
                    .clone()
                    .to_string()
                    .ends_with(file_ending)
                    {
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
                                writeln!(file, "{}\t{}\t{}\t{}", w1, w2, deprel.replace("/", ":"), pmi);
                            }
                        }
                    }
            }
        }
    }

    Ok(())
}

pub fn trigram_pmi_to_dpar_pmis(
    input_dir: &Path,
    output_filename: &str,
    file_ending: &str,
) -> io::Result<()> {
    if fs::metadata(&output_filename).is_ok() {
        println!("Appending to file ({})", output_filename);
        let mut file = OpenOptions::new()
            .append(true)
            .open(output_filename)
            .unwrap();
        if input_dir.is_dir() {
            for entry in fs::read_dir(input_dir).unwrap() {
                let path = entry.unwrap().path();
                if path
                    .to_str()
                    .unwrap()
                    .clone()
                    .to_string()
                    .ends_with(file_ending)
                {
                    if path.is_file() {
                        let f = File::open(&path)?;
                        for l in BufReader::new(f).lines() {
                            let l = l.unwrap();
                            let filename = path.file_stem().unwrap().to_string_lossy().to_string();
                            let deprels = filename.split("_").collect::<Vec<_>>();
                            let (d1, d2) = (deprels[0], deprels[1]);
                            let line = l.split("\t").collect::<Vec<_>>();
                            let (w1, w2, w3, pmi) = (
                                line[0].to_string(),
                                line[1].to_string(),
                                line[2].to_string(),
                                line[3].to_string(),
                            );
                            writeln!(file, "{}\t{}\t{}\t{}\t{}\t{}", w1, w2, w3, d1, d2, pmi);
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
                if path
                    .to_str()
                    .unwrap()
                    .clone()
                    .to_string()
                    .ends_with(file_ending)
                {
                    if path.is_file() {
                        let f = File::open(&path)?;
                        for l in BufReader::new(f).lines() {
                            let l = l.unwrap();
                            let filename = path.file_stem().unwrap().to_string_lossy().to_string();
                            let deprels = filename.split("_").collect::<Vec<_>>();
                            let (d1, d2) = (deprels[0], deprels[1]);
                            let line = l.split("\t").collect::<Vec<_>>();
                            let (w1, w2, w3, pmi) = (
                                line[0].to_string(),
                                line[1].to_string(),
                                line[2].to_string(),
                                line[3].to_string(),
                            );
                            writeln!(file, "{}\t{}\t{}\t{}\t{}\t{}", w1, w2, w3, d1, d2, pmi);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Read association strengths for dependency triples from a text file.
///
/// Such a text file consists of lines with the tab-separated format
///
/// ~~~text,no_run
/// [token+] [token+] [deprel+] association_strength
/// ~~~
pub fn combine_assoc_files(
    in_large: File,
    in_small: File,
) -> io::Result<HashMap<(String, String, String), f32>> {
    let mut association_strengths: HashMap<(String, String, String), f32> = HashMap::new();
    for l in BufReader::new(in_large).lines() {
        let l = l.unwrap();
        let line = l.split("\t").collect::<Vec<_>>();

        let triple = (
            line[0].to_string(),
            line[1].to_string(),
            line[2].to_string(),
        );
        association_strengths.insert(triple, line[3].parse::<f32>().unwrap());
    }

    for l in BufReader::new(in_small).lines() {
        let l = l.unwrap();
        let line = l.split("\t").collect::<Vec<_>>();

        let triple = (
            line[0].to_string(),
            line[1].to_string(),
            line[2].to_string(),
        );

        if !association_strengths.contains_key(&triple) {
            association_strengths.insert(triple, line[3].parse::<f32>().unwrap());
        }
    }
    Ok(association_strengths)
}

pub fn combine_tri_bigram_files(
    in_tri: File,
    in_bi: File,
) -> io::Result<HashMap<(String, String, String, String, String), f32>> {
    let mut association_strengths: HashMap<(String, String, String, String, String), f32> =
        HashMap::new();
    for l in BufReader::new(in_tri).lines() {
        let l = l.unwrap();
        let line = l.split("\t").collect::<Vec<_>>();

        let tuple = (
            line[0].to_string(),
            line[1].to_string(),
            line[2].to_string(),
            line[3].to_string(),
            line[4].to_string(),
        );
        association_strengths.insert(tuple, line[5].parse::<f32>().unwrap());
    }

    for l in BufReader::new(in_bi).lines() {
        let l = l.unwrap();
        let line = l.split("\t").collect::<Vec<_>>();

        let triple = (
            "_".to_string(),
            line[0].to_string(),
            line[1].to_string(),
            "_".to_string(),
            line[2].to_string(),
        );

        if !association_strengths.contains_key(&triple) {
            association_strengths.insert(triple, line[3].parse::<f32>().unwrap());
        }
    }
    Ok(association_strengths)
}

/// Collect xml events of a specific event type.
/// Example usage: read_xml(xml_file, "orthForm");
pub fn read_xml(filename: &str, word_event: &str, frame_event: &str) {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let xml_parser = EventReader::new(reader);
    let mut depth = 0;
    let mut in_word = false;
    let mut in_frame = false;
    for e in xml_parser {
        match e {
            Ok(XmlEvent::StartElement { ref name, .. }) => {
                if name.local_name == word_event {
                    in_word = true;
                } else if name.local_name == frame_event {
                    in_frame = true;
                }
                depth += 1;
            }
            Ok(XmlEvent::Characters { .. } ) => {
                if in_word || in_frame {
                    match e {
                        Ok(XmlEvent::Characters(s)) => {
                            if in_frame {
                                print!("{},", s);
                            } else {
                                print!("\n{}:", s);
                            }
                        },
                        _ => {
                            println!("UNKN");
                        }
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                if in_word {
                    in_word = false;
                } else if in_frame {
                    in_frame = false;
                }
                //println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}