extern crate conllx;

use conllx::{Token, Features};
use std::collections::BTreeMap;

pub fn get_topofields(text: &[Vec<Token>]) {
    let fields = &["VF", "LK", "MF", "RK", "NF"];

    let mut example_sents_C: Vec<Vec<Vec<Token>>> = vec![vec![]; 7]; //[0]: C, [1]: LV, [2]: VF, [3]: LK, [4]: MF, [5]: NF, [6]: VC
    let mut example_sents_LV: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_VF: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_MF: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_NF: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];
    let mut example_sents_VC: Vec<Vec<Vec<Token>>> = vec![vec![]; 7];

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
                let tf = features.get("tf").expect("No features").clone().expect("No topo field info");

                if token.head().expect("No head") > 0 { // head is not ROOT token
                    let head = &sent[token.head().expect("No head")-1];
                    let head_features = head
                        .features()
                        .map(|f| f.as_map().clone())
                        .unwrap_or(BTreeMap::new());
                    let head_tf = head_features.get("tf").expect("No features").clone().expect("No topo field info");
                    if tf == "C" {
                        if head_tf == "C" {
                            example_sents_C[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_C[1].push(sent.to_owned());
                        }  else if head_tf == "VF" {
                            example_sents_C[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_C[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_C[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_C[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_C[6].push(sent.to_owned());
                        } else if head_tf == "UK" {
                            unkn_count = unkn_count + 1;
                        }
                    } else if tf == "LV" {
                        if head_tf == "C" {
                            example_sents_LV[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_LV[1].push(sent.to_owned());
                        }   else if head_tf == "VF" {
                            example_sents_LV[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_LV[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_LV[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_LV[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_LV[6].push(sent.to_owned());
                        } else if head_tf == "UK" {
                            unkn_count = unkn_count + 1;
                        }
                    } else if tf == "VF" {
                        if head_tf == "C" {
                            example_sents_VF[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_VF[1].push(sent.to_owned());
                        }   else if head_tf == "VF" {
                            example_sents_VF[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_VF[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_VF[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_VF[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_VF[6].push(sent.to_owned());
                        } else if head_tf == "UK" {
                            unkn_count = unkn_count + 1;
                        }
                    } else if tf == "MF" {
                        if head_tf == "C" {
                            example_sents_MF[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_MF[1].push(sent.to_owned());
                        }   else if head_tf == "VF" {
                            example_sents_MF[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_MF[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_MF[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_MF[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_MF[6].push(sent.to_owned());
                        } else if head_tf == "UK" {
                            unkn_count = unkn_count + 1;
                        }
                    } else if tf == "NF" {
                        if head_tf == "C" {
                            example_sents_NF[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_NF[1].push(sent.to_owned());
                        }   else if head_tf == "VF" {
                            example_sents_NF[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_NF[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_NF[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_NF[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_NF[6].push(sent.to_owned());
                        } else if head_tf == "UK" {
                            unkn_count = unkn_count + 1;
                        }
                    } else if tf == "VC" {
                        if head_tf == "C" {
                            example_sents_VC[0].push(sent.to_owned());
                        } else if head_tf == "LV" {
                            example_sents_VC[1].push(sent.to_owned());
                        }   else if head_tf == "VF" {
                            example_sents_VC[2].push(sent.to_owned());
                        } else if head_tf == "LK" {
                            example_sents_VC[3].push(sent.to_owned());
                        } else if head_tf == "MF" {
                            example_sents_VC[4].push(sent.to_owned());
                        } else if head_tf == "NF" {
                            example_sents_VC[5].push(sent.to_owned());
                        } else if head_tf == "VC" {
                            example_sents_VC[6].push(sent.to_owned());
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
        println!("{}: {}", i+1, example_sents_C[i].len());
        if example_sents_C[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_C[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
    println!("--LV complex--");
    for i in 0..7 {
        println!("{}: {}", i+1, example_sents_LV[i].len());
        if example_sents_LV[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_LV[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
    println!("--Vorfeld--");
    for i in 0..7 {
        println!("{}: {}", i+1, example_sents_VF[i].len());
        if example_sents_VF[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_VF[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
    println!("--Mittelfeld--");
    for i in 0..7 {
        println!("{}: {}", i+1, example_sents_MF[i].len());
        if example_sents_MF[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_MF[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
    println!("--Nachfeld--");
    for i in 0..7 {
        println!("{}: {}", i+1, example_sents_NF[i].len());
        if example_sents_NF[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_NF[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
    println!("--Verbal complex--");
    for i in 0..7 {
        println!("{}: {}", i+1, example_sents_VC[i].len());
        if example_sents_VC[i].len() > 10 {
            for j in 0..10 {
                for token in example_sents_VC[i][j].iter() {
                    print!("{} ", token.form());
                }
                println!();
            }
        }
        println!();
    }
}