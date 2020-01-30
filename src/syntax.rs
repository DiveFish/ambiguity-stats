extern crate conllx;
extern crate linked_hash_map;

use conllx::Token;

use linked_hash_map::LinkedHashMap;

/// Collect all possible combinations of PoS tags (or dependency relations) to find different
/// syntactic patterns.
pub fn label_combos(input: &[Token], pos_patterns: &mut LinkedHashMap<Vec<String>, usize>, sent_examples: &mut Vec<Vec<String>>) {
    let mut sent = Vec::with_capacity(input.len());
    let mut sent_pos = Vec::with_capacity(input.len());
    let mut subj = false;
    let mut obj = false;
    for i in 0..input.len() {
        sent.push(input[i].form().to_string());
        let deprel = input[i].head_rel().expect("No deprel");
        if ! (deprel == "-PUNCT-") {    //punct
            sent_pos.push(deprel.to_string());
        }

        if deprel == "SUBJ" {   //nsubj
            subj = true;
        } else if deprel.starts_with("OBJ") && !subj {    // For UD: ("obj") || deprel == "iobj")
            obj = true;
        }
    }

    //if subj && obj {
        if !pos_patterns.contains_key(&sent_pos) {
            sent_examples.push(sent);
        }
        *pos_patterns.entry(sent_pos).or_insert(0) += 1;
    //}
}

pub fn sentence_generator(svo_triples: &Vec<Vec<String>>, templates: &Vec<Vec<String>>) {
    for svo_triplet in svo_triples {
        for template in templates {
            for token in template {
                if token == "S" {
                    print!("{} ", svo_triplet[0]);
                } else if token == "V" {
                    print!("{} ", svo_triplet[1]);
                } else if token == "O" {
                    print!("{} ", svo_triplet[2]);
                } else {
                    print!("{} ", token);
                }
            }
            println!();
        }
    }
}