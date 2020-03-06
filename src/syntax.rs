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

pub fn sentence_generator(svo_triples: &Vec<Vec<String>>, templates: &Vec<Vec<String>>, templates_mod: &Vec<Vec<String>>) -> io::Result<()>  {
    eprintln!("Saving to directory /home/patricia/Dokumente/Promotion/Daten/challenge-set/dataset/test/");
    let mut f_svo = File::create("/home/patricia/Dokumente/Promotion/Daten/challenge-set/dataset/test/dataset_svo.conll")?;
    let mut f_ovs = File::create("/home/patricia/Dokumente/Promotion/Daten/challenge-set/dataset/test/dataset_ovs.conll")?;
    let mut f_dvso = File::create("/home/patricia/Dokumente/Promotion/Daten/challenge-set/dataset/test/dataset_dvso.conll")?;
    let mut f_dvos = File::create("/home/patricia/Dokumente/Promotion/Daten/challenge-set/dataset/test/dataset_dvos.conll")?;
    let mut f_wsov = File::create("/home/patricia/Dokumente/Promotion/Daten/challenge-set/dataset/test/dataset_wsov.conll")?;
    let mut f_wosv = File::create("/home/patricia/Dokumente/Promotion/Daten/challenge-set/dataset/test/dataset_wosv.conll")?;

    for svo_triple in svo_triples {
        let mut dative = false;
        if svo_triple[0] == "+d" || svo_triple[0] == "*d" || svo_triple[0] == "?d" {
            dative = true;
        };

        if svo_triple.len() == 5 {
            for template_mod in templates_mod {

                if template_mod[0] == "S" {
                    sent_to_conll_gold(template_mod, svo_triple, dative, &mut f_svo);
                } else if template_mod[0] == "O" {
                    sent_to_conll_gold(template_mod, svo_triple, dative, &mut f_ovs);
                } else if template_mod[0] == "Deshalb" && template_mod[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_mod, svo_triple, dative, &mut f_dvos);
                } else if template_mod[0] == "Deshalb" && template_mod[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_mod, svo_triple, dative, &mut f_dvso);
                } else if template_mod[0] == "Weil" && template_mod[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_mod, svo_triple, dative, &mut f_wosv);
                } else if template_mod[0] == "Weil" && template_mod[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_mod, svo_triple, dative, &mut f_dvso);
                }
            }
        } else if svo_triple.len() == 4 {
            for template in templates {

                if template[0] == "S" {
                    sent_to_conll_gold(template, svo_triple, dative, &mut f_svo);
                } else if template[0] == "O" {
                    sent_to_conll_gold(template, svo_triple, dative, &mut f_ovs);
                } else if template[0] == "Deshalb" && template[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, dative, &mut f_dvos);
                } else if template[0] == "Deshalb" && template[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, dative, &mut f_dvso);
                } else if template[0] == "Weil" && template[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, dative, &mut f_wosv);
                } else if template[0] == "Weil" && template[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, dative, &mut f_wsov);
                }
            }
        } else {
            eprintln!("Triple length {} not supported.", svo_triple.len());
        }
    }
    Ok(())
}

fn sent_to_conll_gold(template: &Vec<String>, svo_triple: &Vec<String>, dative: bool, file: &mut File) {

    let mut conll_idx = 0;
    for idx in 0..template.len() {
        let mut token = &template[idx];
        if token == "S" {
            let mut tokens = svo_triple[1].split(" ").collect::<Vec<_>>();
            if tokens.len() > 1 {
                if idx == 0 {
                    writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\t_\t_\t_", conll_idx, uppercase_first_letter(&tokens[0]));
                } else {
                    writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\t_\t_\t_", conll_idx, &tokens[0]);
                }
                conll_idx += 1;
                writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\tnsubj\t_\t_", conll_idx, &tokens[1]);
            } else {
                if idx == 0 {
                    writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\tnsubj\t_\t_", conll_idx, uppercase_first_letter(&tokens[0]));
                } else {
                    writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\tnsubj\t_\t_", conll_idx, &tokens[0]);
                }
            }
        } else if token == "V" {
            if idx == 0 {
                writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\tverb\t_\t_", conll_idx, uppercase_first_letter(&svo_triple[2]));
            } else {
                writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\tverb\t_\t_", conll_idx, svo_triple[2]);
            }
        } else if token == "O" {
            let mut tokens = svo_triple[3].split(" ").collect::<Vec<_>>();
            let deprel = if dative {
                "iobj"
            } else {
                "obj"
            }.to_string();
            if tokens.len() > 1 {
                if idx == 0 {
                    writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\t_\t_\t_", conll_idx, uppercase_first_letter(&tokens[0]));
                } else {
                    writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\t_\t_\t_", conll_idx, &tokens[0]);
                }
                conll_idx += 1;
                writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\t{}\t_\t_", conll_idx, &tokens[1], deprel);
            } else {
                if idx == 0 {
                    writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\t{}\t_\t_", conll_idx, uppercase_first_letter(&tokens[0]), deprel);
                } else {
                    writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\t{}\t_\t_", conll_idx, &tokens[0], deprel);
                }
            }
        } else if token == "VAUX" {
            if idx == 0 {
                writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\taux\t_\t_", conll_idx, uppercase_first_letter(&svo_triple[4]));
            } else {
                writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\taux\t_\t_", conll_idx, svo_triple[4]);
            }
        } else {
            if idx == 0 {
                writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\t_\t_\t_", conll_idx, uppercase_first_letter(&token));
            } else {
                writeln!(file, "{}\t{}\t_\t_\t_\t_\t_\t_\t_\t_", conll_idx, token);
            }
        }
        conll_idx += 1;
    }
    writeln!(file, "{}\t.\t_\t_\t_\t_\t_\tpunct\t_\t_\n", conll_idx);
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