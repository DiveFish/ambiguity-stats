extern crate conllx;

use conllx::Token;
use std::collections::{HashMap, HashSet};
use {DependencyGraph, DependencyNode, first_matching_edge, sentence_to_graph, to_dot};
use petgraph::dot::{Dot, Config};
use petgraph::EdgeDirection;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

static PP_RELATION: &'static str = "PP";
static OBJP_RELATION: &'static str = "OBJP";
/*
// We are only interested in PP/OBJP edges.
            if !PP_RELATIONS.contains(deprel) {
                continue;
            }
*/

const VERB_PREFIX: char = 'V';

// Code in this file partially taken from conllx-utils repository
// Credits to Daniël de Kok

lazy_static! {
    static ref PP_RELATIONS: HashSet<&'static str> = [PP_RELATION, OBJP_RELATION].iter().cloned().collect();
}

macro_rules! ok_or_continue {
    ($expr:expr) => (match $expr {
        Some(val) => val,
        None => continue,
    })
}

//
// For PP/OBJP-PN combinations, the pmis have already been retrieved

// Only look at trigrams (bigrams are much easier to retrieve than this).
/// Get trigram of shape <dep1 - head - dep2> and <dep2 - head - dep1>
/// for specific dependency relation combinations, e.g. for SUBJ-ROOT-OBJ
/// and OBJ-ROOT-SUBJ, look for dep1=SUBJ and dep2=OBJD.
pub fn get_graph_ngrams(sentences: &Vec<Vec<Token>>, max_depth: usize,
                        dep1: &str, dep2: &str) -> HashMap<String, Vec<String>> {

    let mut rel_map: HashMap<String, Vec<String>> = HashMap::new();

    for sentence in sentences {

        let mut dep1idx = NodeIndex::new(0);
        let mut dep2idx= NodeIndex::new(0);
        let mut dep1headidx= NodeIndex::new(0);
        let mut dep2headidx= NodeIndex::new(0);
        let mut updated1 = false;
        let mut updated2 = false;

        let sentence_graph = sentence_to_graph(&sentence, false);

        // Print graph:
        //to_dot(&sentence_graph);

        for edge_ref in sentence_graph.edge_references() {
            // Skip unlabeled edges.
            let deprel = ok_or_continue!(*edge_ref.weight());

            if deprel == dep1 {
                dep1idx = edge_ref.target();
                dep1headidx = edge_ref.source();
                updated1 = true;
            } else if deprel == dep2 {
                dep2idx = edge_ref.target();
                dep2headidx = edge_ref.source();
                updated2 = true;
            } else {
                continue;
            }

            let head_node = &sentence_graph[edge_ref.source()];
/*
            // Check that the head is a verb.
            let tag = ok_or_continue!(head_node.token.pos());
            if !tag.starts_with(VERB_PREFIX) {
                continue;
            }

            let content_verb_idx = resolve_verb(&sentence_graph, edge_ref.source());
            if content_verb_idx != edge_ref.source() {
                let prep_offset = sentence_graph[edge_ref.target()].offset;
                let content_verb_offset = sentence_graph[content_verb_idx].offset;

                updates.push((prep_offset, content_verb_offset));
            }
*/
            if dep1idx.index() > 0 && dep2idx.index() > 0 && dep1headidx == dep2headidx
                && updated1 && updated2 {
                //rel_map.insert();
            }
        }
    }
    rel_map
}

/*
/// Given a node `verb` that represents a verb, find the content
/// (non-auxiliary/-modal) verb. If the given verb is already a
/// content verb, the index of the verb itself is returned.
fn resolve_verb(graph: &DependencyGraph, verb: NodeIndex) -> NodeIndex {
    match first_matching_edge(graph, verb, EdgeDirection::Outgoing, |e| {
        *e == Some(AUXILIARY_RELATION)
    }) {
        Some(idx) => resolve_verb(graph, idx),
        None => verb,
    }
}
*/