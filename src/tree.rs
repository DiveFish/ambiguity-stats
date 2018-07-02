pub struct SentenceTree {
    sentence: Vec<Node>,
    root: Node
}

impl SentenceTree {
    fn new(root: Node) {
        sentence: Vec::new();
        root: root;
    }
}

pub struct Node {
    index: usize,
    head: usize,
    parent: Node,
    children: Vec<Node>
}

impl Node {
    fn new(index: usize, head: usize, parent: Node) {
        index: index;
        head: head;
        parent: parent; // Todo: check if this makes sense
        children: Vec::new();
    }
}