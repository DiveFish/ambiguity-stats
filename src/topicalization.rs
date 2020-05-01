extern crate conllx;

use conllx::Token;
use std::collections::HashMap;
use verb::{content_verbs_hdt,inflected_verbs_ud};
use io::{print_sentence, print_sentence_ext};

/// Orders distnguished along these lines:
/// VF[S]LK[V]MF[O]
/// VF[O]LK[V]MF[S]
/// LK[V]MF[SO]
/// LK[V]MF[OS]
/// MF[SO]VC[V]
/// MF[OS]VC[V]
pub fn order_freq_hdt(parsed: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>, las: bool, debug: bool) {

    let debug_sent = false;
    let mut num_hits = 0;

    let mut svo = 0;
    let mut ovs = 0;
    let mut vso = 0;
    let mut vos = 0;
    let mut vsoq = 0;
    let mut vosq = 0;
    let mut sov = 0;
    let mut osv = 0;
    let mut svo_err = 0;
    let mut ovs_err = 0;
    let mut vso_err = 0;
    let mut vos_err = 0;
    let mut vsoq_err = 0;
    let mut vosq_err = 0;
    let mut sov_err = 0;
    let mut osv_err = 0;

    for (parsed_sent, gold_sent) in parsed.iter().zip(gold.iter()) {

        let gold_content_verbs = content_verbs_hdt(&gold_sent);
        //let gold_main_verbs:HashMap<usize,usize> = HashMap::new();    //To get results without auxiliary reattachment
        let mut head_verb_args = HashMap::new();
        let mut is_question = false;

        for i in 0..gold_sent.len() {
            let gold_token = &gold_sent[i];
            let gold_deprel = gold_token.head_rel().expect("No deprel");
            let gold_head = gold_token.head().expect("No head");
            let gold_pos = gold_token.pos().expect("No pos");

            let token = &parsed_sent[i];
            let token_deprel = token.head_rel().expect("No deprel");
            let token_head = token.head().expect("No head");

            if gold_token.form() == "?" {
                is_question = true;
            }

            if gold_deprel == "SUBJ" || gold_deprel == "OBJA" || gold_deprel == "OBJD" {

                let mut verb_idx = gold_head - 1;
                // Stores the inflected verb in case of reattachment from inflection auxiliary to content verb
                let mut infl_verb_idx = 0;
                if let Some(content_verb_idx) = gold_content_verbs.get(&verb_idx) {
                    if gold_sent[verb_idx].pos().expect("No pos").ends_with("FIN") {
                        infl_verb_idx = verb_idx;
                    }
                    verb_idx = *content_verb_idx;
                };

                if gold_deprel == "SUBJ" {
                    // Increase indices for head_verb_args by 1 since 0 are ignore in the final calculations
                    let entry = head_verb_args.entry(verb_idx + 1).or_insert(vec![0; 5]);
                    entry[0] = i+1;
                    if infl_verb_idx > 0 {
                        entry[3] = infl_verb_idx + 1;
                    }

                    if debug {
                        if infl_verb_idx > 0 {
                            println!("SUBJ> {} {} - {} {}", entry[0], gold_sent[i].form(), gold_sent[verb_idx].form(), gold_sent[infl_verb_idx].form());
                        } else {
                            println!("SUBJ> {} {} - {}", entry[0], gold_sent[i].form(), gold_sent[verb_idx].form());
                        }
                        println!("{:?}", entry);
                    }

                    if gold_deprel != token_deprel || gold_head != token_head {
                        entry[4] += 1;
                    }
                } else if gold_deprel == "OBJA" || gold_deprel == "OBJD" {
                    let entry = head_verb_args.entry(verb_idx + 1).or_insert(vec![0; 5]);
                    if entry[1] == 0 {
                        entry[1] = i+1;
                    } else {
                        entry[2] = i+1;
                    }
                    if infl_verb_idx > 0 {
                        entry[3] = infl_verb_idx + 1;
                    }
                    if debug {
                         if infl_verb_idx > 0 {
                             println!("OBJ> {} {}(2) {} - {} {}", entry[1], entry[2], gold_sent[i].form(), gold_sent[verb_idx].form(), gold_sent[infl_verb_idx].form());
                         } else {
                             println!("OBJ> {} {}(2) {} - {}", entry[1], entry[2], gold_sent[i].form(), gold_sent[verb_idx].form());
                         }
                        println!("{:?}", entry);
                    }

                    if gold_deprel != token_deprel || gold_head != token_head {
                        entry[4] += 1;
                    }
                }
            }
        }
        get_order_distr(
            &head_verb_args, &mut num_hits, las, is_question,
            &mut svo, &mut svo_err,
            &mut ovs, &mut ovs_err,
            &mut vso, &mut vso_err,
            &mut vos, &mut vos_err,
            &mut vsoq, &mut vsoq_err,
            &mut vosq, &mut vosq_err,
            &mut sov, &mut sov_err,
            &mut osv, &mut osv_err
        )
    }
    println!("HDT");
    print_order_res(
        num_hits, las,
        svo, svo_err,
        ovs, ovs_err,
        vso, vso_err,
        vos, vos_err,
        vsoq, vsoq_err,
        vosq, vosq_err,
        sov, sov_err,
        osv, osv_err
    )
}

