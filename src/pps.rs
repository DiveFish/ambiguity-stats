extern crate conllx;

use conllx::Token;
use std::collections::{BTreeMap,HashMap};

/// Filter prepositional phrases by the topological field in which they occur.
pub fn get_topofields(text: &[Vec<Token>]) {
    let _fields = &["VF", "LK", "MF", "RK", "NF"];

    let mut example_sents_c: Vec<Vec<Vec<Token>>> = vec![vec![]; 7]; //[0]: C, [1]: LV, [2]: VF, [3]: LK, [4]: MF, [5]: NF, [6]: VC
    let mut example_sents_lv: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_vf: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_mf: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_nf: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_vc: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];

    let mut pp_count = 0;
    let mut unkn_count = 0;

    for sent in text.iter() {
        for i in 0..sent.len() {
            let token = &sent[i];

            let deprel = token.head_rel().expect("No deprel");

            if deprel == "PP" || deprel == "OBJP" {
                let features = token
                    .features()
                    .map(|f| f.as_map().clone())
                    .unwrap_or(BTreeMap::new());
                let tf = features
                    .get("tf")
                    .expect("No features")
                    .clone()
                    .expect("No topo field info");

                if token.head().expect("No head") > 0 {
                    // head is not ROOT token
                    let head = &sent[token.head().expect("No head") - 1];
                    let head_features = head
                        .features()
                        .map(|f| f.as_map().clone())
                        .unwrap_or(BTreeMap::new());
                    let head_tf = head_features
                        .get("tf")
                        .expect("No features")
                        .clone()
                        .expect("No topo field info");
                    if tf == "C" {
                        if head_tf == "C" {
                            example_sents_c[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_c[1].push(sent.to_owned());
                        } else if head_tf == "VF" {
                            example_sents_c[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_c[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_c[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_c[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_c[6].push(sent.to_owned());
                        } else if head_tf == "UK" {
                            unkn_count = unkn_count + 1;
                        }
                    } else if tf == "LV" {
                        if head_tf == "C" {
                            example_sents_lv[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_lv[1].push(sent.to_owned());
                        } else if head_tf == "VF" {
                            example_sents_lv[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_lv[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_lv[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_lv[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_lv[6].push(sent.to_owned());
                        } else if head_tf == "UK" {
                            unkn_count = unkn_count + 1;
                        }
                    } else if tf == "VF" {
                        if head_tf == "C" {
                            example_sents_vf[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_vf[1].push(sent.to_owned());
                        } else if head_tf == "VF" {
                            example_sents_vf[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_vf[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_vf[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_vf[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_vf[6].push(sent.to_owned());
                        } else if head_tf == "UK" {
                            unkn_count = unkn_count + 1;
                        }
                    } else if tf == "MF" {
                        if head_tf == "C" {
                            example_sents_mf[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_mf[1].push(sent.to_owned());
                        } else if head_tf == "VF" {
                            example_sents_mf[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_mf[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_mf[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_mf[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_mf[6].push(sent.to_owned());
                        } else if head_tf == "UK" {
                            unkn_count = unkn_count + 1;
                        }
                    } else if tf == "NF" {
                        if head_tf == "C" {
                            example_sents_nf[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_nf[1].push(sent.to_owned());
                        } else if head_tf == "VF" {
                            example_sents_nf[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_nf[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_nf[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_nf[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_nf[6].push(sent.to_owned());
                        } else if head_tf == "UK" {
                            unkn_count = unkn_count + 1;
                        }
                    } else if tf == "VC" {
                        if head_tf == "C" {
                            example_sents_vc[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_vc[1].push(sent.to_owned());
                        } else if head_tf == "VF" {
                            example_sents_vc[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_vc[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_vc[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_vc[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_vc[6].push(sent.to_owned());
                        } else if head_tf == "UK" {
                            unkn_count = unkn_count + 1;
                        }
                    } else if tf == "UK" {
                        unkn_count = unkn_count + 1;
                    }
                    pp_count = pp_count + 1;
                }
            }
        }
    }

    println!("\nPP count: {}", pp_count);
    println!("UK count: {}", unkn_count);

    println!("--Complementizer--");
    for i in 0..7 {
        println!("{}: {}", i + 1, example_sents_c[i].len());
        if example_sents_c[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_c[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
    println!("--LV complex--");
    for i in 0..7 {
        println!("{}: {}", i + 1, example_sents_lv[i].len());
        if example_sents_lv[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_lv[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
    println!("--Vorfeld--");
    for i in 0..7 {
        println!("{}: {}", i + 1, example_sents_vf[i].len());
        if example_sents_vf[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_vf[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
    println!("--Mittelfeld--");
    for i in 0..7 {
        println!("{}: {}", i + 1, example_sents_mf[i].len());
        if example_sents_mf[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_mf[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
    println!("--Nachfeld--");
    for i in 0..7 {
        println!("{}: {}", i + 1, example_sents_nf[i].len());
        if example_sents_nf[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_nf[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
    println!("--Verbal complex--");
    for i in 0..7 {
        println!("{}: {}", i + 1, example_sents_vc[i].len());
        if example_sents_vc[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_vc[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
}

/// Count frequencies of preposition and error per preposition.
pub fn err_by_prep(gold_sent: &[Token], parser_sent: &[Token], prep_errs: &mut HashMap<String, (usize, usize)>) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_head = gold_token.head().expect("No head");

        if gold_deprel == "PP" || gold_deprel == "OBJP" {
            let parser_token = &parser_sent[i];
            let parser_deprel = parser_token.head_rel().expect("No deprel");
            let parser_head = parser_token.head().expect("No head");

            let (pp_freq, pp_err_freq) = prep_errs.entry(gold_token.form().to_string().to_lowercase()).or_insert((0,0));
            *pp_freq += 1;
            if parser_deprel != gold_deprel || parser_head != gold_head {
                *pp_err_freq += 1;
            }
        }
    }
}