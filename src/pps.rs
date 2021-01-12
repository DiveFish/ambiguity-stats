extern crate conllx;

use conllx::Token;
use std::collections::{BTreeMap,HashMap};

/// Filter prepositional phrases by the topological field in which they occur.
pub fn get_topofields(text: &[Vec<Token>], ud: bool) {
    let fields = &["C", "LV", "VF", "LK", "MF", "NF", "VC"];

    let tf_ident = if ud {
        "TopoField"
    } else {
        "tf"
    };

    let mut c: Vec<usize> = vec![0; 7]; //[0]: C, [1]: LV, [2]: VF, [3]: LK, [4]: MF, [5]: NF, [6]: VC
    let mut lv: Vec<usize> = vec![0; 7];
    let mut vf: Vec<usize> = vec![0; 7];
    let mut lk: Vec<usize> = vec![0; 7];
    let mut mf: Vec<usize> = vec![0; 7];
    let mut nf: Vec<usize> = vec![0; 7];
    let mut vc: Vec<usize> = vec![0; 7];

    let mut pp_count = 0;
    let mut unkn_count = 0;

    for sent in text.iter() {
        for i in 0..sent.len() {
            let token = &sent[i];

            let deprel = token.head_rel().expect("No deprel");
            let head_idx = token.head().expect("No head");

            let is_pp = if ud {
                if head_idx > 0 { // Head is not ROOT token
                    let head_deprel = &sent[head_idx-1].head_rel().expect("No deprel");
                    let pos = token.pos().expect("No pos");
                    deprel == "case" && pos.starts_with("ADP") && (head_deprel == &"nmod" || head_deprel == &"obl")
                } else {
                    false
                }
            } else {
                deprel == "PP" || deprel == "OBJP"
            };


            if is_pp {
                let features = token
                    .features()
                    .map(|f| f.as_map().clone())
                    .unwrap_or(BTreeMap::new());

                let mut tf = "";

                match features
                    .get(tf_ident) {
                    Some(Some(tf_feat)) => {
                        if tf_feat.len() > 2 {
                            let fields = tf_feat.split("-");
                            let field_vec = fields.collect::<Vec<_>>();
                            tf = field_vec[field_vec.len() - 1];
                        } else {
                            tf = tf_feat;
                        }
                    },
                    _ => tf = "UNK"
                }

                if head_idx > 0 { // Head is not ROOT token
                    let head = &sent[head_idx - 1];
                    let mut head_features = head
                        .features()
                        .map(|f| f.as_map().clone())
                        .unwrap_or(BTreeMap::new());

                    if ud { // PPs in UD: head >nmod/obl noun >case prep
                        let head_head_idx = head.head().expect("No head");
                        if head_head_idx > 0 {
                            let head_head = &sent[head_head_idx - 1];
                            head_features = head_head
                                .features()
                                .map(|f| f.as_map().clone())
                                .unwrap_or(BTreeMap::new());
                        } else {
                            continue;
                        }
                    }

                    let mut head_tf = "";
                    match head_features
                        .get(tf_ident) {
                        Some(Some(tf_feat)) => {
                            if tf_feat.len() > 2 {
                                let fields = tf_feat.split("-");
                                let field_vec = fields.collect::<Vec<_>>();
                                head_tf = field_vec[field_vec.len() - 1];
                            } else {
                                head_tf = tf_feat;
                            }
                        },
                        _ => head_tf = "UNK"
                    }

                    if (tf != "UNK") & (head_tf != "UNK") {
                        if tf == "C" {
                            if head_tf == "C" {
                                c[0] += 1;
                            } else if head_tf == "LV" {
                                c[1] += 1;
                            } else if head_tf == "VF" {
                                c[2] += 1;
                            } else if head_tf == "LK" {
                                c[3] += 1;
                            } else if head_tf == "MF" {
                                c[4] += 1;
                            } else if head_tf == "NF" {
                                c[5] += 1;
                            } else if head_tf == "VC" {
                                c[6] += 1;
                            }
                        } else if tf == "LV" {
                            if head_tf == "C" {
                                lv[0] += 1;
                            } else if head_tf == "LV" {
                                lv[1] += 1;
                            } else if head_tf == "VF" {
                                lv[2] += 1;
                            } else if head_tf == "LK" {
                                lv[3] += 1;
                            } else if head_tf == "MF" {
                                lv[4] += 1;
                            } else if head_tf == "NF" {
                                lv[5] += 1;
                            } else if head_tf == "VC" {
                                lv[6] += 1;
                            }
                        } else if tf == "VF" {
                            if head_tf == "C" {
                                vf[0] += 1;
                            } else if head_tf == "LV" {
                                vf[1] += 1;
                            } else if head_tf == "VF" {
                                vf[2] += 1;
                            } else if head_tf == "LK" {
                                vf[3] += 1;
                            } else if head_tf == "MF" {
                                vf[4] += 1;
                            } else if head_tf == "NF" {
                                vf[5] += 1;
                            } else if head_tf == "VC" {
                                vf[6] += 1;
                            }
                        } else if tf == "LK" {
                            if head_tf == "C" {
                                lk[0] += 1;
                            } else if head_tf == "LV" {
                                lk[1] += 1;
                            } else if head_tf == "VF" {
                                lk[2] += 1;
                            } else if head_tf == "LK" {
                                lk[3] += 1;
                            } else if head_tf == "MF" {
                                lk[4] += 1;
                            } else if head_tf == "NF" {
                                lk[5] += 1;
                            } else if head_tf == "VC" {
                                lk[6] += 1;
                            }
                        } else if tf == "MF" {
                            if head_tf == "C" {
                                mf[0] += 1;
                            } else if head_tf == "LV" {
                                mf[1] += 1;
                            } else if head_tf == "VF" {
                                mf[2] += 1;
                            } else if head_tf == "LK" {
                                mf[3] += 1;
                            } else if head_tf == "MF" {
                                mf[4] += 1;
                            } else if head_tf == "NF" {
                                mf[5] += 1;
                            } else if head_tf == "VC" {
                                mf[6] += 1;
                            }
                        } else if tf == "NF" {
                            if head_tf == "C" {
                                nf[0] += 1;
                            } else if head_tf == "LV" {
                                nf[1] += 1;
                            } else if head_tf == "VF" {
                                nf[2] += 1;
                            } else if head_tf == "LK" {
                                nf[3] += 1;
                            } else if head_tf == "MF" {
                                nf[4] += 1;
                            } else if head_tf == "NF" {
                                nf[5] += 1;
                            } else if head_tf == "VC" {
                                nf[6] += 1;
                            }
                        } else if tf == "VC" {
                            if head_tf == "C" {
                                vc[0] += 1;
                            } else if head_tf == "LV" {
                                vc[1] += 1;
                            } else if head_tf == "VF" {
                                vc[2] += 1;
                            } else if head_tf == "LK" {
                                vc[3] += 1;
                            } else if head_tf == "MF" {
                                vc[4] += 1;
                            } else if head_tf == "NF" {
                                vc[5] += 1;
                            } else if head_tf == "VC" {
                                vc[6] += 1;
                            }
                        }
                        if fields.contains(&tf) & fields.contains(&head_tf) {
                            pp_count += 1;
                        }
                    } else {
                        unkn_count = unkn_count + 1;
                    }
                }
            }
        }
    }

    println!("\nPP count: {}", pp_count);
    println!("UNK count: {}", unkn_count);

    println!("--Complementizer--");
    for i in 0..7 {
        println!("{}: {}", i + 1, c[i]);
    }
    println!("--LV complex--");
    for i in 0..7 {
        println!("{}: {}", i + 1, lv[i]);
    }
    println!("--Vorfeld--");
    for i in 0..7 {
        println!("{}: {}", i + 1, vf[i]);
    }
    println!("--Linke Klammer--");
    for i in 0..7 {
        println!("{}: {}", i + 1, lk[i]);
    }
    println!("--Mittelfeld--");
    for i in 0..7 {
        println!("{}: {}", i + 1, mf[i]);
    }
    println!("--Nachfeld--");
    for i in 0..7 {
        println!("{}: {}", i + 1, nf[i]);
    }
    println!("--Verbal complex--");
    for i in 0..7 {
        println!("{}: {}", i + 1, vc[i]);
    }
}