pub fn order_freq_ud(parsed: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>, language: &str, las: bool, debug: bool) {

    let mut num_hits = 0;

    let mut svo = 0;
    let mut ovs = 0;
    let mut vso = 0;
    let mut vos = 0;
    let mut vsoq = 0;
    let mut vosq = 0;
    let mut sov = 0;
    let mut osv = 0;
    let mut svo_err = 0;
    let mut ovs_err = 0;
    let mut vso_err = 0;
    let mut vos_err = 0;
    let mut vsoq_err = 0;
    let mut vosq_err = 0;
    let mut sov_err = 0;
    let mut osv_err = 0;

    for (parsed_sent, gold_sent) in parsed.iter().zip(gold.iter()) {
        let mut head_verb_args = HashMap::new();
        let inflected_verbs = inflected_verbs_ud(&gold_sent, language);
        let mut is_question = false;

        for i in 0..gold_sent.len() {
            let gold_token = &gold_sent[i];
            let gold_deprel = gold_token.head_rel().expect("No deprel");
            let gold_head = gold_token.head().expect("No head");
            let gold_pos = gold_token.pos().expect("No pos");

            let token = &parsed_sent[i];
            let token_deprel = token.head_rel().expect("No deprel");
            let token_head = token.head().expect("No head");

            if gold_token.form() == "?" {
                is_question = true;
            }

            if gold_deprel == "nsubj" || gold_deprel == "obj" || gold_deprel == "iobj" {

                let verb_idx = gold_head;
                let infl_verb_idx = if let Some(inflected_verb_idx) = inflected_verbs.get(&(verb_idx - 1)) {
                    inflected_verb_idx + 1
                } else {
                    0
                };

                if gold_deprel == "nsubj" {
                    let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);
                    entry[0] = i + 1;
                    entry[3] = infl_verb_idx;

                    if gold_deprel != token_deprel || gold_head != token_head {
                        entry[4] += 1;
                    }
                    if debug {
                        if infl_verb_idx > 0 {
                            println!("nsubj> {} {} - {} {}", entry[0], gold_sent[i].form(), gold_sent[verb_idx - 1].form(), gold_sent[infl_verb_idx - 1].form());
                        } else {
                            println!("nsubj> {} {} - {}", entry[0], gold_sent[i].form(), gold_sent[verb_idx - 1].form());
                        }
                        println!("{:?}", entry);
                    }
                } else if gold_deprel == "obj" {
                    let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);
                    entry[1] = i + 1;
                    entry[3] = infl_verb_idx;

                    if gold_deprel != token_deprel || gold_head != token_head {
                        entry[4] += 1;
                    }

                    if debug {
                        if infl_verb_idx > 0 {
                            println!("obj> {} {} - {} {}", entry[1], gold_sent[i].form(), gold_sent[verb_idx - 1].form(), gold_sent[infl_verb_idx - 1].form());
                        } else {
                            println!("obj> {} {} - {}", entry[1], gold_sent[i].form(), gold_sent[verb_idx - 1].form());
                        }
                        println!("{:?}", entry);
                    }
                } else if gold_deprel == "iobj" {
                    let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);
                    entry[2] = i + 1;
                    entry[3] = infl_verb_idx;

                    if gold_deprel != token_deprel || gold_head != token_head {
                        entry[4] += 1;
                    }

                    if debug {
                        if infl_verb_idx > 0 {
                            println!("iobj> {} {} - {} {}", entry[2], gold_sent[i].form(), gold_sent[verb_idx - 1].form(), gold_sent[infl_verb_idx - 1].form());
                        } else {
                            println!("iobj> {} {} - {}", entry[2], gold_sent[i].form(), gold_sent[verb_idx - 1].form());
                        }
                        println!("{:?}", entry);
                    }
                }
            }
        }
        get_order_distr(
            &mut head_verb_args, &mut num_hits, las, is_question,
            &mut svo, &mut svo_err,
            &mut ovs, &mut ovs_err,
            &mut vso, &mut vso_err,
            &mut vos, &mut vos_err,
            &mut vsoq, &mut vsoq_err,
            &mut vosq, &mut vosq_err,
            &mut sov, &mut sov_err,
            &mut osv, &mut osv_err
        )
    }
    println!("UD");
    print_order_res(
        num_hits, las,
        svo, svo_err,
        ovs, ovs_err,
        vso, vso_err,
        vos, vos_err,
        vsoq, vsoq_err,
        vosq, vosq_err,
        sov, sov_err,
        osv, osv_err
    )
}

