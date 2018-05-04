extern crate conllx;

use comps::heads_equal;
use comps::deprels_equal;

use conllx::Token;

pub fn pp_attachment(gold_sent: &Vec<Token>, nongold_sent: &Vec<Token>) -> usize {
    let mut errors = 0;
    let mut idx = 0;
    for i in 0..gold_sent.len() {

        // @Daniël: Are some sentences of different sizes in gold and non-gold data?
        if gold_sent.len() > i && nongold_sent.len() > i {
            let gold_token = &gold_sent[i];
            //@Daniël: How to avoid expect() everywhere an Option is returned? Or how to make it more elegant?
            let gold_head_idx = gold_token.head().expect("No head");
            let gold_pos = token.pos().expect("No deprel");

            let token = &nongold_sent[i];
            let head_idx = token.head().expect("No head idx");
            let head = &nongold_sent[head_idx];

            if (gold_pos == "PREP") && deprels_equal(&token, &gold_token)
                && !heads_equal(&token, &gold_token) {
                errors += 1;
            }
            idx += 1;
        }
        /*
        println!("{:?}", &gold_sent[i]);
        */
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