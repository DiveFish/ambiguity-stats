extern crate conllx;

use conllx::Token;

use std::collections::{HashMap, HashSet};

pub fn comp_pp_err_sents(
    input_gold: &Vec<Vec<Token>>,
    input1: &Vec<Vec<Token>>,
    input2: &Vec<Vec<Token>>,
    ud: bool
) -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>) {
    assert_eq!(input1.len(), input2.len());
    assert_eq!(input1.len(), input_gold.len());

    let mut errs_shared = Vec::new();
    let mut errs_parser1 = Vec::new();
    let mut errs_parser2 = Vec::new();

    for sent_idx in 0..input_gold.len() {
        for tok_idx in 0..input_gold[sent_idx].len() {
            let gold_token = input_gold[sent_idx][tok_idx].clone();
            let gold_deprel = gold_token.head_rel().unwrap();
            let gold_pos = gold_token.pos().unwrap();
            let mut gold_head = gold_token.head().expect("No head");
            if gold_head == 0 {
                continue;
            } else {
                gold_head = gold_head - 1;
            }
            let gold_head_deprel = input_gold[sent_idx][gold_head].head_rel().expect("No deprel");
            let gold_head_head= input_gold[sent_idx][gold_head].head().expect("No head");

            let token1 = input1[sent_idx][tok_idx].clone();
            let deprel1 = token1.head_rel().unwrap();
            let mut head1 = token1.head().expect("No head");
            if head1 == 0 {
                continue;
            } else {
                head1 = head1 - 1;
            }
            let head1_deprel = input1[sent_idx][head1].head_rel().expect("No deprel");
            let head1_head = input1[sent_idx][head1].head().expect("No head");

            let token2 = input2[sent_idx][tok_idx].clone();
            let deprel2 = token2.head_rel().unwrap();
            let mut head2 = token2.head().expect("No head");
            if head2 == 0 {
                continue;
            } else {
                head2 = head2 - 1;
            }
            let head2_deprel = input2[sent_idx][head2].head_rel().expect("No deprel");
            let head2_head = input2[sent_idx][head2].head().expect("No head");

            if !ud {
                if gold_deprel == "PP" || gold_deprel == "OBJP" {
                    if (head1 == head2 && head1 != gold_head)
                        || (deprel1 == deprel2 && deprel1 != gold_deprel) {
                        add_err_sentence(&input1[sent_idx], tok_idx, &mut errs_shared);
                    } else if input1[sent_idx][tok_idx] != input_gold[sent_idx][tok_idx] {
                        add_err_sentence(&input1[sent_idx], tok_idx, &mut errs_parser1);
                    } else if input2[sent_idx][tok_idx] != input_gold[sent_idx][tok_idx] {
                        add_err_sentence(&input2[sent_idx], tok_idx, &mut errs_parser2);
                    }
                }
            } else {
                if gold_deprel == "case" &&
                    gold_pos.starts_with("ADP") &&
                    ( gold_head_deprel == "obl" || gold_head_deprel == "nmod" || gold_head_deprel == "root" ) {
                    if (head1 == head2 && head1 != gold_head) ||
                        (deprel1 == deprel2 && deprel1 != gold_deprel) {
                        add_err_sentence(&input1[sent_idx], tok_idx, &mut errs_shared);
                        add_err_sentence(&input_gold[sent_idx], tok_idx, &mut errs_shared);
                    } else if input1[sent_idx][tok_idx] != input_gold[sent_idx][tok_idx] {
                        add_err_sentence(&input1[sent_idx], tok_idx, &mut errs_parser1);
                        add_err_sentence(&input_gold[sent_idx], tok_idx, &mut errs_parser1);
                    } else if input2[sent_idx][tok_idx] != input_gold[sent_idx][tok_idx] {
                        add_err_sentence(&input2[sent_idx], tok_idx, &mut errs_parser2);
                        add_err_sentence(&input_gold[sent_idx], tok_idx, &mut errs_parser2);
                    } else if (head1_head == head2_head && head1_head != gold_head_head) ||
                        (head1_deprel == head2_deprel && head1_deprel != gold_head_deprel) {
                        add_err_sentence(&input1[sent_idx], gold_head, &mut errs_shared);
                        add_err_sentence(&input_gold[sent_idx], gold_head, &mut errs_shared);
                    } else if input1[sent_idx][gold_head] != input_gold[sent_idx][gold_head] {
                        add_err_sentence(&input1[sent_idx], gold_head, &mut errs_parser1);
                        add_err_sentence(&input_gold[sent_idx], gold_head, &mut errs_parser1);
                    } else if input2[sent_idx][gold_head] != input_gold[sent_idx][gold_head] {
                        add_err_sentence(&input2[sent_idx], gold_head, &mut errs_parser2);
                        add_err_sentence(&input_gold[sent_idx], gold_head, &mut errs_parser2);
                    }
                }
            }
        }
    }

    return (errs_shared, errs_parser1, errs_parser2);
}

