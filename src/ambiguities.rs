extern crate conllx;
use conllx::Token;

use heads_equal;
use deprels_equal;

//&[Token] <- reference to slice, allows slices AND vectors

// TODO: Add wrapper method to call any of the ambiguity count functions, signature: pub fn get_ambiguity_counts(gold_sent: &[Token], nongold_sent: &[Token], fun: Function) -> usize
// TODO: Account for ROOT token not being part of the sentence while 0 still used as index for referring to ROOT as head
// TODO: Also count correct attachments to calculate F1 score!
pub fn n_pp_attachments(gold_sent: &[Token], nongold_sent: &[Token]) -> (usize, usize) {

    assert_eq!(gold_sent.len(), nongold_sent.len());

    let mut overall_pps = 0;
    let mut errors = 0;

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

        if (gold_deprel == "PP") && deprels_equal(&token, &gold_token) {
            overall_pps += 1;
            if !heads_equal(&token, &gold_token) {
                errors += 1;
            }
        }
    }
    (overall_pps, errors)
}

pub fn n_pp_objps(gold_sent: &[Token], nongold_sent: &[Token]) -> (usize, usize) {

    assert_eq!(gold_sent.len(), nongold_sent.len());

    let mut overall_pps = 0;
    let mut errors = 0;

    for i in 0..gold_sent.len() {

        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let token = &nongold_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_deprel == "PP" || gold_deprel == "OBJP" {
            overall_pps += 1;
            if gold_deprel == "PP" && token_deprel == "OBJP" {
                errors += 1;
            } else if gold_deprel == "OBJP" && token_deprel == "PP" {
                errors += 1;
            }
        }
    }
    (overall_pps, errors)
}

pub fn n_obj_frontings(gold_sent: &[Token], nongold_sent: &[Token]) -> (usize, usize) {

    assert_eq!(gold_sent.len(), nongold_sent.len());

    let mut overall_frontedobjs = 0;
    let mut errors = 0;

    let mut gold_subjidx = 0;
    let mut gold_objidx = 0;
    let mut subjidx = 0;
    let mut objidx = 0;
    let mut isPassive = false;
    let mut isRelCl = false;

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_lemma = match gold_token.lemma() { //@Daniël: There seem to be None lemmas in the data
            Some(x) => gold_token.lemma().unwrap(),
            None => "",
        };

        let token = &nongold_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_deprel == "OBJD" {
            gold_objidx = i;
            if gold_token.pos().expect("No PoS tag") == "PRELS" {   // Exclude object fronting in relative clauses
                isRelCl = true;
            }
            if token_deprel == "SUBJ" {
                subjidx = i;    // Fronted OBJ mistaken for SUBJ
            }
        } else if gold_deprel == "SUBJ" {
            gold_subjidx = i;

        } else if gold_lemma == "werden%passiv" {   // Exclude passives
            isPassive = true;
        }

        if gold_deprel == "-PUNCT-" || i == gold_sent.len()-1 {
            if gold_subjidx > 0 && gold_objidx > 0 && gold_objidx < gold_subjidx && !isPassive && !isRelCl { // Fronted object
                overall_frontedobjs += 1;
                if subjidx > 0 {
                    errors += 1;
                    subjidx = 0;
                }
            }
            gold_subjidx = 0;
            gold_objidx = 0;
            subjidx = 0;
            isPassive = false;
            isRelCl = false;
        }
    }
    (overall_frontedobjs, errors)
}

fn n_verb_particles(gold_sent: &[Token], nongold_sent: &[Token]) -> (usize, usize) {
    unimplemented!()
}

fn n_subj_obj_splits(gold_sent: &[Token], nongold_sent: &[Token]) -> (usize, usize) {
    unimplemented!()
}

fn n_coordinations(gold_sent: &[Token], nongold_sent: &[Token]) -> (usize, usize) {
    unimplemented!()
}

fn n_adjectives(gold_sent: &[Token], nongold_sent: &[Token]) -> (usize, usize) {
    unimplemented!()
}

fn n_pronoun_quants(gold_sent: &[Token], nongold_sent: &[Token]) -> usize {
    unimplemented!()
}