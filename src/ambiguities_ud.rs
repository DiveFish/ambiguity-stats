extern crate conllx;

use conllx::Token;
use std::collections::HashMap;

/// Ambiguity evaluation methods for treebanks
/// with Universal Dependencies labels.

/// Prepositional phrase errors.
///
/// In UD, the head of the prepositional phrase attaches
/// to the noun in the prepositional phrase, and the
/// following attachments are possible:
/// 1. obl -> case (UD) ~ OBJP/PP with verbal head (HDT)
/// 2. nmod -> case (UD) ~ OBJP/PP with nominal head (HDT)
/// 3. root -> case (UD) ~ PRED with copular verb (HDT)
///
/// Since two relations are involved, PP attachment is more
/// error-prone in UD than it is in HDT where only one relation
/// is involved.
pub fn pp_ambigs(
    overall_pps: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
    print_sents: bool,
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_pos = gold_token.pos().expect("No PoS");

        if gold_deprel == "case" && gold_pos.starts_with("ADP") {

            let gold_headidx = gold_token.head().expect("No head");
            let gold_head_deprel = &gold_sent[gold_headidx-1].head_rel().expect("No deprel");
            let gold_head_headidx= &gold_sent[gold_headidx-1].head().expect("No head");

            if gold_head_deprel == &"obl" || gold_head_deprel == &"nmod" || gold_head_deprel == &"root" {
                *overall_pps += 1;

                let parser_token = &parser_sent[i];
                let parser_deprel = parser_token.head_rel().expect("No deprel");
                let parser_headidx = parser_token.head().expect("No head idx");
                let parser_head_deprel = &parser_sent[parser_headidx].head_rel().expect("No deprel");
                let parser_head_headidx= &parser_sent[parser_headidx].head().expect("No head");

                if gold_headidx != parser_headidx || gold_deprel != parser_deprel ||
                    gold_head_headidx != parser_head_headidx || gold_head_deprel != parser_head_deprel {
                    *errors += 1;

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
                        if gold_head_headidx != parser_head_headidx {
                            println!(
                                "\n{} (GOLD), {} (PARSER) at idx {}",
                                gold_head_headidx, parser_head_headidx, i
                            );
                        }
                        if gold_head_deprel != parser_head_deprel {
                            println!(
                                "\n{} (GOLD), {} (PARSER) at idx {}",
                                gold_head_deprel, parser_head_deprel, i
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Prepositional phrase attachment with (in)correct heads, assuming
/// that the label of the phrase is identical to the gold parse.
/// Since the head of the prepositional phrases attaches to the noun
/// and not the preposition, this is the head that is checked here.
pub fn pp_head_same_label_ambigs(
    overall_pps: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
    print_sents: bool,
) {
    for i in 0..gold_sent.len() {

        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_headidx = gold_token.head().expect("No head");
        let gold_head_deprel = &gold_sent[gold_headidx-1].head_rel().expect("No deprel");

        let parser_token = &parser_sent[i];
        let parser_deprel = parser_token.head_rel().expect("No deprel");
        let parser_headidx = parser_token.head().expect("No head idx");
        let parser_head_deprel = &parser_sent[parser_headidx].head_rel().expect("No deprel");

        if gold_deprel == "case"
            && parser_deprel == "case"
            && ((gold_head_deprel == &"obj" && parser_head_deprel == &"obj")
                || (gold_head_deprel == &"iobj" && parser_head_deprel == &"iobj")
                || (gold_head_deprel == &"obl" && parser_head_deprel == &"obl"))
            {
                *overall_pps += 1;

                let gold_head_headidx= &gold_sent[gold_headidx-1].head().expect("No head");
                let parser_head_headidx= &parser_sent[parser_headidx].head().expect("No head");

                if gold_head_headidx != parser_head_headidx {
                    *errors += 1;

                    if print_sents && gold_sent.len() < 11 {
                        println!();
                        for token in gold_sent {
                            print!("{} ", token.form());
                        }
                        println!(
                            "\n{} idx (GOLD), {} idx (PARSER)",
                            gold_head_headidx, parser_head_headidx
                        );
                    }
                }
            }
    }
}


/// Prepositional phrase attachment with (in)correct labels, assuming that
/// the head of the phrase has been attached correctly. Otherwise,
/// labels cannot be expected to be identical to the gold parse.
//TODO: Add "case" relation as a condition for PPs
pub fn pp_label_same_head_ambigs(
    overall_pps: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
    print_sents: bool,
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_headidx = gold_token.head().expect("No head");
        let gold_head_headidx= &gold_sent[gold_headidx-1].head().expect("No head");

        let parser_token = &parser_sent[i];
        let parser_headidx = parser_token.head().expect("No head idx");
        let parser_head_headidx= &parser_sent[parser_headidx].head().expect("No head");

        if (gold_deprel == "obj" || gold_deprel == "iobj" || gold_deprel == "obl")
            && (gold_head_headidx == parser_head_headidx) {
            *overall_pps += 1;

            let parser_deprel = parser_token.head_rel().expect("No deprel");
            let parser_head_deprel = &parser_sent[parser_headidx].head_rel().expect("No deprel");
            let gold_head_deprel = &gold_sent[gold_headidx-1].head_rel().expect("No deprel");

            if gold_deprel != parser_deprel {
                *errors += 1;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!(
                        "\n{} (GOLD), {} (PARSER) at idx {}",
                        gold_deprel, parser_deprel, i
                    );
                }
            } else if gold_head_deprel != parser_head_deprel {
                *errors += 1;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!(
                        "\n{} (GOLD), {} (PARSER) at idx {}",
                        gold_head_deprel, parser_head_deprel, i
                    );
                }
            }
        }
    }
}


/// Subject-object confusions.
///
/// Note that, in UD, objects are split into two classes
/// - Core arguments: obj and iobj ~ OBJA and OBJD (HDT)
/// - Non-core arguments: obl ~ all other arguments (HDT)
pub fn subj_obj_ambigs(
    overall_subjobjs: &mut usize,
    errors: &mut usize,
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
            let verb_idx = gold_head;

            if gold_deprel == "nsubj" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[0] = i;

                if token_deprel.starts_with("ob") || token_deprel == "iobj" {
                    entry[1] = i; // nsubj mistaken for obj/iobj/obl
                }
            } else if gold_deprel.starts_with("ob") || gold_deprel == "iobj" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[2] = i;

                if token_deprel == "nsubj" {
                    entry[3] = i; // obj/iobj/obl mistaken for nsubj
                }
            }
        }
    }

    for (_, val) in head_verb_args.iter() {
        let gold_subjidx = val[0];
        let parser_objidx = val[1]; // obj/iobj/obl but should have been nsubj
        let gold_objidx = val[2];
        let parser_subjidx = val[3]; // nsubj but should have been obj/iobj/obl

        if gold_subjidx > 0 && gold_objidx > 0 {
            // Clause has a subject and an object
            *overall_subjobjs += 1;

            if parser_objidx > 0 || parser_subjidx > 0 {
                // nsubj and obj/iobj/obl confused
                *errors += 1;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!(
                        "\nnsubj idx {}, obj/iobj/obl idx {} (GOLD)",
                        gold_subjidx, gold_objidx
                    );
                    println!(
                        "nsubj idx {}, obj/iobj/obl idx {} (PARSER)",
                        parser_subjidx, parser_objidx
                    );
                }
            }
        }
    }
}