fn get_order_distr(
    head_verb_args: &HashMap<usize, Vec<usize>>, num_hits: &mut usize, las: bool, is_question: bool,
    svo: &mut usize, svo_err: &mut usize,
    ovs: &mut usize, ovs_err: &mut usize,
    vso: &mut usize, vso_err: &mut usize,
    vos: &mut usize, vos_err: &mut usize,
    vsoq: &mut usize, vsoq_err: &mut usize,
    vosq: &mut usize, vosq_err: &mut usize,
    sov: &mut usize, sov_err: &mut usize,
    osv: &mut usize, osv_err: &mut usize
) {

    for (mut head_verb_idx, verb_args) in head_verb_args.iter() {
        let subj_idx = verb_args[0];
        let obj_idx = verb_args[1];
        let obj2_idx = verb_args[2];
        let infl_verb_idx = verb_args[3];   // For sentences with auxiliary
        let err_cnt = verb_args[4];
        if infl_verb_idx > 0 {
            head_verb_idx = &infl_verb_idx;
        }

        //eprintln!("{} {}(1) {}(2) {} {}", subj_idx, obj_idx, obj_idx, head_verb_idx, is_question);

        if subj_idx > 0 && obj_idx > 0 { // Clause has a subject and an object
            *num_hits += 1;

            // Subject-object pairs
            let occ = if las {
                2
            } else {
                1
            };
            if subj_idx < *head_verb_idx && *head_verb_idx < obj_idx {
                *svo += occ;
                *svo_err += err_cnt;

            } else if obj_idx < *head_verb_idx && *head_verb_idx < subj_idx {
                *ovs += occ;
                *ovs_err += err_cnt;

            } else if *head_verb_idx < subj_idx && subj_idx < obj_idx {
                if is_question {
                    *vsoq += occ;
                    *vsoq_err += err_cnt;
                } else {
                    *vso += occ;
                    *vso_err += err_cnt;
                }
            } else if *head_verb_idx < obj_idx && obj_idx < subj_idx {
                if is_question {
                    *vosq += occ;
                    *vosq_err += err_cnt;
                } else {
                    *vos += occ;
                    *vos_err += err_cnt;
                }
            } else if subj_idx < obj_idx && obj_idx < *head_verb_idx {
                *sov += occ;
                *sov_err += err_cnt;

            } else if obj_idx < subj_idx && subj_idx < *head_verb_idx {
                *osv += occ;
                *osv_err += err_cnt;
            } else {
                println!("{} {}(1) {}", subj_idx, obj_idx, head_verb_idx);
            }

            if obj2_idx > 0 {
                *num_hits += 1;
                // Second object
                if subj_idx < *head_verb_idx && *head_verb_idx < obj2_idx {
                    *svo += 1;
                } else if obj2_idx < *head_verb_idx && *head_verb_idx < subj_idx {
                    *ovs += 1;
                } else if *head_verb_idx < subj_idx && subj_idx < obj2_idx {
                    if is_question {
                        *vsoq += 1;
                    } else {
                        *vso += 1;
                    }
                } else if *head_verb_idx < obj2_idx && obj2_idx < subj_idx {
                    if is_question {
                        *vosq += 1;
                    } else {
                        *vos += 1;
                    }
                } else if subj_idx < obj2_idx && obj2_idx < *head_verb_idx {
                    *sov += 1;
                } else if obj2_idx < subj_idx && subj_idx < *head_verb_idx {
                    *osv += 1;
                } else {
                    eprintln!("{} {}(2) {}", subj_idx, obj2_idx, head_verb_idx);
                }
            }
        }
    }
}