pub fn comp_inv_err_sents(
    input_gold: &Vec<Vec<Token>>,
    input1: &Vec<Vec<Token>>,
    input2: &Vec<Vec<Token>>
) -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>) {
    assert_eq!(input1.len(), input2.len());
    assert_eq!(input1.len(), input_gold.len());

    let mut errs_shared = Vec::new();
    let mut errs_parser1 = Vec::new();
    let mut errs_parser2 = Vec::new();

    for sent_idx in 0..input_gold.len() {
        let so_errs_parser1 = subj_obj_errs(&input1[sent_idx], &input_gold[sent_idx]);
        let mut so_errs_parser2 = subj_obj_errs(&input2[sent_idx], &input_gold[sent_idx]);

        for subj_obj_idxs in so_errs_parser1.iter() {
            if let Some(subj_obj_idxs) = subj_obj_idxs {
                let (subj, obj) = subj_obj_idxs;
                if so_errs_parser2.contains(&Some(*subj_obj_idxs)) {
                    // Both in dep_inv and pmi_inv
                    add_err_sentence_tok2(&input1[sent_idx], *subj, *obj, &mut errs_shared);
                    add_err_sentence_tok2(&input_gold[sent_idx], *subj, *obj, &mut errs_shared);
                    so_errs_parser2.remove(&Some(*subj_obj_idxs));
                } else {
                    // Only in dep_inv
                    add_err_sentence_tok2(&input1[sent_idx], *subj, *obj, &mut errs_parser1);
                    add_err_sentence_tok2(&input_gold[sent_idx], *subj, *obj, &mut errs_parser1);
                }
            }
        }

        for subj_obj_idxs in so_errs_parser2.iter() {
            // Only in pmi_inv
            if let Some(subj_obj_idxs) = subj_obj_idxs {
                let (subj, obj) = subj_obj_idxs;
                add_err_sentence_tok2(&input2[sent_idx], *subj, *obj, &mut errs_parser2);
                add_err_sentence_tok2(&input_gold[sent_idx], *subj, *obj, &mut errs_parser2);
            }
        }
    }

    return (errs_shared, errs_parser1, errs_parser2);
}