/// Subject-object inversions.
///
/// The difference between subject-object confusions and
/// inversions is that inversion considers only subject-object
/// combinations where the object precedes the subject.
pub fn inversion_ambigs(
    overall_subjobjs: &mut usize,
    errors: &mut usize,
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
            let verb_idx = gold_head;

            if gold_deprel == "nsubj" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[0] = i;

                if token_deprel.starts_with("ob") || token_deprel == "iobj" {
                    entry[1] = i; // nsubj mistaken for obj/iobj/obl
                }
            } else if gold_deprel.starts_with("ob") || gold_deprel == "iobj" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[2] = i;

                if token_deprel == "nsubj" {
                    entry[3] = i; // obj/iobj/obl mistaken for nsubj
                }
            }
        }
    }

    for (_, val) in head_verb_args.iter() {
        let gold_subjidx = val[0];
        let parser_objidx = val[1]; // obj/iobj/obl but should have been nsubj
        let gold_objidx = val[2];
        let parser_subjidx = val[3]; // nsubj but should have been obj/iobj/obl

        // Only difference to subj_obj_ambigs: && gold_subjidx > gold_objidx {
        if gold_subjidx > 0 && gold_objidx > 0 && gold_subjidx > gold_objidx {
            // Clause has a subject and an object
            *overall_subjobjs += 1;

            if parser_objidx > 0 || parser_subjidx > 0 {
                // nsubj and obj/iobj/obl confused
                *errors += 1;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!(
                        "\nnsubj idx {}, obj/iobj/obl idx {} (GOLD)",
                        gold_subjidx, gold_objidx
                    );
                    println!(
                        "nsubj idx {}, obj/iobj/obl idx {} (PARSER)",
                        parser_subjidx, parser_objidx
                    );
                }
            }
        }
    }
}

