extern crate conllx;
use conllx::Token;

use heads_equal;
use deprels_equal;

//&[Token] <- reference to slice, allows slices AND vectors

// TODO: Account for ROOT token not being part of the sentence while 0 still used as index for referring to ROOT as head
pub fn pp_attachment(gold_sent: &[Token], nongold_sent: &[Token]) -> usize {
    assert_eq!(gold_sent.len(), nongold_sent.len());
    let mut errors = 0;
    let mut idx = 0;
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let mut gold_head_idx = gold_token.head().expect("No head");
        if gold_head_idx == 0 {
            continue
        } else {
            gold_head_idx -= 1;
        }
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let token = &nongold_sent[i];
        let mut head_idx = token.head().expect("No head idx");
        if head_idx == 0 {
            continue
        } else {
            head_idx -= 1;
        }
        let head = &nongold_sent[head_idx];

        if (gold_deprel == "PP") && deprels_equal(&token, &gold_token)
            && !heads_equal(&token, &gold_token) {
            errors += 1;
        }
        idx += 1;
    }
    errors
}

fn pp_objp(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn obj_fronting(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn verb_particle(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn subj_obj_separation(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn coordination(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn adjectives(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn pronoun_quant(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}