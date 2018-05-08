extern crate conllx;
use conllx::Token;

pub fn get_ambiguity_counts(gold_sent: &[Token], nongold_sent: &[Token], fun: fn(&mut usize, &mut usize, &[Token], &[Token])) -> (usize, usize) {
    assert_eq!(gold_sent.len(), nongold_sent.len());

    let mut overall_occurrences:usize = 0;
    let mut errors:usize = 0;
    fun(&mut overall_occurrences, &mut errors, gold_sent, nongold_sent);
    (overall_occurrences, errors)
}

pub fn n_pp_attachments(overall_pps: &mut usize, errors: &mut usize, gold_sent: &[Token], nongold_sent: &[Token]) {

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
        let deprel = token.head_rel().expect("No deprel");

        if (gold_deprel == "PP") && gold_deprel == deprel {
            *overall_pps += 1;
            if gold_head_idx != head_idx {
                *errors += 1;
            }
        }
    }
}

pub fn n_pp_objps(overall_pps: &mut usize, errors: &mut usize, gold_sent: &[Token], nongold_sent: &[Token]) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let token = &nongold_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_deprel == "PP" || gold_deprel == "OBJP" {
            *overall_pps += 1;
            if gold_deprel == "PP" && token_deprel == "OBJP" {
                *errors += 1;
            } else if gold_deprel == "OBJP" && token_deprel == "PP" {
                *errors += 1;
            }
        }
    }
}

pub fn n_obj_frontings(overall_frontedobjs: &mut usize, errors: &mut usize, gold_sent: &[Token], nongold_sent: &[Token]) {

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

        } else if gold_lemma == "werden%passiv" {   // Exclude passives - currently not used
            isPassive = true;
        }

        if gold_deprel == "-PUNCT-" || i == gold_sent.len()-1 {
            if gold_subjidx > 0 && gold_objidx > 0 && gold_objidx < gold_subjidx && !isRelCl { // Fronted object
                *overall_frontedobjs += 1;
                if subjidx > 0 {
                    *errors += 1;
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
}

pub fn n_verb_particles(overall_verb_particles: &mut usize, errors: &mut usize, gold_sent: &[Token], nongold_sent: &[Token]) {

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
            *overall_verb_particles += 1;
            if nongold_sent[token_head].pos().expect("No deprel").starts_with("N") {
                *errors += 1;
            }
        }
    }
}

pub fn n_subj_obj_splits(overall_subj_objs_separations: &mut usize, errors: &mut usize, gold_sent: &[Token], nongold_sent: &[Token]) {

    let mut gold_subjidx = 0;
    let mut gold_objidx = 0;
    let mut objidx = 0;

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let mut gold_head = gold_token.head().expect("No head");
        if gold_head == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            gold_head -= 1;
        }
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let token = &nongold_sent[i];
        let mut token_head = token.head().expect("No head");
        if token_head == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            token_head -= 1;
        }
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_deprel == "SUBJ" && token.pos().expect("No PoS tag").starts_with("N") {
            gold_subjidx = i;
            if token_deprel.starts_with("OBJ") {
                objidx = i;
            }
        } else if gold_deprel == "OBJA" || gold_deprel == "OBJD"
            && token.pos().expect("No PoS tag").starts_with("N") {
            gold_objidx = i;
        }

        if gold_subjidx > 0 && gold_objidx > 0 && gold_objidx == (gold_subjidx+1) {
            *overall_subj_objs_separations += 1;
            if objidx > 0 && objidx != gold_objidx {
                *errors += 1;
            }
            gold_subjidx = 0;
            gold_objidx = 0;
            objidx = 0;
        }
    }
}

//TODO: Correct head indices
pub fn n_coordinations(overall_coords: &mut usize, errors: &mut usize, gold_sent: &[Token], nongold_sent: &[Token]) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_pos = gold_token.pos().expect("No PoS tag");
        let gold_head = gold_token.head().expect("No head");

        let token = &nongold_sent[i];
        let token_pos = token.pos().expect("No PoS tag");
        let token_head = token.head().expect("No head");

        if gold_pos == "KON" && gold_pos == token_pos
            && gold_sent[gold_head].pos().expect("No PoS tag").starts_with("V") { // Head of coordination is a verb
            *overall_coords += 1;
            if gold_head != token_head {
                *errors += 1;
            }
        }
    }
}

pub fn n_adjectives(overall_adjs: &mut usize, errors: &mut usize, gold_sent: &[Token], nongold_sent: &[Token]) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_pos = gold_token.pos().expect("No PoS tag");
        let mut gold_headidx = gold_token.head().expect("No head");
        if gold_headidx == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            gold_headidx -= 1;
        }
        let gold_headpos = gold_sent[gold_headidx].pos().expect("No PoS tag");

        let token = &nongold_sent[i];
        let token_pos = token.pos().expect("No PoS tag");
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_pos == "PWAV" && gold_headpos == "ADJD" { //|| (gold_pos == "ADJA" && gold_headpos.starts_with("N")) {
            *overall_adjs += 1;
            if token_pos == "PWAV" {
                *errors += 1;
            }
        }
    }
}

fn n_pronoun_quants(overall_prons: &mut usize, errors: &mut usize, gold_sent: &[Token], nongold_sent: &[Token]) {
    unimplemented!()
}