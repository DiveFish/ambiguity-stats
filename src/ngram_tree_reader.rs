extern crate conllx;

use conllx::Token;
use std::collections::HashMap;
use sentence_tree::{SentenceTree, from_sentence};


pub fn get_tree_ngrams(sentences: &Vec<Vec<Token>>, max_depth: usize) -> HashMap<String, Vec<String>> {

    let mut sentence_trees: Vec<SentenceTree> = Vec::new();
    for sentence in sentences {
        sentence_trees.push(from_sentence(sentence));
    }

    let mut rel_map: HashMap<String, Vec<String>> = HashMap::new();
    for tree in sentence_trees {
        for node in tree {
            let mut depth_cnt = 0;
            //TODO: Process tree
        }
    }

    rel_map
}