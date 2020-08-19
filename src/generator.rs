extern crate conllx;

use conllx::{Features,Token};

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;


pub fn sentence_generator(
    svo_triples: &Vec<Vec<String>>,
    properties: &Vec<String>,
    templates: &Vec<Vec<String>>,
    templates_aux: &Vec<Vec<String>>,
    templates_pp: &Vec<Vec<String>>,
    templates_aux_pp: &Vec<Vec<String>>,
    v1_intro: &str,
    vl_intro: &str,
    filename: &str
) -> io::Result<()>  {
    let mut file = File::create(filename)?;

    for (svo_triple, property) in svo_triples.iter().zip(properties.iter()) {

        let aux = if svo_triple.len() > 4 {
            if  is_aux(&svo_triple[4]) {
                true
            } else {
                false
            }
        } else {
            false
        };
        if svo_triple.len() == 6 {    // Auxiliary and PP
            for template_aux_pp in templates_aux_pp {
                if template_aux_pp[0] == "S" {
                    sent_to_conll_gold(template_aux_pp, svo_triple, "VF[S]LK[V]MF[O]", property, aux, &mut file);
                } else if template_aux_pp[0] == "O" {
                    sent_to_conll_gold(template_aux_pp, svo_triple, "VF[O]LK[V]MF[S]", property, aux, &mut file);
                } else if template_aux_pp[0] == v1_intro && template_aux_pp[3] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux_pp, svo_triple, "LK[V]MF[SO]", property, aux, &mut file);
                } else if template_aux_pp[0] == v1_intro && template_aux_pp[3] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux_pp, svo_triple, "LK[V]MF[OS]", property, aux, &mut file);
                } else if template_aux_pp[0] == "VAUX" && template_aux_pp[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux_pp, svo_triple, "LK[V]MF[SO]Q", property, aux, &mut file);
                } else if template_aux_pp[0] == "VAUX" && template_aux_pp[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux_pp, svo_triple, "LK[V]MF[OS]Q", property, aux, &mut file);
                } else if template_aux_pp[0] == vl_intro && template_aux_pp[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux_pp, svo_triple, "MF[SO]VC[V]", property, aux, &mut file);
                } else if template_aux_pp[0] == vl_intro && template_aux_pp[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux_pp, svo_triple, "MF[OS]VC[V]", property, aux, &mut file);
                }
            }
        } else if svo_triple.len() == 5 && aux {    // Auxiliary
            for template_aux in templates_aux {
                if template_aux[0] == "S" {
                    sent_to_conll_gold(template_aux, svo_triple, "VF[S]LK[V]MF[O]", property, aux, &mut file);
                } else if template_aux[0] == "O" {
                    sent_to_conll_gold(template_aux, svo_triple, "VF[O]LK[V]MF[S]", property, aux, &mut file);
                } else if template_aux[0] == v1_intro && template_aux[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "LK[V]MF[SO]", property, aux, &mut file);
                } else if template_aux[0] == v1_intro && template_aux[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "LK[V]MF[OS]", property, aux, &mut file);
                } else if template_aux[0] == "VAUX" && template_aux[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "LK[V]MF[SO]Q", property, aux, &mut file);
                } else if template_aux[0] == "VAUX" && template_aux[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "LK[V]MF[OS]Q", property, aux, &mut file);
                } else if template_aux[0] == vl_intro && template_aux[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "MF[SO]VC[V]", property, aux, &mut file);
                } else if template_aux[0] == vl_intro && template_aux[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_aux, svo_triple, "MF[OS]VC[V]", property, aux, &mut file);
                }
            }
        } else if svo_triple.len() == 5 && !aux {    // PP
            for template_pp in templates_pp {
                if template_pp[0] == "S" {
                    sent_to_conll_gold(template_pp, svo_triple, "VF[S]LK[V]MF[O]", property, aux, &mut file);
                } else if template_pp[0] == "O" {
                    sent_to_conll_gold(template_pp, svo_triple, "VF[O]LK[V]MF[S]", property, aux, &mut file);
                } else if template_pp[0] == v1_intro && template_pp[3] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_pp, svo_triple, "LK[V]MF[SO]", property, aux, &mut file);
                } else if template_pp[0] == v1_intro && template_pp[3] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_pp, svo_triple, "LK[V]MF[OS]", property, aux, &mut file);
                } else if template_pp[0] == "V" && template_pp[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_pp, svo_triple, "LK[V]MF[SO]Q", property, aux, &mut file);
                } else if template_pp[0] == "V" && template_pp[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_pp, svo_triple, "LK[V]MF[OS]Q", property, aux, &mut file);
                } else if template_pp[0] == vl_intro && template_pp[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template_pp, svo_triple, "MF[SO]VC[V]", property, aux, &mut file);
                } else if template_pp[0] == vl_intro && template_pp[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template_pp, svo_triple, "MF[OS]VC[V]", property, aux, &mut file);
                }
            }
        } else if svo_triple.len() == 4 {
            for template in templates {

                if template[0] == "S" {
                    sent_to_conll_gold(template, svo_triple, "VF[S]LK[V]MF[O]", property, aux, &mut file);
                } else if template[0] == "O" {
                    sent_to_conll_gold(template, svo_triple, "VF[O]LK[V]MF[S]", property, aux, &mut file);
                } else if template[0] == v1_intro && template[2] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "LK[V]MF[SO]", property, aux, &mut file);
                } else if template[0] == v1_intro && template[2] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "LK[V]MF[OS]", property, aux, &mut file);
                } else if template[0] == "V" && template[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "LK[V]MF[SO]Q", property, aux, &mut file);
                } else if template[0] == "V" && template[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "LK[V]MF[OS]Q", property, aux, &mut file);
                } else if template[0] == vl_intro && template[1] == "S" {
                    if svo_triple[0].starts_with("?") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "MF[SO]VC[V]", property, aux, &mut file);
                } else if template[0] == vl_intro && template[1] == "O" {
                    if svo_triple[0].starts_with("*") {
                        continue;
                    }
                    sent_to_conll_gold(template, svo_triple, "MF[OS]VC[V]", property, aux, &mut file);
                }
            }
        } else {
            eprintln!("Sentence length {} not supported.", svo_triple.len());
        }
    }
    Ok(())
}

