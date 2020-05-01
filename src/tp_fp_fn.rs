use conllx::Token;

use std::collections::HashMap;
use std::vec::Vec;

use scores::*;

/// Calculate from a corpus the number of true positives,
/// false positives and false negatives under a criterion
/// given by the passed-on function.
pub fn prec_rec_f1(
    gold_sents: &Vec<Vec<Token>>,
    parser_sents: &Vec<Vec<Token>>,
    print: bool,
    fun: fn(&mut f32, &mut f32, &mut f32, &[Token], &[Token], bool),
) -> (f32, f32, f32) {

    assert_eq!(gold_sents.len(), parser_sents.len());

    let mut true_pos = 0.0;
    let mut false_pos = 0.0;
    let mut false_neg = 0.0;

    for (gold_sent, parser_sent) in gold_sents.iter().zip(parser_sents.iter()) {

        assert_eq!(gold_sent.len(), parser_sent.len());
        fun(
            &mut true_pos,
            &mut false_pos,
            &mut false_neg,
            gold_sent,
            parser_sent,
            print,
        );
    }
    println!("Accuracy: {}", 100f32 - (false_pos + false_neg)/ (true_pos / 100f32));

    let precision = precision(true_pos, false_pos);
    let recall = recall(true_pos, false_neg);
    let f1_score = f1_score(precision, recall);
    (precision, recall, f1_score)
}
/// Get true positives, false positives and false negatives
/// for PP attachment.
pub fn pp_acc_comps(
    true_pos: &mut f32,
    false_pos: &mut f32,
    false_neg: &mut f32,
    gold_sent: &[Token],
    parser_sent: &[Token],
    print_sents: bool,
) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_pos =  gold_token.pos().expect("No deprel");
        let gold_headidx = gold_token.head().expect("No head");

        let parser_token = &parser_sent[i];
        let parser_deprel = parser_token.head_rel().expect("No deprel");
        let parser_pos =  parser_token.pos().expect("No deprel");
        let parser_headidx = parser_token.head().expect("No head idx");

        if parser_deprel == "OBJP" || parser_deprel == "PP" || (parser_deprel == "PRED" && parser_pos.starts_with("APP")) {

            if parser_deprel == gold_deprel && parser_headidx == gold_headidx {
                *true_pos += 1.0;
                //println!("TP {:?} ({} {}) -- {:?} ({} {})", gold_token.form(), gold_deprel, gold_headidx, parser_token.form(), parser_deprel, parser_headidx);
            } else {
                *false_pos += 1.0;
                //println!("FP {:?} ({} {}) -- {:?} ({} {})", gold_token.form(), gold_deprel, gold_headidx, parser_token.form(), parser_deprel, parser_headidx);
            }
            /*else if gold_deprel == "OBJP" && parser_deprel != gold_deprel {
                *false_pos += 1.0;
            } else if gold_deprel == "PP" && parser_deprel != gold_deprel {
                *false_pos += 1.0;
            } else if (gold_deprel == "PRED" && gold_pos.starts_with("APP")) && parser_deprel != gold_deprel {
                *false_pos += 1.0;
            } else if parser_headidx != gold_headidx {
                *false_pos += 1.0;
            }
            */
        } else if gold_deprel == "OBJP" || gold_deprel == "PP" || (gold_deprel == "PRED" && gold_pos.starts_with("APP")) {
            *false_neg += 1.0;
            //println!("FN {:?} ({} {}) -- {:?} ({} {})", gold_token.form(), gold_deprel, gold_headidx, parser_token.form(), parser_deprel, parser_headidx);
        }
    }
}

/// Get true positives, false positives and false negatives
/// for PP attachment.
pub fn pp_acc_comps_old(
    true_pos: &mut f32,
    false_pos: &mut f32,
    false_neg: &mut f32,
    gold_sent: &[Token],
    parser_sent: &[Token],
    print_sents: bool,
) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_pos =  gold_token.pos().expect("No deprel");
        let gold_headidx = gold_token.head().expect("No head");

        let parser_token = &parser_sent[i];
        let parser_deprel = parser_token.head_rel().expect("No deprel");
        let parser_pos =  parser_token.pos().expect("No deprel");
        let parser_headidx = parser_token.head().expect("No head idx");

        if gold_deprel == "OBJP" || gold_deprel == "PP" || (gold_deprel == "PRED" && gold_pos.starts_with("APP")) {

            if parser_deprel == gold_deprel && parser_headidx == gold_headidx {
                *true_pos += 1.0;
            } else if parser_deprel != gold_deprel || parser_headidx!= gold_headidx {   // the parser could also predict an OBJP but the gold standard has a PP >> false pos; check by label
                *false_neg += 1.0;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    if gold_headidx != parser_headidx {
                        println!(
                            "\n{} idx (GOLD), {} idx (PARSER)",
                            gold_headidx, parser_headidx
                        );
                    }
                    if gold_deprel != parser_deprel {
                        println!(
                            "\n{} (GOLD), {} (PARSER) at idx {}",
                            gold_deprel, parser_deprel, i
                        );
                    }
                }
            }

        } else if parser_deprel == "OBJP" || parser_deprel == "PP" || (parser_deprel == "PRED" && parser_pos.starts_with("APP")) {
            *false_pos += 1.0;

            if print_sents && gold_sent.len() < 11 {
                println!();
                for token in gold_sent {
                    print!("{} ", token.form());
                }
                if gold_headidx != parser_headidx {
                    println!(
                        "\n{} idx (GOLD), {} idx (PARSER)",
                        gold_headidx, parser_headidx
                    );
                }
                if gold_deprel != parser_deprel {
                    println!(
                        "\n{} (GOLD), {} (PARSER) at idx {}",
                        gold_deprel, parser_deprel, i
                    );
                }
            }
        }
    }
}


