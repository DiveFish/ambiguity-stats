extern crate conllx;

use features::Features;
use conllx::{Token, Features, Sentence};
use std::collections::HashMap;

pub struct SentenceTree {
    sentence: Vec<Node>,
    root: Node
}

impl SentenceTree {
    pub fn new(root: Node) -> SentenceTree {
        SentenceTree {
            sentence: Vec::new(),
            root,
        }
    }

    pub fn from_sentence(sentences: Vec<Token>) -> SentenceTree {
        let root = Node::root();
        let mut sentence_tree = SentenceTree::new(root);

        let mut relations: Vec<(usize, usize)> = Vec::new(); // <child, parent>

        for idx in 0..sentence.len() {
            sentence_tree.add_node(Node::from_token(&sentence[idx].clone(), idx));
            let head_idx: usize = &sentence[idx].head().unwrap();
            relations.push((idx, head_idx));
        }

        for (idx1, idx2) in relations {
            sentence_tree.get_node(idx1).add_parent(idx2);
            sentence_tree.get_node(idx2).add_child(idx1);
        }

        sentence_tree
    }

    pub fn add_node(mut self, node: Node) {
        self.sentence.push(node);
    }

    pub fn get_node(self, index: usize) -> &mut Node {
        self.sentence.get(index).expect("No node at this index");
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    form: String,
    pos: Option<String>,
    deprel: Option<String>,
    features: Option<Features>,
    head: Option<usize>,
    index: usize,
    parents: Vec<usize>,
    children: Vec<usize>
}

impl Node {

    pub fn new(form: String, index: usize) -> Node {
        Node {
            form: token.form(),
            pos: token.pos(),
            deprel: token.head_rel(),
            features: token.features(),
            head: token.head(),
            index,
            parents: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn from_token(token: Token, index: usize) -> Node {
        Node {
            form: token.form(),
            pos: token.pos(),
            deprel: token.head_rel(),
            features: token.features(),
            head: token.head(),
            index,
            parents: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn root() -> Node {
        Node {
            form: "ROOT",
            pos: Option("ROOT"),
            deprel: None,
            features: None,
            head: None,
            index: 0,
            parents: vec![0],
            children: Vec::new(),
        }
    }

    pub fn get_form(&self) -> &str { self.form }

    pub fn get_pos(&self) -> Option<&str> { self.pos.as_ref().map(String::as_ref) }

    pub fn get_deprel(&self) -> Option<&str> { self.deprel.as_ref().map(String::as_ref) }

    pub fn get_features(&self) -> Option<&Features> {
        self.features.as_ref()
    }

    pub fn get_index(self) -> usize { self.index }

    pub fn get_head(self) -> Option<usize> { self.head }

    pub fn get_parent(&self) -> &Node { self.parent.as_ref() }

    pub fn get_children(&self) -> &Vec<Node> { self.children }

    pub fn add_parent(mut self, parent: usize) {
        self.parents.push(parent);
    }

    pub fn add_child(mut self, child: usize) {
        self.children.push(child);
    }
}