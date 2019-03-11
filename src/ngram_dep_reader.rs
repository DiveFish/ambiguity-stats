extern crate conllx;

use conllx::Token;
use std::collections::HashMap;

/// A utility class to read in tokens with direct dependency relation.
/// Save token pairs with same relation into same file,
/// different relations into different files.

pub fn get_deprel_ngrams(
    sentences: &Vec<Vec<Token>>,
    max_depth: usize,
) -> HashMap<String, Vec<String>> {
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

            let mut prev_head = usize::max_value();

            while depth < max_depth && prev_head > 0 {
                match cur_token.head() {
                    None => break,
                    Some(head) => if head == 0 {
                        // Head is ROOT
                        ngram.push_str(" ROOT");
                        deprels.push_str("_ROOT");
                        prev_head = head;
                    } else {
                        deprels.push_str(&cur_token.head_rel().unwrap());
                        deprels.push_str("_");
                        cur_token = sentence[head - 1].clone();
                        ngram.push_str(" ");
                        if cur_token.pos().unwrap().starts_with('N') {
                            ngram.push_str(&cur_token.form());
                        } else {
                            ngram.push_str(&cur_token.form().to_lowercase());
                        }
                        prev_head = head;
                    },
                }
                depth += 1;
            }

            if depth == max_depth {
                ngram.push_str("\n");
                // For removing strings that contain a dash, e.g. "-PUNCT-":
                // deprels = deprels.chars().filter(|&c| !deprels.contains("-")).collect();

                let n = ngram.clone();
                //println!("Ngram: {}deprels: {}\n", &n, &deprels);
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

pub fn get_deprel_bigrams(sentences: &Vec<Vec<Token>>) -> HashMap<String, Vec<String>> {
    let mut rel_map: HashMap<String, Vec<String>> = HashMap::new();

    for sentence in sentences {
        for token in sentence {
            let mut ngram = "".to_string();

            let mut token_form = "".to_string();
            match token.pos() {
                None => continue,
                Some(pos) => if pos.starts_with("N") {
                    token_form = token.form().to_string();
                } else {
                    token_form = token.form().to_lowercase();
                },
            }

            match token.head() {
                None => break,
                Some(head) => if head == 0 {
                    // Head is ROOT
                    ngram = format!("ROOT {}\n", token_form);
                } else {
                    let mut head_form = "".to_string();
                    match sentence[head - 1].pos() {
                        None => continue,
                        Some(pos) => if pos.starts_with("N") {
                            head_form = sentence[head - 1].form().to_string();
                        } else {
                            head_form = sentence[head - 1].form().to_lowercase();
                        },
                    }

                    ngram = format!("{} {}\n", head_form, token_form);
                },
            }
            let deprel = token.head_rel().expect("No deprel");
            if rel_map.contains_key(deprel) {
                rel_map.get_mut(deprel).unwrap().push(ngram);
            } else {
                rel_map.insert(deprel.to_string(), vec![ngram]);
            }
        }
    }

    rel_map
}
