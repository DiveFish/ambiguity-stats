extern crate conllx;

use conllx::{Token, Features};

pub type SentenceTree = Vec<Node>;

pub fn from_sentence(sentence: &Vec<Token>) -> SentenceTree {
    let root = Node::root();
    let mut sentence_tree = Vec::new();
    &sentence_tree.push(root);

    let mut relations: Vec<(usize, usize)> = Vec::new(); // <child, parent>

    for idx in 0..sentence.len() {
        &sentence_tree.push(Node::new(&sentence[idx].clone(), idx));
        let head_idx: usize = sentence[idx].head().expect("No head");
        relations.push((idx, head_idx));
    }

    for (idx1, idx2) in relations {
        sentence_tree.get_mut(idx1).as_mut().unwrap().add_parent(idx2);
        sentence_tree.get_mut(idx2).as_mut().unwrap().add_child(idx1);
    }

    sentence_tree
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    form: String,
    pos: String,
    deprel: Option<String>,
    features: Option<Features>,
    head: Option<usize>,
    index: usize,
    parents: Vec<usize>,
    children: Vec<usize>
}

impl Node {

    pub fn new(token: &Token, index: usize) -> Node {
        Node {
            form: token.form().to_string(),
            pos: token.pos().expect("No PoS tag").to_string(),
            deprel: Some(token.head_rel().expect("No deprel").to_string()),
            features: Some(token.features().expect("No features").clone()),
            head: Some(token.head().expect("No head")),
            index,
            parents: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn root() -> Node {
        Node {
            form: "ROOT".to_string(),
            pos: "ROOT".to_string(),
            deprel: None,
            features: None,
            head: None,
            index: 0,
            parents: vec![0],
            children: Vec::new(),
        }
    }

    pub fn get_form(&self) -> &str { self.form.as_ref() }

    pub fn get_pos(&self) -> &str { self.pos.as_ref() }

    pub fn get_deprel(&self) -> Option<&str> { self.deprel.as_ref().map(String::as_ref) }

    pub fn get_features(&self) -> Option<&Features> {
        self.features.as_ref()
    }

    pub fn get_index(&self) -> usize { self.index }

    pub fn get_head(&self) -> Option<usize> { self.head }

    pub fn get_parents(&self) -> &Vec<usize> { self.parents.as_ref() }

    pub fn get_children(&self) -> &Vec<usize> { self.children.as_ref() }

    pub fn add_parent(&mut self, parent: usize) {
        self.parents.push(parent);
    }

    pub fn add_child(&mut self, child: usize) {
        self.children.push(child);
    }
}