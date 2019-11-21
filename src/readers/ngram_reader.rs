extern crate conllx;
use conllx::Token;

use std::collections::HashMap;

/// A utility class to read in token pairs with direct dependency relation.
/// Save token pairs with same relation into same file,
/// different relations into different files.

/// Read token ngrams, their deprels and save into map of shape <deprel, ngrams related by deprel>
pub fn get_ngrams(sentences: &Vec<Vec<Token>>, ngram_size: usize) -> HashMap<String, Vec<String>> {
    let mut rel_map: HashMap<String, Vec<String>> = HashMap::new(); //TODO: rather use a set than a vec here?

    for sentence in sentences {
        if sentence.len() >= ngram_size {
            for mut idx in 0..(sentence.len() - ngram_size) {
                let mut deprels = "".to_owned();
                let mut ngram_concat = "".to_owned();

                let mut ngram_idx: usize = 0;

                // Get all words within the ngram_size range; store words AND relation between them
                // in the map (key: relations as concatenated string; value: vec of words as string)
                while ngram_idx < ngram_size {
                    let pos = sentence[ngram_idx + idx].pos().unwrap();
                    // Don't lowercase (proper) nouns
                    if pos.starts_with('N') {
                        ngram_concat.push_str(&sentence[ngram_idx + idx].form());
                    } else {
                        ngram_concat.push_str(&sentence[ngram_idx + idx].form().to_lowercase());
                    }
                    ngram_concat.push_str(" ");
                    deprels.push_str(&sentence[ngram_idx + idx].head_rel().expect("No deprel"));
                    deprels.push_str("_");
                    ngram_idx += 1;
                }

                ngram_concat.push_str("\n");
                deprels = deprels
                    .chars()
                    .filter(|&c| !deprels.contains("-"))
                    .collect();

                let n = ngram_concat.clone();
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
