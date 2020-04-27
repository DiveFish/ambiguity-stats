extern crate conllx;

use conllx::Token;
use std::collections::HashMap;
use verb::reattach_auxiliary;

/// Orders distnguished along these lines:
/// VF[S]LK[V]MF[O]
/// VF[O]LK[V]MF[S]
/// LK[V]MF[SO]
/// LK[V]MF[OS]
/// MF[SO]VC[V]
/// MF[OS]VC[V]
pub fn order_freq(parsed: &mut Vec<Vec<Token>>, gold: &mut Vec<Vec<Token>>) {

    let mut svo = 0;
    let mut ovs = 0;
    let mut vso = 0;
    let mut vos = 0;
    let mut sov = 0;
    let mut osv = 0;
    let mut svo_err = 0;
    let mut ovs_err = 0;
    let mut vso_err = 0;
    let mut vos_err = 0;
    let mut sov_err = 0;
    let mut osv_err = 0;

    for (mut parsed_sent, mut gold_sent) in parsed.iter().zip(gold.iter()) {
        let mut gold_sent = gold_sent.clone();
        let mut parsed_sent = parsed_sent.clone();
        reattach_auxiliary(&mut gold_sent);
        reattach_auxiliary(&mut parsed_sent);
        let mut head_verb_args = HashMap::new();

        for i in 0..gold_sent.len() {
            let gold_token = &gold_sent[i];
            let gold_deprel = gold_token.head_rel().expect("No deprel");
            let gold_head = gold_token.head().expect("No head");

            let token = &parsed_sent[i];
            let token_deprel = token.head_rel().expect("No deprel");
            let token_head = token.head().expect("No head");

            if gold_deprel == "SUBJ" || gold_deprel == "OBJA" {

                let verb_idx = gold_head;
                if gold_deprel == "SUBJ" {
                    let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 3]);
                    entry[0] = i;

                    if gold_deprel != token_deprel || gold_head != token_head {
                        entry[2] += 1;
                    }
                } else if gold_deprel == "OBJA" {
                    let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 3]);
                    entry[1] = i;

                    if gold_deprel != token_deprel || gold_head != token_head {
                        entry[2] += 1;
                    }
                }
            }
        }

        for (head_verb_idx, verb_args) in head_verb_args.iter() {
            let subj_idx = verb_args[0];
            let obj_idx = verb_args[1];
            let err_cnt = verb_args[2];

            if subj_idx > 0 && obj_idx > 0 { // Clause has a subject and an object

                if subj_idx < *head_verb_idx && *head_verb_idx < obj_idx {
                    svo += 2;
                    svo_err += err_cnt;

                } else if obj_idx < *head_verb_idx && *head_verb_idx < subj_idx {
                    ovs += 2;
                    ovs_err += err_cnt;

                } else if *head_verb_idx < subj_idx && subj_idx < obj_idx {
                    vso += 2;
                    vso_err += err_cnt;

                } else if *head_verb_idx < obj_idx && obj_idx < subj_idx {
                    vos += 2;
                    vos_err += err_cnt;

                } else if subj_idx < obj_idx && obj_idx < *head_verb_idx {
                    sov += 2;
                    sov_err += err_cnt;

                } else if obj_idx < subj_idx && subj_idx < *head_verb_idx {
                    osv += 2;
                    osv_err += err_cnt;
                }
            }
        }
    }
    println!("SVO {} {}\nOVS {} {}\nVSO {} {}\nVOS {} {}\nSOV {} {}\nOSV {} {}\n",
             svo, 1.0 - svo_err as f32/svo as f32,
             ovs, 1.0 - ovs_err as f32/ovs as f32,
             vso, 1.0 - vso_err as f32/vso as f32,
             vos, 1.0 - vos_err as f32/vos as f32,
             sov, 1.0 - sov_err as f32/sov as f32,
             osv, 1.0 - osv_err as f32/osv as f32
    );
}


