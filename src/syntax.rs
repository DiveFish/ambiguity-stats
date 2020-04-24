extern crate conllx;
extern crate linked_hash_map;

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use conllx::Token;
use linked_hash_map::LinkedHashMap;

/// Collect all possible combinations of PoS tags (or dependency relations) to find different
/// syntactic patterns.
pub fn label_combos(input: &[Token], pos_patterns: &mut LinkedHashMap<Vec<String>, usize>, sent_examples: &mut Vec<Vec<String>>, inv: bool) {
    let mut sent = Vec::with_capacity(input.len());
    let mut sent_pos = Vec::with_capacity(input.len());
    let mut subj = false;
    let mut obj = false;
    for i in 0..input.len() {
        sent.push(input[i].form().to_string());
        let deprel = input[i].head_rel().expect("No deprel");
        if ! (deprel == "-PUNCT-") {    //punct
            sent_pos.push(deprel.to_string());
        }

        if deprel == "SUBJ" {   //nsubj
            subj = true;
        } else if deprel.starts_with("OBJ") && !subj {    // For UD: ("obj") || deprel == "iobj")
            obj = true;
        }
    }

    if inv && subj && obj {
        if !pos_patterns.contains_key(&sent_pos) {
            sent_examples.push(sent);
        }
        *pos_patterns.entry(sent_pos).or_insert(0) += 1;
    }
}

pub fn sentence_generator(
    svo_triples: &Vec<Vec<String>>,
    properties: &Vec<String>,
    templates: &Vec<Vec<String>>,
    templates_aux: &Vec<Vec<String>>,
    v1_intro: &str,
    vl_intro: &str,
    filename: &str
) -> io::Result<()>  {
    let mut file = File::create(filename)?;

    for (svo_triple, property) in svo_triples.iter().zip(properties.iter()) {

        if svo_triple.len() == 5 {
            for template_aux in templates_aux {

                if template_aux[0] == "S" {
                    sent_to_conll_gold(template_aux, svo_triple, "VF[S]LK[V]MF[O]", property, &mut file);
                } else if template_aux[0] == "O" {
                    sent_to_conll_gold(template_aux, svo_triple, "VF[O]LK[V]MF[S]", property, &mut file);
                } else if template_aux[0] == v1_intro && template_aux[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "LK[V]MF[SO]", property, &mut file);
                } else if template_aux[0] == v1_intro && template_aux[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "LK[V]MF[OS]", property, &mut file);
                } else if template_aux[0] == "VAUX" && template_aux[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "LK[V]MF[SO]Q", property, &mut file);
                } else if template_aux[0] == "VAUX" && template_aux[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "LK[V]MF[OS]Q", property, &mut file);
                } else if template_aux[0] == vl_intro && template_aux[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "MF[SO]VC[V]", property, &mut file);
                } else if template_aux[0] == vl_intro && template_aux[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "MF[OS]VC[V]", property, &mut file);
                }
            }
        } else if svo_triple.len() == 4 {
            for template in templates {

                if template[0] == "S" {
                    sent_to_conll_gold(template, svo_triple, "VF[S]LK[V]MF[O]", property, &mut file);
                } else if template[0] == "O" {
                    sent_to_conll_gold(template, svo_triple, "VF[O]LK[V]MF[S]", property, &mut file);
                } else if template[0] == v1_intro && template[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "LK[V]MF[SO]", property, &mut file);
                } else if template[0] == v1_intro && template[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "LK[V]MF[OS]", property, &mut file);
                } else if template[0] == "V" && template[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "LK[V]MF[SO]Q", property, &mut file);
                } else if template[0] == "V" && template[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "LK[V]MF[OS]Q", property, &mut file);
                } else if template[0] == vl_intro && template[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "MF[SO]VC[V]", property, &mut file);
                } else if template[0] == vl_intro && template[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "MF[OS]VC[V]", property, &mut file);
                }
            }
        } else {
            eprintln!("Triple length {} not supported.", svo_triple.len());
        }
    }
    Ok(())
}

