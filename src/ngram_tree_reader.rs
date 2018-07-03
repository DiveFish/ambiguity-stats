extern crate conllx;

use conllx::Token;
use std::collections::HashMap;
use ambiguity_stats::sentence_tree::*;


pub fn get_tree_ngrams(sentences: &Vec<Vec<Token>>, max_depth: usize) -> HashMap<String, Vec<String>> {

    let mut rel_map: HashMap<String, Vec<String>> = HashMap::new();

    let sentence_trees: Vec<SentenceTree> = Vec::new();
    for sentence in sentences {
        sentence_trees.push(SentenceTree::from_sentence(sentence));
    }

    rel_map
}