/// Definiteness combinations distinguished along these lines:
/// 1. subject definite - object definite
/// 2. subject definite - object indefinite
/// 3. subject indefinite - object definite
/// 4. subject indefinite - object indefinite
pub fn definiteness(parsed: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>) {

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
        reattach_auxiliary(&mut gold_sent);
        reattach_auxiliary(&mut parsed_sent);
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

                    if gold_deprel == "DET" && (gold_head_deprel == "SUBJ" || gold_head_deprel == "OBJA") {
                        let def = if gold_lemma == "der" || gold_lemma == "die" || gold_lemma == "das" || gold_lemma == "der|die|das" || gold_pos == "PDAT" {
                            true
                        } else {
                            //if lemma == "eine" || lemma == "ein" || pos == "PIDAT" || pos == "PIAT" || pos == "PWAT" {
                            false
                        };

                        let verb_idx = gold_head_head;
                        if gold_head_deprel == "SUBJ" {
                            let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);
                            if def {
                                entry[0] = gold_head;
                            } else {
                                entry[1] = gold_head;
                            }
                            if gold_head_deprel != token_head_deprel || gold_head_head != *&token_head_head {
                                entry[4] += 1;
                            }
                        } else if gold_head_deprel == "OBJA" {
                            let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 5]);
                            if def {
                                entry[2] = gold_head;
                            } else {
                                entry[3] = gold_head;
                            }
                            if gold_head_deprel != token_head_deprel || gold_head_head != *&token_head_head {
                                entry[4] += 1;
                            }
                        }
                    }
                }
            }
        }

        for (head_verb_idx, verb_args) in head_verb_args.iter() {
            let subj_def_idx = verb_args[0];
            let subj_indef_idx = verb_args[1];
            let obj_def_idx = verb_args[2];
            let obj_indef_idx = verb_args[3];
            let err_cnt = verb_args[4];

            let subj_obj = if (subj_def_idx > 0 || subj_indef_idx > 0 ) && (obj_def_idx > 0 || obj_indef_idx > 0) {
                true
            } else {
                false
            };

            if subj_obj {
                if subj_def_idx > 0 && obj_def_idx > 0 {
                    if subj_def_idx < obj_def_idx {
                        sdef_odef_so += 2;
                        sdef_odef_so_err += err_cnt;
                    } else {
                        sdef_odef_os += 2;
                        sdef_odef_os_err += err_cnt;
                    }
                } else if subj_def_idx > 0 && obj_indef_idx > 0 {
                    if subj_def_idx < obj_indef_idx {
                        sdef_oindef_so += 2;
                        sdef_oindef_so_err += err_cnt;
                    } else {
                        sdef_oindef_os += 2;
                        sdef_oindef_os_err += err_cnt;
                    }

                } else if subj_indef_idx > 0 && obj_def_idx > 0 {
                    if subj_indef_idx < obj_def_idx {
                        sindef_odef_so += 2;
                        sindef_odef_so_err += err_cnt;
                    } else {
                        sindef_odef_os += 2;
                        sindef_odef_os_err += err_cnt;
                    }

                } else if subj_indef_idx > 0 && obj_indef_idx > 0 {
                    if subj_indef_idx < obj_indef_idx {
                        sindef_oindef_so += 2;
                        sindef_oindef_so_err += err_cnt;
                    } else {
                        sindef_oindef_os += 2;
                        sindef_oindef_os_err += err_cnt;
                    }

                }
            }
        }
    }
    println!("S-def O-def, SO {} {}\n\
              S-def O-indef, SO {} {}\n\
              S-indef O-def, SO {} {}\n\
              S-indef O-indef, SO {} {}\n\
              S-def O-def, OS {} {}\n\
              S-def O-indef, OS {} {}\n\
              S-indef O-def, OS {} {}\n\
              S-indef O-indef, OS {} {}\n",
             sdef_odef_so, 1.0 - sdef_odef_so_err as f32 / sdef_odef_so as f32,
             sdef_oindef_so, 1.0 - sdef_oindef_so_err as f32 / sdef_oindef_so as f32,
             sindef_odef_so, 1.0 - sindef_odef_so_err as f32 / sindef_odef_so as f32,
             sindef_oindef_so, 1.0 - sindef_oindef_so_err as f32 / sindef_oindef_so as f32,
             sdef_odef_os, 1.0 - sdef_odef_os_err as f32 / sdef_odef_os as f32,
             sdef_oindef_os, 1.0 - sdef_oindef_os_err as f32 / sdef_oindef_os as f32,
             sindef_odef_os, 1.0 - sindef_odef_os_err as f32 / sindef_odef_os as f32,
             sindef_oindef_os, 1.0 - sindef_oindef_os_err as f32 / sindef_oindef_os as f32,
    );
}