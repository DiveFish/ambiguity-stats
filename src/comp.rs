extern crate conllx;
use conllx::Token;

/// Utility class for comparing tokens and
/// specific properties of one or two tokens

pub fn heads_and_deprels_equal(token1: &Token, token2: &Token) -> bool {
    heads_equal(token1, token2) && deprels_equal(token1, token2)
}

pub fn heads_equal(token1: &Token, token2: &Token) -> bool {
    token1.head().expect("No head") == token2.head().expect("No head")
}

pub fn deprels_equal(token1: &Token, token2: &Token) -> bool {
    token1.head_rel().expect("No deprel") == token2.head_rel().expect("No deprel")
}

pub fn postags_equal(token1: &Token, token2: &Token) -> bool {
    token1.pos().expect("No PoS tag") == token2.pos().expect("No PoS tag")
}

pub fn check_deprel(token: &Token, deprel: &str) -> bool {
    token.head_rel().expect("No PoS tag") == deprel
}

pub fn check_postag(token: &Token, pos: &str) -> bool {
    token.pos().expect("No PoS tag") == pos
}

pub fn check_deprels(token1: &Token, deprel1: &str, token2: &Token, deprel2: &str) -> bool {
    (token1.head_rel().expect("No PoS tag") == deprel1)
        && (token2.head_rel().expect("No PoS tag") == deprel2)
}

pub fn check_postags(token1: &Token, pos1: &str, token2: &Token, pos2: &str) -> bool {
    (token1.pos().expect("No PoS tag") == pos1) && (token2.pos().expect("No PoS tag") == pos2)
}