fn print_order_res(
    num_hits: usize, las: bool,
    svo: usize, svo_err: usize,
    ovs: usize, ovs_err: usize,
    vso: usize, vso_err: usize,
    vos: usize, vos_err: usize,
    vsoq: usize, vsoq_err: usize,
    vosq: usize, vosq_err: usize,
    sov: usize, sov_err: usize,
    osv: usize, osv_err: usize
) {
    println!("Hits: {}", num_hits);
    if las {
        println!("Word order,SVO token frequency,Error count,LAS\nSVO,{},{},{}\nOVS,{},{},{}\nVSO,{},{},{}\nVOS,{},{},{}\nVSOQ,{},{},{}\nVOSQ,{},{},{}\nSOV,{},{},{}\nOSV,{},{},{}\n",
                 svo, svo_err, 1.0 - svo_err as f32/svo as f32,
                 ovs, ovs_err, 1.0 - ovs_err as f32/ovs as f32,
                 vso, vso_err, 1.0 - vso_err as f32/vso as f32,
                 vos, vos_err, 1.0 - vos_err as f32/vos as f32,
                 vsoq, vsoq_err, 1.0 - vsoq_err as f32/vsoq as f32,
                 vosq, vosq_err, 1.0 - vosq_err as f32/vosq as f32,
                 sov, sov_err, 1.0 - sov_err as f32/sov as f32,
                 osv, osv_err, 1.0 - osv_err as f32/osv as f32
        );
    } else {
        println!("Word order,Frequency\nSVO,{}\nOVS,{}\nVSO,{}\nVOS,{}\nVSOQ,{}\nVOSQ,{}\nSOV,{}\nOSV,{}\n",
                 svo,
                 ovs,
                 vso,
                 vos,
                 vsoq,
                 vosq,
                 sov,
                 osv
        );
    }
}