/// Not working for sentence 'frühe Zucht bringt keine gute Frucht'
fn sent_to_conll_gold(template: &Vec<String>, svo_triple: &Vec<String>, order: &str, property: &str, file: &mut File) {
    let subj_len = svo_triple[1].split(" ").collect::<Vec<_>>().len();
    let obj_len = svo_triple[3].split(" ").collect::<Vec<_>>().len();
    let aux_len = if svo_triple.len() == 5 {
        1
    } else {
        0
    };
    let mut v_head = if svo_triple.len() == 5 {  // With auxiliary
        match order {
            "LK[V]MF[SO]" => 2 + subj_len + obj_len + 1,   // deshalb vso
            "LK[V]MF[OS]" => 2 + obj_len + subj_len + 1,   // deshalb vos
            "LK[V]MF[SO]Q" => 1 + subj_len + obj_len + 1,   // vso ?
            "LK[V]MF[OS]Q" => 1 + obj_len + subj_len + 1,   // vos ?
            "VF[S]LK[V]MF[O]" => subj_len + 1 + obj_len + 1,   // svo
            "VF[O]LK[V]MF[S]" => obj_len + 1 + subj_len + 1,   // ovs
            "MF[SO]VC[V]" => 1 + subj_len + obj_len + 1,   // weil sov
            "MF[OS]VC[V]" => 1 + obj_len + subj_len + 1,   // weil osv
            _ => 0
        }
    } else {
        match order {  // No auxiliary
            "LK[V]MF[SO]" => 2,
            "LK[V]MF[OS]" => 2,
            "LK[V]MF[SO]Q" => 1,
            "LK[V]MF[OS]Q" => 1,
            "VF[S]LK[V]MF[O]" => subj_len + 1,
            "VF[O]LK[V]MF[S]" => obj_len + 1,
            "MF[SO]VC[V]" => 1 + subj_len + obj_len + 1,
            "MF[OS]VC[V]" =>1 + obj_len + subj_len + 1,
            _ => 0
        }
    };

    let mut conll_idx = 0;
    for templ_idx in 0..template.len() {
        let mut token = &template[templ_idx];
        if token == "S" {
            let mut tokens_inner = svo_triple[1].split(" ").collect::<Vec<_>>();
            for subj_idx in 0..tokens_inner.len() {
                if subj_idx == tokens_inner.len() - 1 {
                    if conll_idx == 0 {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t{}\tnsubj\t_\t_", conll_idx + 1, uppercase_first_letter(&tokens_inner[subj_idx]), order, property, v_head);
                    } else {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t{}\tnsubj\t_\t_", conll_idx + 1, &tokens_inner[subj_idx], order, property, v_head);
                    }
                    continue;
                } else {
                    if conll_idx == 0 {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, uppercase_first_letter(&tokens_inner[subj_idx]), order, property);
                    } else {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, &tokens_inner[subj_idx], order, property);
                    }
                    if tokens_inner.len() > 1 {
                        conll_idx += 1;
                    }
                }
            }
        } else if token == "V" {
            if conll_idx == 0 {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t0\tverb\t_\t_", conll_idx + 1, uppercase_first_letter(&svo_triple[2]), order, property);
            } else {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t0\tverb\t_\t_", conll_idx + 1, svo_triple[2], order, property);
            }
        } else if token == "O" {
            let mut tokens_inner = svo_triple[3].split(" ").collect::<Vec<_>>();
            for obj_idx in 0..tokens_inner.len() {
                if obj_idx == tokens_inner.len() - 1 {
                    if conll_idx == 0 {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t{}\tobj\t_\t_", conll_idx + 1, uppercase_first_letter(&tokens_inner[obj_idx]), order, property, v_head);
                    } else {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t{}\tobj\t_\t_", conll_idx + 1, &tokens_inner[obj_idx], order, property, v_head);
                    }
                    continue;
                } else {
                    if conll_idx == 0 {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, uppercase_first_letter(&tokens_inner[obj_idx]), order, property);
                    } else {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, &tokens_inner[obj_idx], order, property);
                    }
                    if tokens_inner.len() > 1 {
                        conll_idx += 1;
                    }
                }
            }
        } else if token == "VAUX" {
            if conll_idx == 0 {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, uppercase_first_letter(&svo_triple[4]), order, property);
            } else {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, svo_triple[4], order, property);
            }
        } else if token != "?" {
            if conll_idx == 0 {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, uppercase_first_letter(&token), order, property);
            } else {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, token, order, property);
            }
        }
        conll_idx += 1;
    }
    if template[template.len() - 1] == "?" {
        writeln!(file, "{}\t?\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_\n", conll_idx + 1, order, property);
    } else {
        writeln!(file, "{}\t.\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_\n", conll_idx + 1, order, property);
    }
}

fn is_aux(verb: &str) -> bool {
    match verb {
        "werde" => true,
        "wirst" => true,
        "wird" => true,
        "werden" => true,
        "werdet" => true,
        _ => false
    }
}

fn sent_to_conll(template: &Vec<String>, svo_triple: &Vec<String>, file: &mut File) {

    let mut conll_idx = 0;
    for idx in 0..template.len() {

        let mut token = &template[idx];

        if token == "S" {
            let mut tokens = svo_triple[1].split(" ").collect::<Vec<_>>();
            for tok_idx in 0..tokens.len() {
                if idx == 0 && tok_idx == 0 {
                    writeln!(file, "{}\t{}", conll_idx, uppercase_first_letter(&tokens[tok_idx]));
                    conll_idx += 1;
                } else {
                    writeln!(file, "{}\t{}", conll_idx, tokens[tok_idx]);
                    conll_idx += 1;
                }
            }
        } else if token == "V" {
            if idx == 0 {
                writeln!(file, "{}\t{}", conll_idx, uppercase_first_letter(&svo_triple[2]));
                conll_idx += 1;
            } else {
                writeln!(file, "{}\t{}", conll_idx, svo_triple[2]);
                conll_idx += 1;
            }
        } else if token == "O" {
            let mut tokens = svo_triple[3].split(" ").collect::<Vec<_>>();
            for tok_idx in 0..tokens.len() {
                if idx == 0 && tok_idx == 0 {
                    writeln!(file, "{}\t{}", conll_idx, uppercase_first_letter(&tokens[tok_idx]));
                    conll_idx += 1;
                } else {
                    writeln!(file, "{}\t{}", conll_idx, tokens[tok_idx]);
                    conll_idx += 1;
                }
            }
        } else if token == "VAUX" {
            if idx == 0 {
                writeln!(file, "{}\t{}", conll_idx, uppercase_first_letter(&svo_triple[4]));
                conll_idx += 1;
            } else {
                writeln!(file, "{}\t{}", conll_idx, svo_triple[4]);
                conll_idx += 1;
            }
        } else {
            if idx == 0 {
                writeln!(file, "{}\t{}", conll_idx, uppercase_first_letter(&token));
                conll_idx += 1;
            } else {
                writeln!(file, "{}\t{}", conll_idx, token);
                conll_idx += 1;
            }
        }
    }
    writeln!(file, "{}\t.\n", conll_idx);
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}