/// Get all prepositions and their head preference.
/// Content of `preps`: frequency, verb heads, noun heads, other heads
pub fn pp_preps_ud(
    preps: &mut HashMap<String, Vec<usize>>,
    gold_sent: &[Token],
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_pos = gold_token.pos().expect("No PoS");
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let mut gold_headidx = gold_token.head().expect("No head");
        if gold_headidx == 0 {
            continue;
        } else {
            gold_headidx = gold_headidx - 1;
        }
        let gold_head_deprel = gold_sent[gold_headidx].head_rel().expect("No deprel");
        let gold_head_headidx= gold_sent[gold_headidx].head().expect("No head");

        if gold_deprel == "case" &&
            gold_pos.starts_with("ADP") &&
            ( gold_head_deprel == "obl" || gold_head_deprel == "nmod" || gold_head_deprel == "root" ) {
            let value = preps
                .entry(gold_token.lemma().expect("No lemma").to_lowercase().to_string())
                .or_insert(vec![0; 4]);
            value[0] += 1;

            if gold_head_headidx > 0 {
                let gold_head_headpos = &gold_sent[gold_head_headidx-1].pos().expect("No PoS tag");

                if gold_head_headpos.starts_with("VERB") || gold_head_headpos.starts_with("AUX") {
                    value[1] += 1;
                } else if gold_head_headpos.starts_with("NOUN") || gold_head_headpos.starts_with("PROPN") || gold_head_headpos.starts_with("PRON") {
                    value[2] += 1;
                } else {
                    value[3] += 1;
                }
            }
        }
    }
}

/// Get all prepositions and the number of errors made with them
/// Content of `preps`: frequency, errors, verb heads, noun heads, other heads
pub fn pp_preps_errs_ud(
    preps: &mut HashMap<String, Vec<usize>>,
    gold_sent: &[Token],
    parser_sent: &[Token],
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let mut gold_headidx = gold_token.head().expect("No head");
        if gold_headidx == 0 {
            continue;
        } else {
            gold_headidx = gold_headidx - 1;
        }
        let gold_head_deprel = gold_sent[gold_headidx].head_rel().expect("No deprel");
        let gold_head_headidx= gold_sent[gold_headidx].head().expect("No head");

        let parser_token = &parser_sent[i];
        let parser_deprel = parser_token.head_rel().expect("No deprel");
        let mut parser_headidx = parser_token.head().expect("No head");
        if parser_headidx == 0 {
            continue;
        } else {
            parser_headidx = parser_headidx - 1;
        }
        let parser_head_deprel = parser_sent[parser_headidx].head_rel().expect("No deprel");
        let parser_head_headidx= parser_sent[parser_headidx].head().expect("No head");

        if gold_deprel == "case" && ( gold_head_deprel == "obl" || gold_head_deprel == "nmod" || gold_head_deprel == "root" ) {
            let value = preps
                .entry(gold_token.form().to_lowercase().to_string())
                .or_insert(vec![0; 5]);
            value[0] += 1;

            if gold_headidx != parser_headidx || gold_deprel != parser_deprel ||
                gold_head_headidx != parser_head_headidx || gold_head_deprel != parser_head_deprel {
                value[1] += 1;
            }

            if gold_head_headidx > 0 && parser_head_headidx > 0 {
                let gold_head_headpos = &gold_sent[gold_head_headidx-1].pos().expect("No PoS tag");
                let parser_head_headpos = &parser_sent[parser_head_headidx-1].pos().expect("No PoS tag");

                if gold_head_headpos.starts_with("VERB") || gold_head_headpos.starts_with("AUX") { // && (parser_head_headpos != gold_head_headpos) {
                    value[2] += 1;
                } else if gold_head_headpos.starts_with("NOUN") {//&& (parser_head_headpos != gold_head_headpos) {
                    value[3] += 1;
                } else { // if parser_head_headpos != gold_head_headpos {
                    value[4] += 1;
                }
            }
        }
    }
}

/// Get all prepositions and their head preference.
/// Content of `preps`: frequency, verb heads, noun heads, other heads
pub fn pp_objs_ud(
    prep_objs: &mut HashMap<String, usize>,
    gold_sent: &[Token],
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_pos = gold_token.pos().expect("No PoS");
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let mut gold_headidx = gold_token.head().expect("No head");
        if gold_headidx == 0 {
            continue;
        } else {
            gold_headidx = gold_headidx - 1;
        }
        let gold_head_poses: Vec<&str> = gold_sent[gold_headidx].pos().expect("No PoS").split("-").collect();
        let gold_head_pos = gold_head_poses[0];
        let gold_head_deprel = gold_sent[gold_headidx].head_rel().expect("No deprel");

        if gold_deprel == "case" &&
            gold_pos.starts_with("ADP") &&
            ( gold_head_deprel == "obl" || gold_head_deprel == "nmod" || gold_head_deprel == "root" ) {
            let value = prep_objs
                .entry(gold_head_pos.to_string())
                .or_insert(0);
            *value += 1;

        }
    }
}