/// Definiteness combinations distinguished along these lines:
/// 1. subject definite - object definite
/// 2. subject definite - object indefinite
/// 3. subject indefinite - object definite
/// 4. subject indefinite - object indefinite
pub fn definiteness_hdt(parsed: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>, _language: &str, las: bool, debug: bool) {

    let mut num_hits = 0;

    let mut sdef_odef_so = 0;
    let mut sdef_oindef_so = 0;
    let mut sindef_odef_so = 0;
    let mut sindef_oindef_so = 0;
    let mut sdef_odef_os = 0;
    let mut sdef_oindef_os = 0;
    let mut sindef_odef_os = 0;
    let mut sindef_oindef_os = 0;
    let mut sdef_odef_so_err = 0;
    let mut sdef_oindef_so_err = 0;
    let mut sindef_odef_so_err = 0;
    let mut sindef_oindef_so_err = 0;
    let mut sdef_odef_os_err = 0;
    let mut sdef_oindef_os_err = 0;
    let mut sindef_odef_os_err = 0;
    let mut sindef_oindef_os_err = 0;

    for (mut parsed_sent, mut gold_sent) in parsed.iter().zip(gold.iter()) {

        let gold_content_verbs = content_verbs_hdt(&mut gold_sent);
        let mut head_verb_args = HashMap::new();

        for i in 0..gold_sent.len() {
            let gold_token = &gold_sent[i];

            if let Some(gold_lemma) = gold_token.lemma() {
                let gold_pos = gold_token.pos().expect("No pos");
                let gold_deprel = gold_token.head_rel().expect("No deprel");
                let gold_head = gold_token.head().expect("No head");

                let token = &parsed_sent[i];
                let token_deprel = token.head_rel().expect("No deprel");
                let token_head = token.head().expect("No head");

                if gold_head > 0 && token_head > 0 {
                    let gold_head_token = &gold_sent[gold_head - 1];
                    let gold_head_deprel = gold_head_token.head_rel().expect("No deprel");
                    let gold_head_head = gold_head_token.head().expect("No head");

                    let token_head_token = &parsed_sent[token_head - 1];
                    let token_head_deprel = token_head_token.head_rel().expect("No deprel");
                    let token_head_head = token_head_token.head().expect("No head");

                    if gold_deprel == "DET" && (gold_head_deprel == "SUBJ" || gold_head_deprel == "OBJA" || gold_head_deprel == "OBJD") {
                        let def = if gold_lemma == "der" || gold_lemma == "die" || gold_lemma == "das" || gold_lemma == "der|die|das" || gold_pos == "PDAT" {
                            true
                        } else {
                            //if lemma == "eine" || lemma == "ein" || pos == "PIDAT" || pos == "PIAT" || pos == "PWAT" {
                            false
                        };
                        let mut verb_idx = gold_head_head - 1;
                        if let Some(content_verb_idx) = gold_content_verbs.get(&verb_idx) {
                            verb_idx = *content_verb_idx;
                        };

                        if gold_head_deprel == "SUBJ" {
                            let mut entry = head_verb_args.entry(verb_idx + 1).or_insert(vec![0; 7]);
                            add_def_indef_entry(def, &mut entry, 0, 1, gold_head);
                            if gold_head_deprel != token_head_deprel || gold_head_head != *&token_head_head {
                                entry[6] += 1;
                            }
                        } else if gold_head_deprel == "OBJA" || gold_head_deprel == "OBJD" {
                            let mut entry = head_verb_args.entry(verb_idx + 1).or_insert(vec![0; 7]);
                            if entry[2] == 0 && entry[3] == 0 {
                                add_def_indef_entry(def, &mut entry, 2, 3, gold_head);
                            } else {
                                add_def_indef_entry(def, &mut entry, 4, 5, gold_head);
                            }
                            if gold_head_deprel != token_head_deprel || gold_head_head != *&token_head_head {
                                entry[6] += 1;
                            }
                        }
                    } else if gold_pos == "NN" && (gold_deprel == "SUBJ" || gold_deprel == "OBJA" || gold_deprel == "OBJD")  {    // Plural nouns and proper names

                        let mut verb_idx = gold_head - 1;
                        if let Some(content_verb_idx) = gold_content_verbs.get(&verb_idx) {
                            verb_idx = *content_verb_idx;
                        };
                        if gold_deprel == "SUBJ" {
                            let mut entry = head_verb_args.entry(verb_idx + 1).or_insert(vec![0; 7]);
                            if entry[0] == 0 && entry[1] == 0 {
                                add_def_indef_entry(false, &mut entry, 0, 1, i + 1);
                                if gold_deprel != token_deprel || gold_head != *&token_head {
                                    entry[6] += 1;
                                }
                            }
                        } else if gold_deprel == "OBJA" || gold_deprel == "OBJD" {
                            let mut entry = head_verb_args.entry(verb_idx + 1).or_insert(vec![0; 7]);
                            if entry[2] == 0 && entry[3] == 0 {
                                add_def_indef_entry(false, &mut entry, 2, 3, i + 1);
                                if gold_deprel != token_deprel || gold_head != *&token_head {
                                    entry[6] += 1;
                                }
                            } else if entry[2] != i+1 && entry[3] != i+1 && entry[4] == 0 && entry[5] == 0 {
                                add_def_indef_entry(false, &mut entry, 4, 5, i + 1);
                                if gold_deprel != token_deprel || gold_head != *&token_head {
                                    entry[6] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        get_def_distribution(
            &head_verb_args, &mut num_hits, las, debug,
            &mut sdef_odef_so, &mut sdef_odef_so_err,
            &mut sdef_oindef_so, &mut sdef_oindef_so_err,
            &mut sindef_odef_so, &mut sindef_odef_so_err,
            &mut sindef_oindef_so, &mut sindef_oindef_so_err,
            &mut sdef_odef_os, &mut sdef_odef_os_err,
            &mut sdef_oindef_os, &mut sdef_oindef_os_err,
            &mut sindef_odef_os, &mut sindef_odef_os_err,
            &mut sindef_oindef_os, &mut sindef_oindef_os_err
        );
    }
    println!("HDT");
    print_def_res(
        num_hits, las,
        sdef_odef_so, sdef_odef_so_err,
        sdef_oindef_so, sdef_oindef_so_err,
        sindef_oindef_so, sindef_oindef_so_err,
        sindef_odef_so, sindef_odef_so_err,
        sdef_odef_os, sdef_odef_os_err,
        sdef_oindef_os, sdef_oindef_os_err,
        sindef_odef_os, sindef_odef_os_err,
        sindef_oindef_os, sindef_oindef_os_err
    );
}

pub fn definiteness_ud(parsed: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>, language: &str, las: bool, debug: bool) {

    let german = language == "german";
    let dutch = language == "dutch";
    assert_ne!(german, dutch);
    if german {
        eprintln!("Language: German");
    } else if dutch {
        eprintln!("Language: Dutch");
    } else {
        eprintln!("Choose a language!");
    }

    let mut num_hits = 0;

    let mut sdef_odef_so = 0;
    let mut sdef_oindef_so = 0;
    let mut sindef_odef_so = 0;
    let mut sindef_oindef_so = 0;
    let mut sdef_odef_os = 0;
    let mut sdef_oindef_os = 0;
    let mut sindef_odef_os = 0;
    let mut sindef_oindef_os = 0;
    let mut sdef_odef_so_err = 0;
    let mut sdef_oindef_so_err = 0;
    let mut sindef_odef_so_err = 0;
    let mut sindef_oindef_so_err = 0;
    let mut sdef_odef_os_err = 0;
    let mut sdef_oindef_os_err = 0;
    let mut sindef_odef_os_err = 0;
    let mut sindef_oindef_os_err = 0;

    for (mut parsed_sent, mut gold_sent) in parsed.iter().zip(gold.iter()) {
        let mut gold_sent = gold_sent.clone();
        let mut parsed_sent = parsed_sent.clone();
        let mut head_verb_args = HashMap::new();

        for i in 0..gold_sent.len() {
            let gold_token = &gold_sent[i];

            if let Some(gold_lemma) = gold_token.lemma() {
                let gold_pos = gold_token.pos().expect("No pos");
                let gold_deprel = gold_token.head_rel().expect("No deprel");
                let gold_head = gold_token.head().expect("No head");

                let token = &parsed_sent[i];
                let token_deprel = token.head_rel().expect("No deprel");
                let token_head = token.head().expect("No head");

                if gold_head > 0 && token_head > 0 {
                    let gold_head_token = &gold_sent[gold_head - 1];
                    let gold_head_deprel = gold_head_token.head_rel().expect("No deprel");
                    let gold_head_head = gold_head_token.head().expect("No head");

                    let token_head_token = &parsed_sent[token_head - 1];
                    let token_head_deprel = token_head_token.head_rel().expect("No deprel");
                    let token_head_head = token_head_token.head().expect("No head");

                    if gold_deprel == "det" && (gold_head_deprel == "nsubj" || gold_head_deprel == "obj" || gold_head_deprel == "iobj") {

                        let def = if dutch && (gold_pos == "DET-bep" || gold_pos == "DET-aanw") {
                            true
                        } else if dutch {
                            false
                        } else if german && (gold_lemma == "der" || gold_lemma == "die" || gold_lemma == "das" || gold_lemma == "der|die|das" || gold_pos.ends_with("PDAT")) {
                            true
                        } else if german {
                            false
                        } else {
                            eprintln!("Again: Choose a language!");
                            false
                        };

                        let verb_idx = gold_head_head;
                        if gold_head_deprel == "nsubj" {
                            let mut entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 7]);
                            add_def_indef_entry(def, &mut entry, 0, 1, gold_head);
                            if gold_head_deprel != token_head_deprel || gold_head_head != *&token_head_head {
                                entry[6] += 1;
                            }
                        } else if gold_head_deprel == "obj" {
                            let mut entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 7]);
                            add_def_indef_entry(def, &mut entry, 2, 3, gold_head);
                            if gold_head_deprel != token_head_deprel || gold_head_head != *&token_head_head {
                                entry[6] += 1;
                            }
                        } else if gold_head_deprel == "iobj" {
                            let mut entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 7]);
                            add_def_indef_entry(def, &mut entry, 4, 5, gold_head);
                            if gold_head_deprel != token_head_deprel || gold_head_head != *&token_head_head {
                                entry[6] += 1;
                            }
                        }
                    } else if gold_pos.starts_with("NOUN") && (gold_deprel == "nsubj" || gold_deprel == "obj" || gold_deprel == "iobj")  {    // Plural nouns and proper names

                        let mut verb_idx = gold_head - 1;
                        if gold_deprel == "nsubj" {
                            let mut entry = head_verb_args.entry(verb_idx + 1).or_insert(vec![0; 7]);
                            if entry[0] == 0 && entry[1] == 0 {
                                add_def_indef_entry(false, &mut entry, 0, 1, i + 1);
                                if gold_deprel != token_deprel || gold_head != *&token_head {
                                    entry[6] += 1;
                                }
                            }
                        } else if gold_deprel == "obj" {
                            let mut entry = head_verb_args.entry(verb_idx + 1).or_insert(vec![0; 7]);
                            if entry[2] == 0 && entry[3] == 0 {
                                add_def_indef_entry(false, &mut entry, 2, 3, i + 1);
                                if gold_deprel != token_deprel || gold_head != *&token_head {
                                    entry[6] += 1;
                                }
                            }
                        } else if gold_deprel == "iobj" {
                            let mut entry = head_verb_args.entry(verb_idx + 1).or_insert(vec![0; 7]);
                            if entry[4] == 0 && entry[5] == 0 {
                                add_def_indef_entry(false, &mut entry, 4, 5, i + 1);
                                if gold_deprel != token_deprel || gold_head != *&token_head {
                                    entry[6] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        get_def_distribution(
            &head_verb_args, &mut num_hits, las, debug,
            &mut sdef_odef_so, &mut sdef_odef_so_err,
            &mut sdef_oindef_so, &mut sdef_oindef_so_err,
            &mut sindef_odef_so, &mut sindef_odef_so_err,
            &mut sindef_oindef_so, &mut sindef_oindef_so_err,
            &mut sdef_odef_os, &mut sdef_odef_os_err,
            &mut sdef_oindef_os, &mut sdef_oindef_os_err,
            &mut sindef_odef_os, &mut sindef_odef_os_err,
            &mut sindef_oindef_os, &mut sindef_oindef_os_err
        );
    }
    println!("UD");
    print_def_res(
        num_hits, las,
        sdef_odef_so, sdef_odef_so_err,
        sdef_oindef_so, sdef_oindef_so_err,
        sindef_oindef_so, sindef_oindef_so_err,
        sindef_odef_so, sindef_odef_so_err,
        sdef_odef_os, sdef_odef_os_err,
        sdef_oindef_os, sdef_oindef_os_err,
        sindef_odef_os, sindef_odef_os_err,
        sindef_oindef_os, sindef_oindef_os_err
    );
}

fn add_def_indef_entry(def: bool, entry: &mut Vec<usize>, idx_def: usize, idx_indef: usize, val: usize) {
    if def {
        entry[idx_def] = val;
    } else {
        entry[idx_indef] = val;
    }
}

fn get_def_distribution(
    head_verb_args: &HashMap<usize, Vec<usize>>, num_hits: &mut usize, las: bool, debug: bool,
    sdef_odef_so: &mut usize, sdef_odef_so_err: &mut usize,
    sdef_oindef_so: &mut usize, sdef_oindef_so_err: &mut usize,
    sindef_odef_so: &mut usize, sindef_odef_so_err: &mut usize,
    sindef_oindef_so: &mut usize, sindef_oindef_so_err: &mut usize,
    sdef_odef_os: &mut usize, sdef_odef_os_err: &mut usize,
    sdef_oindef_os: &mut usize, sdef_oindef_os_err: &mut usize,
    sindef_odef_os: &mut usize, sindef_odef_os_err: &mut usize,
    sindef_oindef_os: &mut usize, sindef_oindef_os_err: &mut usize
) {
    for (head_verb_idx, verb_args) in head_verb_args.iter() {
        let subj_def_idx = verb_args[0];
        let subj_indef_idx = verb_args[1];
        let obj_def_idx = verb_args[2];
        let obj_indef_idx = verb_args[3];
        let obj2_def_idx = verb_args[4];
        let obj2_indef_idx = verb_args[5];
        let err_cnt = verb_args[6];

        if (subj_def_idx > 0 || subj_indef_idx > 0 ) && (obj_def_idx > 0 || obj_indef_idx > 0) {
            *num_hits += 1;
            if debug {
                eprintln!("{} {} {} {} {} {}(2) {}", subj_def_idx, subj_indef_idx, obj_def_idx, obj_indef_idx, obj2_def_idx, obj2_indef_idx, err_cnt);
            }

            // Subject-object pairs
            let occ = if las {
                2
            } else {
                1
            };
            if subj_def_idx > 0 && obj_def_idx > 0 {
                if subj_def_idx < obj_def_idx {
                    *sdef_odef_so += occ;
                    *sdef_odef_so_err += err_cnt;
                } else {
                    *sdef_odef_os += occ;
                    *sdef_odef_os_err += err_cnt;
                }
            } else if subj_def_idx > 0 && obj_indef_idx > 0 {
                if subj_def_idx < obj_indef_idx {
                    *sdef_oindef_so += occ;
                    *sdef_oindef_so_err += err_cnt;
                } else {
                    *sdef_oindef_os += occ;
                    *sdef_oindef_os_err += err_cnt;
                }

            } else if subj_indef_idx > 0 && obj_def_idx > 0 {
                if subj_indef_idx < obj_def_idx {
                    *sindef_odef_so += occ;
                    *sindef_odef_so_err += err_cnt;
                } else {
                    *sindef_odef_os += occ;
                    *sindef_odef_os_err += err_cnt;
                }

            } else if subj_indef_idx > 0 && obj_indef_idx > 0 {
                if subj_indef_idx < obj_indef_idx {
                    *sindef_oindef_so += occ;
                    *sindef_oindef_so_err += err_cnt;
                } else {
                    *sindef_oindef_os += occ;
                    *sindef_oindef_os_err += err_cnt;
                }
            } else {
                eprintln!("{} {} {} {} {} {}(2) {}", subj_def_idx, subj_indef_idx, obj_def_idx, obj_indef_idx, obj2_def_idx, obj2_indef_idx, err_cnt);
            }

            if obj2_def_idx > 0 || obj2_indef_idx > 0 {
                *num_hits += 1;

                // Second object
                if subj_def_idx > 0 && obj2_def_idx > 0 {
                    if subj_def_idx < obj2_def_idx {
                        *sdef_odef_so += occ;
                        *sdef_odef_so_err += err_cnt;
                    } else {
                        *sdef_odef_os += occ;
                        *sdef_odef_os_err += err_cnt;
                    }
                } else if subj_def_idx > 0 && obj2_indef_idx > 0 {
                    if subj_def_idx < obj2_indef_idx {
                        *sdef_oindef_so += occ;
                        *sdef_oindef_so_err += err_cnt;
                    } else {
                        *sdef_oindef_os += occ;
                        *sdef_oindef_os_err += err_cnt;
                    }

                } else if subj_indef_idx > 0 && obj2_def_idx > 0 {
                    if subj_indef_idx < obj2_def_idx {
                        *sindef_odef_so += occ;
                        *sindef_odef_so_err += err_cnt;
                    } else {
                        *sindef_odef_os += occ;
                        *sindef_odef_os_err += err_cnt;
                    }

                } else if subj_indef_idx > 0 && obj2_indef_idx > 0 {
                    if subj_indef_idx < obj2_indef_idx {
                        *sindef_oindef_so += occ;
                        *sindef_oindef_so_err += err_cnt;
                    } else {
                        *sindef_oindef_os += occ;
                        *sindef_oindef_os_err += err_cnt;
                    }
                } else {
                    eprintln!("{} {} {} {} {} {}(2) {}", subj_def_idx, subj_indef_idx, obj_def_idx, obj_indef_idx, obj2_def_idx, obj2_indef_idx, err_cnt);
                }
            }
        }
    }
    if debug {
        eprintln!("---");
    }
}

fn print_def_res(
    num_hits: usize, las: bool,
    sdef_odef_so: usize, sdef_odef_so_err: usize,
    sdef_oindef_so: usize, sdef_oindef_so_err: usize,
    sindef_oindef_so: usize, sindef_oindef_so_err: usize,
    sindef_odef_so: usize, sindef_odef_so_err: usize,
    sdef_odef_os: usize, sdef_odef_os_err: usize,
    sdef_oindef_os: usize, sdef_oindef_os_err: usize,
    sindef_odef_os: usize, sindef_odef_os_err: usize,
    sindef_oindef_os: usize, sindef_oindef_os_err: usize
) {
    println!("Hits: {}", num_hits);

    if las {
        println!("Subject,Object,Word order,SVO tokens,Error count,LAS\n\
        S-def,O-def,SO,{},{},{}\n\
        S-def,O-indef,SO,{},{},{}\n\
        S-indef,O-def,SO,{},{},{}\n\
        S-indef,O-indef,SO,{},{},{}\n\
        S-def,O-def,OS,{},{},{}\n\
        S-def,O-indef,OS,{},{},{}\n\
        S-indef,O-def,OS,{},{},{}\n\
        S-indef,O-indef,OS,{},{},{}\n",
                 sdef_odef_so, sdef_odef_so_err, 1.0 - sdef_odef_so_err as f32 / sdef_odef_so as f32,
                 sdef_oindef_so, sdef_oindef_so_err, 1.0 - sdef_oindef_so_err as f32 / sdef_oindef_so as f32,
                 sindef_odef_so, sindef_odef_so_err, 1.0 - sindef_odef_so_err as f32 / sindef_odef_so as f32,
                 sindef_oindef_so, sindef_oindef_so_err, 1.0 - sindef_oindef_so_err as f32 / sindef_oindef_so as f32,
                 sdef_odef_os, sdef_odef_os_err, 1.0 - sdef_odef_os_err as f32 / sdef_odef_os as f32,
                 sdef_oindef_os, sdef_oindef_os_err, 1.0 - sdef_oindef_os_err as f32 / sdef_oindef_os as f32,
                 sindef_odef_os, sindef_odef_os_err, 1.0 - sindef_odef_os_err as f32 / sindef_odef_os as f32,
                 sindef_oindef_os, sindef_oindef_os_err, 1.0 - sindef_oindef_os_err as f32 / sindef_oindef_os as f32
        );
    } else {
        println!("Subject,Object,Word order,Frequency\n\
        S-def,O-def,SO,{}\n\
        S-def,O-indef,SO,{}\n\
        S-indef,O-def,SO,{}\n\
        S-indef,O-indef,SO,{}\n\
        S-def,O-def,OS,{}\n\
        S-def,O-indef,OS,{}\n\
        S-indef,O-def,OS,{}\n\
        S-indef,O-indef,OS,{}\n",
                 sdef_odef_so,
                 sdef_oindef_so,
                 sindef_odef_so,
                 sindef_oindef_so,
                 sdef_odef_os,
                 sdef_oindef_os,
                 sindef_odef_os,
                 sindef_oindef_os
        );
    }
}