fn sent_to_conll_gold(template: &Vec<String>, svo_triple: &Vec<String>, order: &str, property: &str, aux: bool, file: &mut File) {
    let subj_len = svo_triple[1].split(" ").collect::<Vec<_>>().len();
    let obj_len = svo_triple[3].split(" ").collect::<Vec<_>>().len();
    let pp_len = if (svo_triple.len() == 5 && !aux)  || svo_triple.len() == 6 {
        svo_triple[svo_triple.len()-1].split(" ").collect::<Vec<_>>().len()
    } else {
        0
    };
    let mut v_head = if svo_triple.len() == 6 {  // With auxiliary AND PP
        match order {
            "LK[V]MF[SO]" => 2 + pp_len + subj_len + obj_len + 1,   // deshalb vso
            "LK[V]MF[OS]" => 2 + pp_len + subj_len + obj_len + 1,   // deshalb vos
            "LK[V]MF[SO]Q" => 1 + pp_len + subj_len + obj_len + 1,   // vso ?
            "LK[V]MF[OS]Q" => 1 + pp_len + subj_len + obj_len + 1,   // vos ?
            "VF[S]LK[V]MF[O]" => subj_len + 1 + pp_len + obj_len + 1,   // svo
            "VF[O]LK[V]MF[S]" => obj_len +  1 + pp_len + subj_len + 1,   // ovs
            "MF[SO]VC[V]" => 1 + pp_len + subj_len + obj_len + 1,   // weil sov
            "MF[OS]VC[V]" => 1 + pp_len + obj_len + subj_len + 1,   // weil osv
            _ => 0
        }
    } else if svo_triple.len() == 5 {  // With auxiliary OR PP
        if aux {
            match order {   // With auxiliary
                "LK[V]MF[SO]" => 2 + subj_len + obj_len + 1,
                "LK[V]MF[OS]" => 2 + obj_len + subj_len + 1,
                "LK[V]MF[SO]Q" => 1 + subj_len + obj_len + 1,
                "LK[V]MF[OS]Q" => 1 + obj_len + subj_len + 1,
                "VF[S]LK[V]MF[O]" => subj_len + 1 + obj_len + 1,
                "VF[O]LK[V]MF[S]" => obj_len + 1 + subj_len + 1,
                "MF[SO]VC[V]" => 1 + subj_len + obj_len + 1,
                "MF[OS]VC[V]" => 1 + obj_len + subj_len + 1,
                _ => 0
            }
        } else {
            match order {  // With PP
                "LK[V]MF[SO]" => 2,
                "LK[V]MF[OS]" => 2,
                "LK[V]MF[SO]Q" => 1,
                "LK[V]MF[OS]Q" => 1,
                "VF[S]LK[V]MF[O]" => subj_len + 1,
                "VF[O]LK[V]MF[S]" => obj_len + 1,
                "MF[SO]VC[V]" => 1 + subj_len + obj_len + pp_len + 1,
                "MF[OS]VC[V]" => 1 + obj_len + subj_len + pp_len + 1,
                _ => 0
            }
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
            let tokens_inner = svo_triple[1].split(" ").collect::<Vec<_>>();
            for subj_idx in 0..tokens_inner.len() {
                if subj_idx == tokens_inner.len() - 1 {
                    if conll_idx == 0 {
                        let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t{}\tnsubj\t_\t_", conll_idx + 1, uppercase_first_letter(&tokens_inner[subj_idx]), order, property, v_head);
                    } else {
                        let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t{}\tnsubj\t_\t_", conll_idx + 1, &tokens_inner[subj_idx], order, property, v_head);
                    }
                } else {
                    if conll_idx == 0 {
                        let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, uppercase_first_letter(&tokens_inner[subj_idx]), order, property);
                    } else {
                        let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, &tokens_inner[subj_idx], order, property);
                    }
                }
                conll_idx += 1;
            }
        } else if token == "V" {
            if conll_idx == 0 {
                let _ =  writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t0\tverb\t_\t_", conll_idx + 1, uppercase_first_letter(&svo_triple[2]), order, property);
            } else {
                let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t0\tverb\t_\t_", conll_idx + 1, svo_triple[2], order, property);
            }
            conll_idx += 1;
        } else if token == "O" {
            let tokens_inner = svo_triple[3].split(" ").collect::<Vec<_>>();
            for obj_idx in 0..tokens_inner.len() {
                if obj_idx == tokens_inner.len() - 1 {
                    if conll_idx == 0 {
                        let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t{}\tobj\t_\t_", conll_idx + 1, uppercase_first_letter(&tokens_inner[obj_idx]), order, property, v_head);
                    } else {
                        let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t{}\tobj\t_\t_", conll_idx + 1, &tokens_inner[obj_idx], order, property, v_head);
                    }
                } else {
                    if conll_idx == 0 {
                        let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, uppercase_first_letter(&tokens_inner[obj_idx]), order, property);
                    } else {
                        let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, &tokens_inner[obj_idx], order, property);
                    }
                }
                conll_idx += 1;
            }
        } else if token == "VAUX" {   // Only auxiliary
            if conll_idx == 0 {
                let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, uppercase_first_letter(&svo_triple[4]), order, property);
            } else {
                let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, svo_triple[4], order, property);
            }
            conll_idx += 1;
        } else if token == "PP" && aux {    // Auxiliary and PP
            let tokens_inner = svo_triple[5].split(" ").collect::<Vec<_>>();
            for pp_idx in 0..tokens_inner.len() {
                if conll_idx == 0 {
                    let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, uppercase_first_letter(&tokens_inner[pp_idx]), order, property);
                } else {
                    let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, &tokens_inner[pp_idx], order, property);
                }
                conll_idx += 1;
            }
        } else if token == "PP" {   // Only PP
            let tokens_inner = svo_triple[4].split(" ").collect::<Vec<_>>();
            for pp_idx in 0..tokens_inner.len() {
                if conll_idx == 0 {
                    let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, uppercase_first_letter(&tokens_inner[pp_idx]), order, property);
                } else {
                    let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, &tokens_inner[pp_idx], order, property);
                }
                conll_idx += 1;
            }
        } else if token != "?" {
            if conll_idx == 0 {
                let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, uppercase_first_letter(&token), order, property);
            } else {
                let _ = writeln!(file, "{}\t{}\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_", conll_idx + 1, token, order, property);
            }
            conll_idx += 1;
        }
    }
    if template[template.len() - 1] == "?" {
        let _ = writeln!(file, "{}\t?\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_\n", conll_idx + 1, order, property);
    } else {
        let _ = writeln!(file, "{}\t.\t_\t_\t_\torder:{}|props:{}\t_\t_\t_\t_\n", conll_idx + 1, order, property);
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

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn filter(text: Vec<Vec<Token>>, props_allowed: &[&str]) {
    for sent in text {
        let mut first = true;
        let mut morph = "";
        let mut ne = "";
        let mut tf = "";
        let mut order = "";
        let mut prop1 = "";
        let mut prop2 = "";

        for idx in 0..sent.len() {
            let token = &sent[idx];
            if first {
                let features = token.features().map(Features::as_map).expect("No mapping");
                morph = &features.get("Morph").expect("No morph").as_ref().expect("No more morph");
                ne = &features.get("NE").expect("No ne").as_ref().expect("No more ne");
                tf = &features.get("tf").expect("No tf").as_ref().expect("No more tf");
                order = &features.get("order").expect("No order").as_ref().expect("No more order");
                let props = &features.get("props").expect("No props").as_ref().expect("No more props").split("-").collect::<Vec<_>>();
                prop1 = props[0];
                prop2 = props[1];
                first = false;
            }
            let lemma = token.lemma().expect("No lemma");
            let pos = token.pos().expect("No pos");
            if ! (
                (order == "LK[V]MF[OS]" || order == "LK[V]MF[OS]Q" || order == "MF[OS]VC[V]")
                    && !(props_allowed.contains(&prop1) || props_allowed.contains(&prop2))
            ) {
                println!("{}\t{}\t{}\t_\t{}\tMorph:{}|NE:{}|order:{}|props:{}-{}|tf:{}", idx + 1, token.form(), lemma, pos, morph, ne, order, prop1, prop2, tf);
            }
        }
        println!();
    }
}


pub fn filter_gold(text: Vec<Vec<Token>>, props_allowed: &[&str]) {
    for sent in text {
        let mut first = true;
        let mut order = "";
        let mut prop1 = "";
        let mut prop2 = "";

        for idx in 0..sent.len() {
            let token = &sent[idx];
            if first {
                let features = token.features().map(Features::as_map).expect("No mapping");
                order = &features.get("order").expect("No order").as_ref().expect("No more order");
                let props = &features.get("props").expect("No props").as_ref().expect("No more props").split("-").collect::<Vec<_>>();
                prop1 = props[0];
                prop2 = props[1];
                first = false;
            }

            let mut head = "_".to_string();
            if let Some(token_head) = token.head() {
                head = token_head.to_string()
            };

            let mut deprel = "_";
            if let Some(token_deprel) = token.head_rel() {
                deprel = token_deprel
            };

            if ! (
                (order == "LK[V]MF[OS]" || order == "LK[V]MF[OS]Q" || order == "MF[OS]VC[V]")
                    && !(props_allowed.contains(&prop1) || props_allowed.contains(&prop2))
            ) {
                println!("{}\t{}\t_\t_\t_\torder:{}|props:{}-{}\t{}\t{}\t_\t_", idx + 1, token.form(), order, prop1, prop2, head, deprel);
            }
        }
        println!();
    }
}