/// Get true positives, false positives and false negatives
/// for subject-object inversion.
pub fn inv_acc_comps(
    true_pos: &mut f32,
    false_pos: &mut f32,
    false_neg: &mut f32,
    gold_sent: &[Token],
    parser_sent: &[Token],
    print_sents: bool,
) {
    let mut head_verb_args = HashMap::new();

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_head = gold_token.head().expect("No head");

        let token = &parser_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_deprel == "SUBJ" || gold_deprel.starts_with("OBJ") {
            //TODO: Replace by output of function verb::contentverb()
            let verb_idx = if (gold_head > 0)
                && gold_sent[gold_head - 1]
                .head_rel()
                .expect("No deprel")
                .eq("AUX")
                {
                    // Reattach auxiliary verb to content verb
                    gold_sent[gold_head - 1].head().expect("No head")
                } else {
                gold_head
            };

            if gold_deprel == "SUBJ" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[0] = i;

                if token_deprel.starts_with("OBJ") {
                    entry[1] = i; // SUBJ mistaken for OBJ
                } else if gold_deprel == token_deprel {
                    entry[1] = 1001;
                }
            } else if gold_deprel.starts_with("OBJ") {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[2] = i;

                if token_deprel == "SUBJ" {
                    entry[3] = i; // OBJ mistaken for SUBJ
                } else if gold_deprel == token_deprel {
                    entry[3] = 1001;
                }
            }
        }
    }

    for (_, val) in head_verb_args.iter() {
        let gold_subjidx = val[0];
        let parser_objidx = val[1]; // OBJ but should have been SUBJ
        let gold_objidx = val[2];
        let parser_subjidx = val[3]; // SUBJ but should have been OBJ

        if gold_subjidx > 0 && gold_objidx > 0 {

            // Subject-object inversion not recognized by parser
            if (gold_subjidx > gold_objidx) && (parser_objidx > 0 || parser_subjidx > 0 ) {

                *false_neg += 1.0;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!(
                        "\nSUBJ idx {}, OBJ idx {} (GOLD)",
                        gold_subjidx, gold_objidx
                    );
                    println!(
                        "SUBJ idx {}, OBJ idx {} (PARSER)",
                        parser_subjidx, parser_objidx
                    );
                }
            // No subject-object inversion, but labeled as such by parser
            } else if (gold_subjidx < gold_objidx) && (parser_objidx > 0 || parser_subjidx > 0) {

                *false_pos += 1.0;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!(
                        "\nSUBJ idx {}, OBJ idx {} (GOLD)",
                        gold_subjidx, gold_objidx
                    );
                    println!(
                        "SUBJ idx {}, OBJ idx {} (PARSER)",
                        parser_subjidx, parser_objidx
                    );
                }
            // Subject-object inversion recognized by parser
            } else if (gold_subjidx > gold_objidx) && (parser_objidx > 1000 || parser_subjidx > 1000) {

                *true_pos += 1.0;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!(
                        "\nSUBJ idx {}, OBJ idx {} (GOLD)",
                        gold_subjidx, gold_objidx
                    );
                    println!(
                        "SUBJ idx {}, OBJ idx {} (PARSER)",
                        parser_subjidx, parser_objidx
                    );
                }
            }
        }
    }
}


