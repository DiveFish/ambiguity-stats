use conllx::{Sentence, Token};
use petgraph::{Directed, EdgeDirection, Graph};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

//Code in this file taken from conllx-utils repository
//Credits: Daniël de Kok

#[derive(Debug)]
pub struct DependencyNode<'a> {
    pub token: &'a Token,
    pub offset: usize,
}

pub type DependencyGraph<'a> = Graph<DependencyNode<'a>, Option<&'a str>, Directed>;

pub fn sentence_to_graph(sentence: &Sentence, projective: bool) -> DependencyGraph {
    let mut g = Graph::new();

    let nodes: Vec<_> = sentence
        .iter()
        .enumerate()
        .map(|(offset, token)| {
            g.add_node(DependencyNode {
                token: token,
                offset: offset,
            })
        })
        .collect();

    for (idx, token) in sentence.iter().enumerate() {
        let head = if projective {
            token.p_head()
        } else {
            token.head()
        };

        let rel = if projective {
            token.p_head_rel()
        } else {
            token.head_rel()
        };

        if let Some(head) = head {
            if head != 0 {
                g.add_edge(nodes[head - 1], nodes[idx], rel);
            }
        }
    }

    g
}

pub fn first_matching_edge<F>(
    graph: &DependencyGraph,
    index: NodeIndex,
    direction: EdgeDirection,   //Outgoing, Incoming
    predicate: F,
) -> Option<NodeIndex>
where
    F: Fn(&Option<&str>) -> bool,
{
    graph
        .edges_directed(index, direction)
        .find(|edge_ref| predicate(edge_ref.weight()))
        .map(|edge_ref| edge_ref.target())
}