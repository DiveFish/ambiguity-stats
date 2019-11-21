extern crate conllx;

use conllx::Token;

use std::collections::{HashMap, HashSet};

pub fn comp_pp_err_sents(
    input_dep: &Vec<Vec<Token>>,
    input_pmi: &Vec<Vec<Token>>,
    gold: &Vec<Vec<Token>>,
) -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>) {
    assert_eq!(input_dep.len(), input_pmi.len());
    assert_eq!(input_dep.len(), gold.len());

    let mut errs_shared = Vec::new();
    let mut dep_errs = Vec::new();
    let mut pmi_errs = Vec::new();

    for sent_idx in 0..gold.len() {
        for tok_idx in 0..gold[sent_idx].len() {
            let gold_token = gold[sent_idx][tok_idx].clone();
            let gold_head = gold_token.head().unwrap();
            let gold_deprel = gold_token.head_rel().unwrap();
            let dep_token = input_dep[sent_idx][tok_idx].clone();
            let dep_head = dep_token.head().unwrap();
            let dep_deprel = dep_token.head_rel().unwrap();
            let pmi_token = input_pmi[sent_idx][tok_idx].clone();
            let pmi_head = pmi_token.head().unwrap();
            let pmi_deprel = pmi_token.head_rel().unwrap();

            if gold_deprel == "PP" || gold_deprel == "OBJP" {
                if (dep_head == pmi_head && dep_head != gold_head)
                    || (dep_deprel == pmi_deprel && dep_deprel != gold_deprel)
                {
                    add_err_sentence(&input_dep[sent_idx], tok_idx, &mut errs_shared);
                } else if input_dep[sent_idx][tok_idx] != gold[sent_idx][tok_idx] {
                    add_err_sentence(&input_dep[sent_idx], tok_idx, &mut dep_errs);
                } else if input_pmi[sent_idx][tok_idx] != gold[sent_idx][tok_idx] {
                    add_err_sentence(&input_pmi[sent_idx], tok_idx, &mut pmi_errs);
                }
            }
        }
    }

    return (errs_shared, dep_errs, pmi_errs);
}

pub fn comp_inv_err_sents(
    input_dep: &Vec<Vec<Token>>,
    input_pmi: &Vec<Vec<Token>>,
    gold: &Vec<Vec<Token>>,
) -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>) {
    assert_eq!(input_dep.len(), input_pmi.len());
    assert_eq!(input_dep.len(), gold.len());

    let mut errs_shared = Vec::new();
    let mut dep_errs = Vec::new();
    let mut pmi_errs = Vec::new();

    for sent_idx in 0..gold.len() {
        let dep_inv = inversion_errs(&gold[sent_idx], &input_dep[sent_idx]);
        let mut pmi_inv = inversion_errs(&gold[sent_idx], &input_pmi[sent_idx]);

        for subj_obj_idxs in dep_inv.iter() {
            if let Some(subj_obj_idxs) = subj_obj_idxs {
                let (subj, obj) = subj_obj_idxs;
                if pmi_inv.contains(&Some(*subj_obj_idxs)) {
                    // Both in dep_inv and pmi_inv
                    add_err_sentence_tok2(&input_dep[sent_idx], *subj, *obj, &mut errs_shared);
                    pmi_inv.remove(&Some(*subj_obj_idxs));
                } else {
                    // Only in dep_inv
                    add_err_sentence_tok2(&input_dep[sent_idx], *subj, *obj, &mut dep_errs);
                }
            }
        }

        for subj_obj_idxs in pmi_inv.iter() {
            // Only in pmi_inv
            if let Some(subj_obj_idxs) = subj_obj_idxs {
                let (subj, obj) = subj_obj_idxs;
                add_err_sentence_tok2(&input_pmi[sent_idx], *subj, *obj, &mut pmi_errs);
            }
        }
    }

    return (errs_shared, dep_errs, pmi_errs);
}

pub fn inversion_errs(
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

        if gold_deprel == "SUBJ" || gold_deprel.starts_with("OBJ") {
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

            if gold_deprel == "SUBJ" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[0] = i;

                if token_deprel.starts_with("OBJ") {
                    entry[1] = i; // SUBJ mistaken for OBJ
                }
            } else if gold_deprel.starts_with("OBJ") {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[2] = i;

                if token_deprel == "SUBJ" {
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

        // A clause with subj and obj where obj precedes subj
        if gold_subjidx > 0 && gold_objidx > 0 && gold_subjidx > gold_objidx {
            // Inversion errors
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