/// Get true positives, false positives and false negatives
/// for PP attachment in UD.
pub fn pp_ud_acc_comps(
    true_pos: &mut f32,
    false_pos: &mut f32,
    false_neg: &mut f32,
    gold_sent: &[Token],
    parser_sent: &[Token],
    print_sents: bool,
) {

    for i in 0..gold_sent.len() {

        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_pos = gold_token.pos().expect("No PoS");

        let mut gold_headidx = gold_token.head().expect("No head");

        let parser_token = &parser_sent[i];
        let parser_deprel = parser_token.head_rel().expect("No deprel");
        let parser_pos = parser_token.pos().expect("No PoS");

        let parser_headidx = parser_token.head().expect("No head idx");

        if parser_deprel == "case" && parser_pos.starts_with("ADP") {

            if parser_headidx > 0 {

                let parser_head_deprel = &parser_sent[parser_headidx - 1].head_rel().expect("No deprel").to_string();
                let parser_head_headidx= &parser_sent[parser_headidx - 1].head().expect("No head");

                if parser_head_deprel == "obl" || parser_head_deprel == "nmod" || parser_head_deprel == "root" {

                    if gold_headidx > 0 {
                        let gold_head_deprel = &gold_sent[gold_headidx - 1].head_rel().expect("No deprel").to_string();
                        let gold_head_headidx= &gold_sent[gold_headidx - 1].head().expect("No head");
                        if gold_headidx == parser_headidx && gold_deprel == parser_deprel &&
                            gold_head_headidx == parser_head_headidx && gold_head_deprel == parser_head_deprel {
                            *true_pos += 1.0;
                        } else {
                            *false_pos += 1.0;
                        }
                    } else {
                        *false_pos += 1.0;
                    }
                }
            }
        } else if gold_deprel == "case" && gold_pos.starts_with("ADP") {

            if gold_headidx > 0 {
                let gold_head_deprel = &gold_sent[gold_headidx - 1].head_rel().expect("No deprel").to_string();
                let gold_head_headidx = &gold_sent[gold_headidx - 1].head().expect("No head");

                if gold_head_deprel == "obl" || gold_head_deprel == "nmod" || gold_head_deprel == "root" {
                    *false_neg += 1.0;
                }
            }
        }
    }
}


/// Get true positives, false positives and false negatives
/// for subject-object inversion in UD.
pub fn inv_ud_acc_comps(
    true_pos: &mut f32,
    false_pos: &mut f32,
    false_neg: &mut f32,
    gold_sent: &[Token],
    parser_sent: &[Token],
    print_sents: bool,
) {
    let mut head_verb_args = HashMap::new();

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_head = gold_token.head().expect("No head");

        let token = &parser_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_deprel == "nsubj" || gold_deprel.starts_with("ob") || gold_deprel == "iobj" {
            //TODO: Replace by output of function verb::contentverb()
            let verb_idx = if (gold_head > 0)
                && gold_sent[gold_head - 1]
                .head_rel()
                .expect("No deprel")
                .eq("AUX")
                {
                    // Reattach auxiliary verb to content verb
                    gold_sent[gold_head - 1].head().expect("No head")
                } else {
                gold_head
            };

            if gold_deprel == "nsubj" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[0] = i;

                if token_deprel.starts_with("ob") || token_deprel == "iobj" {
                    entry[1] = i; // SUBJ mistaken for OBJ
                } else if gold_deprel == token_deprel {
                    entry[1] = 1001;
                }
            } else if gold_deprel.starts_with("ob") || gold_deprel == "iobj" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[2] = i;

                if token_deprel == "nsubj" {
                    entry[3] = i; // OBJ mistaken for SUBJ
                } else if gold_deprel == token_deprel {
                    entry[3] = 1001;
                }
            }
        }
    }

    for (_, val) in head_verb_args.iter() {
        let gold_subjidx = val[0];
        let parser_objidx = val[1]; // OBJ but should have been SUBJ
        let gold_objidx = val[2];
        let parser_subjidx = val[3]; // SUBJ but should have been OBJ

        if gold_subjidx > 0 && gold_objidx > 0 {

            // Subject-object inversion not recognized by parser
            if (gold_subjidx > gold_objidx) && (parser_objidx > 0 || parser_subjidx > 0 ) {

                *false_neg += 1.0;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!(
                        "\nSubj idx {}, obj idx {} (GOLD)",
                        gold_subjidx, gold_objidx
                    );
                    println!(
                        "Subj idx {}, obj idx {} (PARSER)",
                        parser_subjidx, parser_objidx
                    );
                }
                // No subject-object inversion, but labeled as such by parser
            } else if (gold_subjidx < gold_objidx) && (parser_objidx > 0 || parser_subjidx > 0) {

                *false_pos += 1.0;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!(
                        "\nSubj idx {}, obj idx {} (GOLD)",
                        gold_subjidx, gold_objidx
                    );
                    println!(
                        "Subj idx {}, obj idx {} (PARSER)",
                        parser_subjidx, parser_objidx
                    );
                }
                // Subject-object inversion recognized by parser
            } else if (gold_subjidx > gold_objidx) && (parser_objidx > 1000 || parser_subjidx > 1000) {

                *true_pos += 1.0;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!(
                        "\nSubj idx {}, obj idx {} (GOLD)",
                        gold_subjidx, gold_objidx
                    );
                    println!(
                        "Subj idx {}, obj idx {} (PARSER)",
                        parser_subjidx, parser_objidx
                    );
                }
            }
        }
    }
}