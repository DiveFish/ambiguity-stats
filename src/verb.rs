extern crate conllx;
extern crate flate2;

use std::collections::HashMap;

use conllx::Token;
use flate2::Compression;
use flate2::write::GzEncoder;

use std::fmt::{self, Formatter, Display};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

//TODO: Check for all places with i whether i+1 should be used

pub fn svo_triples(sent: &[Token], lemma: bool, object_rel: &str) {
    let mut head_verb_args = HashMap::new();
    let mut aux_content_verbs = HashMap::new();

    for i in 0..sent.len() {
        let token = &sent[i];
        let deprel = token.head_rel().expect("No deprel");
        let head = token.head().expect("No head");
        if head > 0 && deprel == "AUX" { // Reattach auxiliary verb to content verb
            let mut aux_verb_idx = head - 1;
            while aux_verb_idx > 0 && sent[aux_verb_idx].head_rel().expect("No deprel").eq("AUX") {
                if sent[aux_verb_idx].head().expect("No head") > 0 {
                    aux_verb_idx = sent[aux_verb_idx].head().expect("No head") - 1;
                } else {
                    break;
                }
            }
            aux_content_verbs.insert(aux_verb_idx, i);
        }
    }

    for i in 1..sent.len() {
        let gold_token = &sent[i];

        let deprel = gold_token.head_rel().expect("No deprel");
        let head = gold_token.head().expect("No head");

        if (deprel == "SUBJ" || deprel == object_rel) && head > 0 {
            let mut verb_idx = head - 1;
            if let Some(content_verb_idx) = aux_content_verbs.get(&verb_idx) {
                verb_idx = *content_verb_idx;
            };
            if deprel == "SUBJ" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);
                entry[0] = i;
            } else if deprel == object_rel {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);

                if entry[1] == 0 {
                    entry[1] = i;
                } else if entry[2] == 0 {
                    entry[2] = i;
                } else if entry[3] == 0 {
                    entry[3] = i;
                } else if entry[4] == 0 {
                    entry[4] = i;
                }
            }
        }
    }

    for (verb_idx, verb_args) in head_verb_args.iter() {

        let verb = sent[*verb_idx].clone();
        let verb_form = verb.form().clone();

        let subj_idx = verb_args[0];
        let obj1_idx = verb_args[1];
        let obj2_idx = verb_args[2];
        let obj3_idx = verb_args[3];
        let obj4_idx = verb_args[4];
        let mut objects = Vec::with_capacity(4);
        objects.push(obj1_idx);
        objects.push(obj2_idx);
        objects.push(obj3_idx);
        objects.push(obj4_idx);

        if subj_idx > 0 {

            let subj = sent[subj_idx].clone();
            let subj_form = subj.form().clone();

            for obj_idx in objects {
                if obj_idx == 0 {
                    break;
                } else {
                    let order = if subj_idx < *verb_idx && *verb_idx < obj_idx {
                        "SVO"
                    } else if subj_idx < obj_idx && obj_idx < *verb_idx {
                        "SOV"
                    } else if *verb_idx < subj_idx && subj_idx < obj_idx {
                        "VSO"
                    } else if *verb_idx < obj_idx && obj_idx < subj_idx {
                        "VOS"
                    } else if obj_idx < subj_idx && subj_idx < *verb_idx {
                        "OSV"
                    } else if obj_idx < *verb_idx && *verb_idx < subj_idx {
                        "OVS"
                    } else {
                        "UNK"
                    };
                    let obj = sent[obj_idx].clone();
                    let obj_form = obj.form().clone();
                    if lemma {
                        if let Some(obj_lemma) = obj.lemma()  {
                            if ! (obj_lemma.starts_with("#refl") || obj_lemma == "mich" || obj_lemma == "dich" || obj_lemma == "sich" || obj_lemma == "uns" || obj_lemma == "euch") {
                                if let Some(subj_lemma) = subj.lemma() {
                                    print!("{}\t", subj_lemma);
                                }
                                if let Some(verb_lemma) = verb.lemma() {
                                    print!("{}\t", verb_lemma.replace("\"", "").replace("#", "").split("%").collect::<Vec<&str>>()[0]);
                                }
                                print!("{}\t{}\n", obj_lemma, order);
                            }
                        }
                    } else {
                        if let Some(obj_lemma) = obj.lemma() {
                            if !(obj_lemma.starts_with("#refl") || obj_lemma == "sich") {
                                println!("{}\t{}\t{}\t{}", subj_form.replace(" ", "_space_"), verb_form.to_lowercase().replace(" ", "_space_"), obj_form.replace(" ", "_space_"), order);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn ccrawl_triples(text: &Vec<Vec<Token>>, lemma: bool, output_filename: &str) {

    let output_file = File::create(&output_filename).expect("Unable to create file");
    let mut encoded_file = GzEncoder::new(output_file, Compression::default());

    for sent in text.iter() {
        let mut head_verb_args = HashMap::new();
        let mut aux_content_verbs = HashMap::new();

        for i in 0..sent.len() {
            let token = &sent[i];
            let deprel = token.head_rel().expect("No deprel");
            let head = token.head().expect("No head");
            if head > 0 && deprel == "AUX" {
                // Reattach auxiliary verb to content verb
                let mut aux_verb_idx = head - 1;
                while aux_verb_idx > 0 && sent[aux_verb_idx].head_rel().expect("No deprel").eq("AUX") {
                    if sent[aux_verb_idx].head().expect("No head") > 0 {
                        aux_verb_idx = sent[aux_verb_idx].head().expect("No head") - 1;
                    } else {
                        break;
                    }
                }
                aux_content_verbs.insert(aux_verb_idx, i);
            }
        }

        for i in 1..sent.len() {
            let gold_token = &sent[i];

            let deprel = gold_token.head_rel().expect("No deprel");
            let head = gold_token.head().expect("No head");

            if (deprel == "SUBJ" || deprel == "OBJA" || deprel == "OBJD") && head > 0 {
                let mut verb_idx = head - 1;
                if let Some(content_verb_idx) = aux_content_verbs.get(&verb_idx) {
                    verb_idx = *content_verb_idx;
                };
                if deprel == "SUBJ" {
                    let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);
                    entry[0] = i;
                } else if deprel == "OBJA" || deprel == "OBJD" {
                    let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);

                    if entry[1] == 0 {
                        entry[1] = i;
                    } else if entry[2] == 0 {
                        entry[2] = i;
                    } else if entry[3] == 0 {
                        entry[3] = i;
                    } else if entry[4] == 0 {
                        entry[4] = i;
                    }
                }
            }
        }

        for (verb_idx, verb_args) in head_verb_args.iter() {
            let verb = sent[*verb_idx].clone();
            let verb_form = verb.form().clone();

            let subj_idx = verb_args[0];
            let obj1_idx = verb_args[1];
            let obj2_idx = verb_args[2];
            let obj3_idx = verb_args[3];
            let obj4_idx = verb_args[4];
            let mut objects = Vec::with_capacity(4);
            objects.push(obj1_idx);
            objects.push(obj2_idx);
            objects.push(obj3_idx);
            objects.push(obj4_idx);

            if subj_idx > 0 {
                let subj = sent[subj_idx].clone();
                let subj_form = subj.form().clone();

                for obj_idx in objects {
                    if obj_idx == 0 {
                        break;
                    } else {
                        let obj = sent[obj_idx].clone();
                        let obj_form = obj.form().clone();

                        if lemma {
                            if subj.lemma().is_some() && verb.lemma().is_some() && obj.lemma().is_some() {
                                if subj_idx > obj_idx {
                                    let _ = encoded_file.write_all(format!("{}\t{}\t{}\n", obj.lemma().expect("No object lemma"), verb.lemma().expect("No verb lemma").replace("\"", "").replace("#", "").split("%").collect::<Vec<&str>>()[0], subj.lemma().expect("No subject lemma")).as_bytes());
                                } else {
                                    let _ = encoded_file.write_all(format!("{}\t{}\t{}\n", subj.lemma().expect("No object lemma"), verb.lemma().expect("No verb lemma").replace("\"", "").replace("#", "").split("%").collect::<Vec<&str>>()[0], obj.lemma().expect("No subject lemma")).as_bytes());
                                }
                            }
                        } else {
                            if subj_idx > obj_idx {
                                let _ = encoded_file.write_all(format!("{}\t{}\t{}\n", subj_form, verb_form.to_lowercase(), obj_form).as_bytes());
                            } else {
                                let _ = encoded_file.write_all(format!("{}\t{}\t{}\n", obj_form, verb_form.to_lowercase(), subj_form).as_bytes());
                            }
                        }
                    }
                }
            }
        }
    }
    let _ = encoded_file.finish();
}

pub fn inversion_verbs_content(gold_sent: &[Token]) -> (Vec<String>, Vec<String>) {
    let mut head_verb_args = HashMap::new();
    let mut aux_content_verbs = HashMap::new();

    for i in 1..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_head = gold_token.head().expect("No head");
        if gold_head > 0 && gold_deprel == "AUX" { // Reattach auxiliary verb to content verb
            let mut aux_verb_idx = gold_head;
            while aux_verb_idx > 0 && gold_sent[aux_verb_idx - 1].head_rel().expect("No deprel").eq("AUX") {
                aux_verb_idx = gold_sent[aux_verb_idx - 1].head().expect("No head");
            }
            aux_content_verbs.insert(aux_verb_idx, i);
        }
    }

    for i in 1..gold_sent.len() {
        let gold_token = &gold_sent[i];

        if gold_token.head_rel().is_some() && gold_token.head().is_some() {
            //For NoSta-D << Remove?
            let gold_deprel = gold_token.head_rel().expect("No deprel");
            let gold_head = gold_token.head().expect("No head");

            if (gold_deprel == "SUBJ" || gold_deprel == "OBJA") && gold_head > 0 {
                let mut verb_idx = gold_head;
                if let Some(content_verb_idx) = aux_content_verbs.get(&verb_idx) {
                    //verb_idx = *content_verb_idx;
                    continue;   // To dismiss auxiliary verbs
                };
                if gold_deprel == "SUBJ" {
                    let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                    entry[0] = i;
                } else if gold_deprel == "OBJA" {
                    let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);

                    let verb_distance_i = if verb_idx > i {
                        verb_idx - i
                    } else { i - verb_idx };
                    let verb_distance_entry = if verb_idx > entry[1] {
                        verb_idx - entry[1]
                    } else { entry[1] - verb_idx };

                    if entry[1] == 0 || verb_distance_entry > verb_distance_i {
                        entry[1] = i;
                    }
                }
            }
        }
    }

    let mut verbs = Vec::with_capacity(3);
    let mut inversion_verbs = Vec::with_capacity(3);
    for (verb_idx, verb_args) in head_verb_args.iter() {
        let subj_idx = verb_args[0];    // Idx in sent, no 'idx - 1' necessary
        let obj_idx = verb_args[1];     // Idx in sent, no 'idx - 1' necessary
        let verb = gold_sent[*verb_idx - 1].clone();
        let verb_lemma = verb.lemma().clone();

        if verb_lemma.is_some() && verb.pos().expect("No PoS").starts_with("V") {

            verbs.push(verb_lemma.unwrap().to_string().to_lowercase());

            // Clause has a subject and object where object precedes subject
            if subj_idx > 0 && obj_idx > 0 && obj_idx < subj_idx {
                inversion_verbs.push(verb_lemma.unwrap().to_string().to_lowercase());
            } else {
                inversion_verbs.push("UNKNOWN".to_string());
            }

        } else if verb.head().is_some() && verb.head().expect("No head") == 0 { // Head is the ROOT token

            verbs.push("ROOT".to_string().to_lowercase());
            // Clause has a subject and object where object precedes subject
            if subj_idx > 0 && obj_idx > 0 && obj_idx < subj_idx {
                inversion_verbs.push("ROOT".to_string().to_lowercase());
            }

        } else if verb_lemma.is_some() && verb_lemma.expect("No lemma").ends_with("end") {  // Present participles

            let mut lemma = verb_lemma.expect("No lemma").to_string();
            lemma.truncate(lemma.len() - 1);    // Alternative: lemma.pop()
            verbs.push(lemma.to_lowercase());

            // Clause has a subject and object where object precedes subject
            if subj_idx > 0 && obj_idx > 0 && obj_idx < subj_idx {
                inversion_verbs.push(lemma.to_lowercase());
            } else {
                inversion_verbs.push("UNKNOWN".to_string());
            }

        } else if verb.head().is_some() && verb.head().expect("No head") > 0 && gold_sent[verb.head().expect("No head") - 1].lemma().is_some() {

            if gold_sent[verb.head().expect("No head") - 1].pos().expect("No PoS").starts_with("V") { // Filter non-verbs

                verbs.push(gold_sent[verb.head().expect("No head") - 1].lemma().unwrap().to_string().to_lowercase());

                // Clause has a subject and object where object precedes subject
                if subj_idx > 0 && obj_idx > 0 && obj_idx < subj_idx {
                    inversion_verbs.push(gold_sent[verb.head().expect("No head") - 1].lemma().unwrap().to_string().to_lowercase());
                } else {
                    inversion_verbs.push("UNKNOWN".to_string());
                }
            }
        }
    }
    (verbs, inversion_verbs)
}

/// Collect all verbs which take part in inversion. As object consider
/// only the direct object that is closest to the main verb.
/// For NoStaD, change pos() to cpos()
pub fn inversion_verbs(sent: &[Token]) -> (Vec<String>, Vec<String>) {
    let mut head_verb_args = HashMap::new();

    for i in 1..sent.len() {
        let gold_token = &sent[i];
        let mut main_verb_idx = 0;

        if gold_token.head_rel().is_some() && gold_token.head().is_some() {
            //For NoSta-D << Remove?
            let gold_deprel = gold_token.head_rel().expect("No deprel");
            let gold_head = gold_token.head().expect("No head");

            if (gold_deprel == "SUBJ" || gold_deprel == "OBJA") && gold_head > 0 {
                let mut verb_idx;
                if (gold_head > 0)
                    &&
                    sent[gold_head - 1].head_rel().is_some()
                    &&
                    (
                        sent[gold_head - 1]
                            .head_rel()
                            .expect("No deprel")
                            .eq("AUX")
                        /*|| gold_sent[gold_head - 1]
                        .head_rel()
                        .expect("No deprel")
                        .eq("OBJI") // For NoSta-D*/
                    )
                    {
                        // Reattach auxiliary verb to content verb
                        verb_idx = sent[gold_head - 1].head().expect("No head");
                        main_verb_idx = gold_head;
                    } else {
                    verb_idx = gold_head;
                }

                if gold_deprel == "SUBJ" {
                    let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);
                    entry[0] = i;
                } else if gold_deprel == "OBJA" {
                    let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);

                    let verb_distance_i = if verb_idx > i {
                        verb_idx - i
                    } else { i - verb_idx };
                    let verb_distance_entry = if verb_idx > entry[1] {
                        verb_idx - entry[1]
                    } else { entry[1] - verb_idx };

                    if entry[1] == 0 || verb_distance_entry > verb_distance_i {
                        entry[1] = i;
                    }
                    entry[4] = main_verb_idx;
                }
            }
        }
    }

    let mut verbs = Vec::with_capacity(3);
    let mut inversion_verbs = Vec::with_capacity(3);
    for (verb_idx, verb_args) in head_verb_args.iter() {
        let subj_idx = verb_args[0];
        let obj_idx = verb_args[1];
        let verb = sent[*verb_idx - 1].clone();
        let mut verb_lemma = verb.lemma().clone();

        if verb_lemma.is_some() && verb.pos().expect("No PoS").starts_with("V") {

            verbs.push(verb_lemma.unwrap().to_string().to_lowercase());

            // Clause has a subject and object where object precedes subject
            if subj_idx > 0 && obj_idx > 0 && obj_idx < subj_idx {
                inversion_verbs.push(verb_lemma.unwrap().to_string().to_lowercase());
            } else {
                inversion_verbs.push("UNKNOWN".to_string());
            }

        } else if verb.head().is_some() && verb.head().expect("No head") == 0 { // Head is the ROOT token

            verbs.push("ROOT".to_string().to_lowercase());
            // Clause has a subject and object where object precedes subject
            if subj_idx > 0 && obj_idx > 0 && obj_idx < subj_idx {
                inversion_verbs.push("ROOT".to_string().to_lowercase());
            }

        } else if verb_lemma.is_some() && verb_lemma.expect("No lemma").ends_with("end") {  // Present participles

            let mut lemma = verb_lemma.expect("No lemma").to_string();
            lemma.truncate(lemma.len() - 1);    // Alternative: lemma.pop()
            verbs.push(lemma.to_lowercase());

            // Clause has a subject and object where object precedes subject
            if subj_idx > 0 && obj_idx > 0 && obj_idx < subj_idx {
                inversion_verbs.push(lemma.to_lowercase());
            } else {
                inversion_verbs.push("UNKNOWN".to_string());
            }

        } else if verb.head().is_some() && verb.head().expect("No head") > 0 && sent[verb.head().expect("No head") - 1].lemma().is_some() {

            if sent[verb.head().expect("No head") - 1].pos().expect("No PoS").starts_with("V") { // Filter non-verbs

                verbs.push(sent[verb.head().expect("No head") - 1].lemma().unwrap().to_string().to_lowercase());

                // Clause has a subject and object where object precedes subject
                if subj_idx > 0 && obj_idx > 0 && obj_idx < subj_idx {
                    inversion_verbs.push(sent[verb.head().expect("No head") - 1].lemma().unwrap().to_string().to_lowercase());
                } else {
                    inversion_verbs.push("UNKNOWN".to_string());
                }
            }
        }
    }

    (verbs, inversion_verbs)
}

/// Count passive or auxiliary verbs in a sentence. Set `aux_pass_marker' to "aux" for auxiliaries,
/// to "passiv" for passives.
pub fn aux_pass_count(sent: &[Token], counter: &mut usize, aux_pass_marker: &str) {
    if !(aux_pass_marker == "aux" || aux_pass_marker == "passiv") {
        panic!("Usage: aux_pass_marker must be \"aux\" or \"passiv\"");
    }
    for i in 0..sent.len() {
        let gold_lemma = &sent[i].lemma().expect("No lemma");
        if gold_lemma.ends_with(aux_pass_marker) {
            *counter += 1;
        }
    }
}

// TODO: Untested
/// Count frequencies of inversion verbs and error per inversion.
pub fn err_by_verb(gold_sent: &[Token], parser_sent: &[Token], inv_errs: &mut HashMap<String, (usize, usize, usize)>) {
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
                    entry[1] = i;   // SUBJ mistaken for OBJ
                }
            } else if gold_deprel.starts_with("OBJ") {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);

                let verb_distance_i = if verb_idx > i {
                    verb_idx - i
                } else { i - verb_idx };
                let verb_distance_entry = if verb_idx > entry[2] {
                    verb_idx - entry[2]
                } else { entry[2] - verb_idx };

                if entry[2] == 0 || verb_distance_entry > verb_distance_i {
                    entry[2] = i;

                    if token_deprel == "SUBJ" {
                        entry[3] = i; // OBJ mistaken for SUBJ
                    }
                }
            }
        }
    }

    for (verb_idx, val) in head_verb_args.iter() {
        let gold_subjidx = val[0];
        let parser_objidx = val[1]; // OBJ but should have been SUBJ
        let gold_objidx = val[2];
        let parser_subjidx = val[3]; // SUBJ but should have been OBJ

        // verb_freq: Overall frequency of verb
        // verb_inv_freq: Frequency of verb in inversion
        // verb_err_freq: Number of errors in inversion
        let (verb_freq, verb_inv_freq, verb_err_freq) = inv_errs.entry(gold_sent[*verb_idx - 1].lemma().expect("No lemma").to_string().to_lowercase()).or_insert((0, 0, 0));
        *verb_freq += 1;

        if gold_subjidx > 0 && gold_objidx > 0 && gold_subjidx > gold_objidx {
            *verb_inv_freq += 1;
            if parser_objidx > 0 || parser_subjidx > 0 {
                *verb_err_freq += 1;
            }
        }
    }
}

