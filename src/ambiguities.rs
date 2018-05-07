extern crate conllx;
use conllx::Token;

use heads_equal;
use deprels_equal;

//&[Token] <- reference to slice, allows slices AND vectors

// TODO: Add wrapper method to call any of the ambiguity count functions, signature: pub fn get_ambiguity_counts(gold_sent: &[Token], nongold_sent: &[Token], fun: Function) -> usize
// TODO: Account for ROOT token not being part of the sentence while 0 still used as index for referring to ROOT as head
// TODO: Also count correct attachments to calculate F1 score!
pub fn n_incorrect_pp_attachments(gold_sent: &[Token], nongold_sent: &[Token]) -> usize {

    assert_eq!(gold_sent.len(), nongold_sent.len());

    let mut errors = 0;
    let mut idx = 0;

    for i in 0..gold_sent.len() {

        let gold_token = &gold_sent[i];
        let mut gold_head_idx = gold_token.head().expect("No head");    //To avoid panic, use `match`
        if gold_head_idx == 0 { //Ignore tokens with ROOT as head
            continue
        } else {
            gold_head_idx -= 1;
        }
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let token = &nongold_sent[i];
        let mut head_idx = token.head().expect("No head idx");
        if head_idx == 0 {  //Ignore tokens with ROOT as head
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

fn n_incorrect_pp_objps(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn n_incorrect_obj_frontings(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn n_incorrect_verb_particles(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn n_incorrect_subj_obj_splits(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn n_incorrect_coordinations(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn n_incorrect_adjectives(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}

fn n_incorrect_pronoun_quants(sent: Vec<Token>) -> usize {
    let errors = 0;
    errors
}