/// Filter prepositional phrases by the topological field in which they occur.
pub fn get_topofields_examples(text: &[Vec<Token>], ud: bool, print_examples: bool) {
    let _fields = &["VF", "LK", "MF", "RK", "NF"];

    let tf_ident = if ud {
        "TopoField"
    } else {
        "tf"
    };

    let mut example_sents_c: Vec<Vec<Vec<Token>>> = vec![vec![]; 7]; //[0]: C, [1]: LV, [2]: VF, [3]: LK, [4]: MF, [5]: NF, [6]: VC
    let mut example_sents_lv: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_vf: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_lk: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_mf: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_nf: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_vc: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];

    let mut pp_count = 0;
    let mut unkn_count = 0;

    for sent in text.iter() {
        for i in 0..sent.len() {
            let token = &sent[i];

            let deprel = token.head_rel().expect("No deprel");
            let head_idx = token.head().expect("No head");

            let is_pp = if ud {
                if head_idx > 0 { // Head is not ROOT token
                    let head_deprel = &sent[head_idx-1].head_rel().expect("No deprel");
                    let pos = token.pos().expect("No pos");
                    deprel == "case" && pos.starts_with("ADP") && (head_deprel == &"nmod" || head_deprel == &"obl")
                } else {
                    false
                }
            } else {
                deprel == "PP" || deprel == "OBJP"
            };


            if is_pp {
                let features = token
                    .features()
                    .map(|f| f.as_map().clone())
                    .unwrap_or(BTreeMap::new());

                let mut tf = "";

                match features
                    .get(tf_ident) {
                    Some(Some(tf_feat)) => tf = tf_feat,
                    _ => tf = "UNK"
                }

                if head_idx > 0 { // Head is not ROOT token
                    let head = &sent[head_idx - 1];
                    let mut head_features = head
                        .features()
                        .map(|f| f.as_map().clone())
                        .unwrap_or(BTreeMap::new());

                    if ud { // PPs in UD: head >nmod/obl noun >case prep
                        let head_head_idx = head.head().expect("No head");
                        if head_head_idx > 0 {
                            let head_head = &sent[head_head_idx - 1];
                            head_features = head_head
                                .features()
                                .map(|f| f.as_map().clone())
                                .unwrap_or(BTreeMap::new());
                        } else {
                            continue;
                        }
                    }

                    let mut head_tf = "";
                    match head_features
                        .get(tf_ident) {
                        Some(Some(tf_feat)) => head_tf = tf_feat,
                        _ => head_tf = "UNK"
                    }

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
                    } else if tf == "LK" {
                        if head_tf == "C" {
                            example_sents_lk[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_lk[1].push(sent.to_owned());
                        } else if head_tf == "VF" {
                            example_sents_lk[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_lk[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_lk[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_lk[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_lk[6].push(sent.to_owned());
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
        if (example_sents_c[i].len() > 10) & print_examples {
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
        if (example_sents_lv[i].len() > 10) & print_examples {
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
        if (example_sents_vf[i].len() > 10) & print_examples {
            for j in 0..10 {
                for token in example_sents_vf[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
    println!("--Linke Klammer--");
    for i in 0..7 {
        println!("{}: {}", i + 1, example_sents_lk[i].len());
        if (example_sents_lk[i].len() > 10) & print_examples {
            for j in 0..10 {
                for token in example_sents_lk[i][j].iter() {
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
        if (example_sents_mf[i].len() > 10) & print_examples {
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
        if (example_sents_nf[i].len() > 10) & print_examples {
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
        if (example_sents_vc[i].len() > 10) & print_examples {
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