pub fn wo_freqs(sent: &[Token]) -> (usize, usize, usize, usize, usize, usize) {
    let mut head_verb_args = HashMap::new();
    let mut aux_content_verbs = HashMap::new();

    for i in 0..sent.len() {
        let token = &sent[i];
        let deprel = token.head_rel().expect("No deprel");
        let head = token.head().expect("No head");
        if head > 0 && deprel == "AUX" { // Reattach auxiliary verb to content verb
            let mut aux_verb_idx = head - 1;
            while aux_verb_idx > 0 && sent[aux_verb_idx].head_rel().expect("No deprel").eq("AUX") {
                if sent[aux_verb_idx].head().expect("No head") > 0 {
                    aux_verb_idx = sent[aux_verb_idx].head().expect("No head") - 1;
                } else {
                    break;
                }
            }
            aux_content_verbs.insert(aux_verb_idx, i);
        }
    }

    for i in 1..sent.len() {
        let gold_token = &sent[i];

        let deprel = gold_token.head_rel().expect("No deprel");
        let head = gold_token.head().expect("No head");

        if (deprel == "SUBJ" || deprel == "OBJA" || deprel == "OBJD") && head > 0 {
            let mut verb_idx = head - 1;
            if let Some(content_verb_idx) = aux_content_verbs.get(&verb_idx) {
                verb_idx = *content_verb_idx;
            };
            if deprel == "SUBJ" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);
                entry[0] = i;
            } else if deprel == "OBJA" || deprel == "OBJD" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);

                if entry[1] == 0 {
                    entry[1] = i;
                } else if entry[2] == 0 {
                    entry[2] = i;
                } else if entry[3] == 0 {
                    entry[3] = i;
                } else if entry[4] == 0 {
                    entry[4] = i;
                }
            }
        }
    }

    let mut svo = 0;
    let mut ovs = 0;
    let mut vso = 0;
    let mut vos = 0;
    let mut sov = 0;
    let mut osv = 0;

    for (verb_idx, verb_args) in head_verb_args.iter() {

        let verb = sent[*verb_idx].clone();
        let verb_form = verb.form().clone();

        let subj_idx = verb_args[0];
        let obj1_idx = verb_args[1];
        let obj2_idx = verb_args[2];
        let obj3_idx = verb_args[3];
        let obj4_idx = verb_args[4];
        let mut objects = Vec::with_capacity(4);
        objects.push(obj1_idx);
        objects.push(obj2_idx);
        objects.push(obj3_idx);
        objects.push(obj4_idx);

        if subj_idx > 0 {

            let subj = sent[subj_idx].clone();
            let subj_form = subj.form().clone();

            for obj_idx in objects {
                if obj_idx == 0 {
                    break;
                } else {
                    if subj_idx < *verb_idx && *verb_idx < obj_idx {
                        svo += 1;
                    } else if subj_idx < obj_idx && obj_idx < *verb_idx {
                        sov += 1;
                    } else if *verb_idx < subj_idx && subj_idx < obj_idx {
                        vso += 1;
                    } else if *verb_idx < obj_idx && obj_idx < subj_idx {
                        vos += 1;
                    } else if obj_idx < subj_idx && subj_idx < *verb_idx {
                        osv += 1;
                    } else if obj_idx < *verb_idx && *verb_idx < subj_idx {
                        ovs += 1;
                    }
                }
            }
        }
    }
    (svo, ovs, vso, vos, sov, osv)
}