pub fn subj_obj_errs(
    parser_sent: &[Token],
    gold_sent: &[Token],
) -> HashSet<Option<(usize, usize)>> {
    let mut head_verb_args = HashMap::new();

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_head = gold_token.head().expect("No head");

        let token = &parser_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_deprel == "SUBJ" || gold_deprel.starts_with("OBJ") || gold_deprel == "nsubj" || gold_deprel == "obj" {

            let mut verb_idx;
            if (gold_head > 0)
                && gold_sent[gold_head - 1]
                    .head_rel()
                    .expect("No deprel")
                    .eq("AUX")
            {
                // Reattach auxiliary verb to content verb
                verb_idx = gold_sent[gold_head - 1].head().expect("No head");
            } else {
                verb_idx = gold_head;
            }

            if gold_deprel == "SUBJ" || gold_deprel == "nsubj" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[0] = i;

                if token_deprel.starts_with("OBJ") || token_deprel == "obj" {
                    entry[1] = i; // SUBJ mistaken for OBJ
                }
            } else if gold_deprel.starts_with("OBJ") || gold_deprel == "obj" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[2] = i;

                if token_deprel == "SUBJ" || token_deprel == "nsubj" {
                    entry[3] = i; // OBJ mistaken for SUBJ
                }
            }
        }
    }

    let mut subj_obj_idxs = HashSet::new();
    for (_, val) in head_verb_args.iter() {
        let gold_subjidx = val[0];
        let parser_objidx = val[1]; // OBJ but should have been SUBJ
        let gold_objidx = val[2];
        let parser_subjidx = val[3]; // SUBJ but should have been OBJ

        // A clause with subj and obj // where obj precedes subj
        if gold_subjidx > 0 && gold_objidx > 0 { // && gold_subjidx > gold_objidx { // Inversion errors
            if parser_objidx > 0 || parser_subjidx > 0 {
                subj_obj_idxs.insert(Some((gold_subjidx, gold_objidx)));
            }
        }
    }
    subj_obj_idxs.clone()
}

fn add_err_sentence(input_sent: &Vec<Token>, tok_idx: usize, err_sents: &mut Vec<Vec<String>>) {
    let mut sent = Vec::with_capacity(input_sent.len());
    for idx in 0..input_sent.len() {
        let token = input_sent[idx].clone();
        let form = token.form();
        let head = token.head().unwrap();
        let deprel = token.head_rel().unwrap();

        if idx == tok_idx {
            let token_str = format!("*{}_{}_{}*", form, head, deprel);
            sent.push(token_str);
        } else {
            let token_str = format!("{}_{}_{}", form, head, deprel);
            sent.push(token_str);
        }
    }
    err_sents.push(sent);
}

fn add_err_sentence_tok2(
    input_sent: &Vec<Token>,
    tok_idx: usize,
    tok_idx2: usize,
    err_sents: &mut Vec<Vec<String>>,
) {
    let mut sent = Vec::with_capacity(input_sent.len());
    for idx in 0..input_sent.len() {
        let token = input_sent[idx].clone();
        let form = token.form();
        let head = token.head().unwrap();
        let deprel = token.head_rel().unwrap();

        if idx == tok_idx {
            let token_str = format!("*{}_{}_{}*", form, head, deprel);
            sent.push(token_str);
        } else if idx == tok_idx2 {
            let token_str = format!("*{}_{}_{}*", form, head, deprel);
            sent.push(token_str);
        } else {
            let token_str = format!("{}_{}_{}", form, head, deprel);
            sent.push(token_str);
        }
    }
    err_sents.push(sent);
}

/// Get all parser errors for particular dependency relations.
/// Count erroneous labels to calculate LAS.
pub fn get_errors_by_labels(
    label: &str,
    gold_sent: &[Token],
    parser_sent: &[Token],
) -> (usize, usize, usize, usize, HashMap<String, usize>) {

    assert_eq!(gold_sent.len(), parser_sent.len());

    let mut attachments = 0;
    let mut combined_errors = 0;
    let mut head_errors = 0;
    let mut label_errors = 0;

    let mut wrong_labels: HashMap<String, usize> = HashMap::new();

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        if &gold_deprel == &label {
            attachments += 1;

            let gold_headidx = gold_token.head().expect("No head");

            let parser_token = &parser_sent[i];
            let parser_headidx = parser_token.head().expect("No head idx");
            let parser_deprel = parser_token.head_rel().expect("No deprel");

            if gold_headidx != parser_headidx && gold_deprel != parser_deprel {
                combined_errors += 1;
                *wrong_labels.entry(parser_deprel.to_string()).or_insert(0) += 1;
            } else if gold_headidx != parser_headidx {
                head_errors += 1;
            } else if gold_deprel != parser_deprel {
                label_errors += 1;
                *wrong_labels.entry(parser_deprel.to_string()).or_insert(0) += 1;
            }
        }
    }

    (
        attachments,
        combined_errors,
        head_errors,
        label_errors,
        wrong_labels,
    )
}
