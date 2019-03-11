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

//TODO: Fix variable initialization
#[cfg(test)]
mod tests {
    use comps::*;

    //use tests::{TOKEN_1, TOKEN_2};
    use std::fs::File;
    use std::io::BufReader;

    extern crate conllx;
    use conllx::{ReadSentence, Reader, Token};

    static sent: Vec<Vec<Token>> =
        Reader::new(BufReader::new(File::open("data/testdata.conll").unwrap()))
            .sentences()
            .map(|s| s.unwrap())
            .collect();

    #[test]
    fn test_heads_and_deprels_equal() {
        assert!(heads_and_deprels_equal(&sent[0][0], &sent[0][0]), true);
    }

    #[test]
    fn test1_check_deprels() {
        assert!(check_deprels(&sent[0][0], "ART", &sent[0][1], "NN"), true);
    }

    #[test]
    fn test2_check_deprels() {
        assert!(check_deprels(&sent[0][0], "ART", &sent[0][1], "ART"), false);
    }
}