/// In the Hamburg treebank annotation scheme, subjects are attached to the inflected verb, objects
/// to the content verb. In order to retrieve subjects and objects that belong together, the inflected
/// verb has to be mapped to the content verb, just as the object.
pub fn content_verbs_hdt(sent: &Vec<Token>) -> HashMap<usize, usize> {

    let mut aux_content_verbs = HashMap::new();

    for i in 0..sent.len() {
        let token = &sent[i];
        let deprel = token.head_rel().expect("No deprel");
        let head = token.head().expect("No head");
        if head > 0 && deprel == "AUX" {
            // Reattach auxiliary verb to content verb
            let mut aux_verb_idx = head - 1;
            while aux_verb_idx > 0 && sent[aux_verb_idx].head_rel().expect("No deprel").eq("AUX") {
                if sent[aux_verb_idx].head().expect("No head") > 0 {
                    aux_verb_idx = sent[aux_verb_idx].head().expect("No head") - 1;
                } else {
                    break;
                }
            }
            aux_content_verbs.entry(aux_verb_idx).or_insert(i);
        }
    }
    aux_content_verbs
}

/// In UD, all verbs in a sentence are headed by the content verb. This methods looks for all
/// inflected verbs in a sentence and stores a mapping from the content verb to the inflected verb.
/// It is then possible to retrieve the content verb's inflected verb which is important when
/// dealing with topoligical fields and word order in German.
pub fn inflected_verbs_ud(sent: &Vec<Token>, language: &str) -> HashMap<usize, usize> {

    let mut content_inflected_verbs = HashMap::new();
    for i in 0..sent.len() {
        let token = &sent[i];
        let deprel = token.head_rel().expect("No deprel");
        let pos = token.pos().expect("No pos");
        let head = token.head().expect("No head");
        if language == "german" {
            if head > 0 && pos.ends_with("FIN") && deprel == "aux" {    // Assumes combined UD-HDT PoS tags
                content_inflected_verbs.insert(head - 1, i);
            }
        } else if language == "dutch" {
            if head > 0 && pos.ends_with("pv") && deprel == "aux" {    // Used Lassy small PoS tags
                content_inflected_verbs.insert(head - 1, i);
            }
        }
    }
    content_inflected_verbs
}
