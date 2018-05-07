extern crate conllx;
use conllx::Token;

use heads_equal;
use deprels_equal;

//&[Token] <- reference to slice, allows slices AND vectors

// TODO: Add wrapper method to call any of the ambiguity count functions, signature: pub fn get_ambiguity_counts(gold_sent: &[Token], nongold_sent: &[Token], fun: Function) -> usize
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
    let mut isPassive = false;  // Exclude passive, relative clauses or reflexives?
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

        if gold_deprel == "OBJD" || gold_deprel == "OBJA" {
            gold_objidx = i;
            if gold_token.pos().expect("No PoS tag") == "PRELS" {   // Exclude object fronting in relative clauses
                isRelCl = true;
            }
            if token_deprel == "SUBJ" {
                subjidx = i;    // Fronted OBJ mistaken for SUBJ
            }
        } else if gold_deprel == "SUBJ" {
            gold_subjidx = i;

        } else if gold_lemma == "werden%passiv" {
            isPassive = true;
        }

        if gold_deprel == "-PUNCT-" || i == gold_sent.len()-1 {
            if gold_subjidx > 0 && gold_objidx > 0 && gold_objidx < gold_subjidx && !isRelCl { // Fronted object
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

pub fn n_verb_particles(gold_sent: &[Token], nongold_sent: &[Token]) -> (usize, usize) {

    assert_eq!(gold_sent.len(), nongold_sent.len());

    let mut overall_verb_particles = 0;
    let mut errors = 0;

    for i in 0..gold_sent.len() {

        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let mut gold_head = gold_token.head().expect("No head");
        if gold_head == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            gold_head -= 1;
        }

        let token = &nongold_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");
        let mut token_head = token.head().expect("No head");
        if token_head == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            token_head -= 1;
        }

        if (gold_deprel == "PP" || gold_deprel == "OBJP") && gold_deprel == token_deprel
            && gold_sent[gold_head].pos().expect("No deprel").starts_with("V") { // Head of PP is a verb
            overall_verb_particles += 1;
            if nongold_sent[token_head].pos().expect("No deprel").starts_with("N") {
                errors += 1;
            }
        }
    }
    (overall_verb_particles, errors)
}

fn n_subj_obj_splits(gold_sent: &[Token], nongold_sent: &[Token]) -> (usize, usize) {
    unimplemented!()
}

pub fn n_coordinations(gold_sent: &[Token], nongold_sent: &[Token]) -> (usize, usize) {

    assert_eq!(gold_sent.len(), nongold_sent.len());

    let mut overall_coords = 0;
    let mut errors = 0;

    for i in 0..gold_sent.len() {

        let gold_token = &gold_sent[i];
        let gold_pos = gold_token.pos().expect("No PoS tag");
        let gold_head = gold_token.head().expect("No head");

        let token = &nongold_sent[i];
        let token_pos = token.pos().expect("No PoS tag");
        let token_head = token.head().expect("No head");

        if gold_pos == "KON" && gold_pos == token_pos
            && gold_sent[gold_head].pos().expect("No PoS tag").starts_with("V") { // Head of coordination is a verb
            overall_coords += 1;
            if gold_head != token_head {
                errors += 1;
            }
        }
    }
    (overall_coords, errors)
}

fn n_adjectives(gold_sent: &[Token], nongold_sent: &[Token]) -> (usize, usize) {
    unimplemented!()
}

fn n_pronoun_quants(gold_sent: &[Token], nongold_sent: &[Token]) -> usize {
    unimplemented!()
}