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

pub fn sentence_generator(svo_triples: &Vec<Vec<String>>, properties: &Vec<String>, templates: &Vec<Vec<String>>, templates_mod: &Vec<Vec<String>>, filename: &str) -> io::Result<()>  {
    let mut file = File::create(filename)?;

    for (svo_triple, property) in svo_triples.iter().zip(properties.iter()) {

        if svo_triple.len() == 5 {
            for template_mod in templates_mod {

                if template_mod[0] == "S" {
                    sent_to_conll_gold(template_mod, svo_triple, "svo", property, &mut file);
                } else if template_mod[0] == "O" {
                    sent_to_conll_gold(template_mod, svo_triple, "ovs", property, &mut file);
                } else if template_mod[0] == "Deshalb" && template_mod[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_mod, svo_triple, "dvos", property, &mut file);
                } else if template_mod[0] == "Deshalb" && template_mod[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_mod, svo_triple, "dvso", property, &mut file);
                } else if template_mod[0] == "Weil" && template_mod[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_mod, svo_triple, "wosv", property, &mut file);
                } else if template_mod[0] == "Weil" && template_mod[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_mod, svo_triple, "wsov", property, &mut file);
                }
            }
        } else if svo_triple.len() == 4 {
            for template in templates {

                if template[0] == "S" {
                    sent_to_conll_gold(template, svo_triple, "svo", property, &mut file);
                } else if template[0] == "O" {
                    sent_to_conll_gold(template, svo_triple, "ovs", property, &mut file);
                } else if template[0] == "Deshalb" && template[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "dvos", property, &mut file);
                } else if template[0] == "Deshalb" && template[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "dvso", property, &mut file);
                } else if template[0] == "Weil" && template[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "wosv", property, &mut file);
                } else if template[0] == "Weil" && template[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "wsov", property, &mut file);
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

    let mut conll_idx = 0;
    for templ_idx in 0..template.len() {
        let mut token = &template[templ_idx];
        if token == "S" {
            let mut tokens = svo_triple[1].split(" ").collect::<Vec<_>>();
            for subj_idx in 0..tokens.len() {
                if subj_idx == tokens.len() - 1 {
                    if templ_idx == 0 {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\tnsubj\t_\t_", conll_idx, uppercase_first_letter(&tokens[subj_idx]), order, property);
                    } else {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\tnsubj\t_\t_", conll_idx, &tokens[subj_idx], order, property);
                    }
                    continue;
                } else {
                    if templ_idx == 0 {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx, uppercase_first_letter(&tokens[subj_idx]), order, property);
                    } else {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx, &tokens[subj_idx], order, property);
                    }
                    if tokens.len() > 1 {
                        conll_idx += 1;
                    }
                }
            }
        } else if token == "V" {
            if templ_idx == 0 {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\tverb\t_\t_", conll_idx, uppercase_first_letter(&svo_triple[2]), order, property);
            } else {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\tverb\t_\t_", conll_idx, svo_triple[2], order, property);
            }
        } else if token == "O" {
            let mut tokens = svo_triple[3].split(" ").collect::<Vec<_>>();
            for obj_idx in 0..tokens.len() {
                if obj_idx == tokens.len() - 1 {
                    if templ_idx == 0 {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\tobj\t_\t_", conll_idx, uppercase_first_letter(&tokens[obj_idx]), order, property);
                    } else {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\tobj\t_\t_", conll_idx, &tokens[obj_idx], order, property);
                    }
                    continue;
                } else {
                    if templ_idx == 0 {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx, uppercase_first_letter(&tokens[obj_idx]), order, property);
                    } else {
                        writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx, &tokens[obj_idx], order, property);
                    }
                    if tokens.len() > 1 {
                        conll_idx += 1;
                    }
                }
            }
        } else if token == "VAUX" {
            if templ_idx == 0 {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\taux\t_\t_", conll_idx, uppercase_first_letter(&svo_triple[4]), order, property);
            } else {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\taux\t_\t_", conll_idx, svo_triple[4], order, property);
            }
        } else {
            if templ_idx == 0 {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx, uppercase_first_letter(&token), order, property);
            } else {
                writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx, token, order, property);
            }
        }
        conll_idx += 1;
    }
    writeln!(file, "{}\t.\t_\t_\t_\torder:{}|props:{}\t_\tpunct\t_\t_\n", conll_idx, order, property);
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