extern crate conllx;
use conllx::Token;
use std::collections::HashMap;

/// Ambiguity evaluation methods for treebanks
/// with Hamburg Dependecy Treebank labels.

pub fn get_ambiguity_counts(
    gold_sent: &[Token],
    parser_sent: &[Token],
    print: bool,
    fun: fn(&mut usize, &mut usize, &[Token], &[Token], bool),
) -> (usize, usize) {
    assert_eq!(gold_sent.len(), parser_sent.len());

    let mut overall_occurrences: usize = 0;
    let mut errors: usize = 0;
    fun(
        &mut overall_occurrences,
        &mut errors,
        gold_sent,
        parser_sent,
        print,
    );
    (overall_occurrences, errors)
}

/// Get all prepositions and the number of errors made with them
/// Content of `preps`: frequency, errors, verb heads, noun heads, other heads
pub fn pp_preps(
    preps: &mut HashMap<String, Vec<usize>>,
    gold_sent: &[Token],
    parser_sent: &[Token],
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let mut gold_headidx = gold_token.head().expect("No head"); //To avoid panic, use `match`
        if gold_headidx == 0 {
            //Ignore tokens with ROOT as head
            continue;
        } else {
            gold_headidx -= 1;
        }
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_pos =  gold_token.pos().expect("No deprel");

        let parser_token = &parser_sent[i];
        let mut parser_headidx = parser_token.head().expect("No head idx");
        if parser_headidx == 0 {
            //Ignore tokens with ROOT as head
            continue;
        } else {
            parser_headidx -= 1;
        }
        let parser_deprel = parser_token.head_rel().expect("No deprel");

        if gold_deprel == "PP" || gold_deprel == "OBJP" || (gold_deprel == "PRED" && gold_pos.starts_with("APP")) {
            let value = preps
                .entry(gold_token.form().to_lowercase().to_string())
                .or_insert(vec![0; 5]);
            value[0] += 1;

            if (gold_deprel != parser_deprel) || (gold_headidx != parser_headidx) {
                value[1] += 1;
            }

            let gold_head_pos = &gold_sent[gold_headidx].pos().expect("No PoS tag");
            let head_pos = &parser_sent[parser_headidx].pos().expect("No PoS tag");

            if gold_head_pos.starts_with("V") {//&& (head_pos != gold_head_pos) {
                value[2] += 1;
            } else if gold_head_pos.starts_with("N") {//&& (head_pos != gold_head_pos) {
                value[3] += 1;
            } else {//if head_pos != gold_head_pos {
                value[4] += 1;
            }
        }
    }
}
// <--- Parser evaluation

/// Calculate error rate from PPs in gold data which were incorrectly labeled.
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
        let gold_pos =  gold_token.pos().expect("No deprel");

        if gold_deprel == "OBJP" || gold_deprel == "PP" || (gold_deprel == "PRED" && gold_pos.starts_with("APP")) {
            *overall_pps += 1;

            let gold_headidx = gold_token.head().expect("No head");

            let parser_token = &parser_sent[i];
            let parser_headidx = parser_token.head().expect("No head idx");
            let parser_deprel = parser_token.head_rel().expect("No deprel");

            if gold_headidx != parser_headidx || gold_deprel != parser_deprel {
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
                }
            }
        }
    }
}

/// Calculate error rate from PPs in gold data which were incorrectly labeled
/// and from PPs in non-gold data which are not PPs in gold data.
pub fn pp_gng_ambigs(
    overall_pps: &mut usize,
    errors: &mut usize,
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
            *overall_pps += 1;

            if gold_headidx != parser_headidx || gold_deprel != parser_deprel {
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
                }
            }
        } else if parser_deprel == "OBJP" || parser_deprel == "PP" || (parser_deprel == "PRED" && parser_pos.starts_with("APP")) {
            *overall_pps += 1;
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
            }
        }
    }
}

pub fn pp_head_ambigs(
    overall_pps: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
    print_sents: bool,
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        if gold_deprel == "OBJP" || gold_deprel == "PP" {
            *overall_pps += 1;

            let gold_headidx = gold_token.head().expect("No head");

            let parser_token = &parser_sent[i];
            let parser_headidx = parser_token.head().expect("No head idx");

            if gold_headidx != parser_headidx {
                *errors += 1;

                if print_sents && gold_sent.len() < 11 {
                    println!();
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!(
                        "\n{} idx (GOLD), {} idx (PARSER)",
                        gold_headidx, parser_headidx
                    );
                }
            }
        }
    }
}

/// Adds to `pp_head_ambigs` the condition that the label must be PP or OBJP.
/// Otherwise, attaching a different head than in the gold standard may be
/// reasonable if the label is also different from the gold standard.
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

        let parser_token = &parser_sent[i];
        let parser_deprel = parser_token.head_rel().expect("No deprel");

        if (gold_deprel == "OBJP" && parser_deprel == "OBJP")
            || (gold_deprel == "PP" && parser_deprel == "PP")
            {
                *overall_pps += 1;

                let gold_headidx = gold_token.head().expect("No head");

                let parser_headidx = parser_token.head().expect("No head idx");

                if gold_headidx != parser_headidx {
                    *errors += 1;

                    if print_sents && gold_sent.len() < 11 {
                        println!();
                        for token in gold_sent {
                            print!("{} ", token.form());
                        }
                        println!(
                            "\n{} idx (GOLD), {} idx (PARSER)",
                            gold_headidx, parser_headidx
                        );
                    }
                }
            }
    }
}

pub fn pp_label_ambigs(
    overall_pps: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
    print_sents: bool,
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        if gold_deprel == "OBJP" || gold_deprel == "PP" {
            *overall_pps += 1;

            let parser_token = &parser_sent[i];
            let parser_deprel = parser_token.head_rel().expect("No deprel");

            if (gold_deprel == "PP") && (parser_deprel == "OBJP") {
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
            } else if (gold_deprel == "OBJP") && (parser_deprel == "PP") {
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
            }
        }
    }
}

/// Adds to `pp_label_ambigs` the condition that the gold and parser heads must be
/// the same. Otherwise, attaching with different label than in the gold standard may
/// be reasonable if the head is also different from the gold standard.
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

        let parser_token = &parser_sent[i];
        let parser_headidx = parser_token.head().expect("No head idx");

        if (gold_deprel == "OBJP" || gold_deprel == "PP") && (gold_headidx == parser_headidx) {
            *overall_pps += 1;

            let parser_deprel = parser_token.head_rel().expect("No deprel");

            if (gold_deprel == "PP") && (parser_deprel == "OBJP") {
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
            } else if (gold_deprel == "OBJP") && (parser_deprel == "PP") {
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
            }
        }
    }
}

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
                    entry[1] = i; // SUBJ mistaken for OBJ
                }
            } else if gold_deprel.starts_with("OBJ") {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[2] = i;

                if token_deprel == "SUBJ" {
                    entry[3] = i; // OBJ mistaken for SUBJ
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
            // Clause has a subject and an object
            *overall_subjobjs += 1;

            if parser_objidx > 0 || parser_subjidx > 0 {
                // SUBJ and OBJ confused
                *errors += 1;

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
                    entry[1] = i; // SUBJ mistaken for OBJ
                }
            } else if gold_deprel.starts_with("OBJ") {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[2] = i;

                if token_deprel == "SUBJ" {
                    entry[3] = i; // OBJ mistaken for SUBJ
                }
            }
        }
    }

    for (_, val) in head_verb_args.iter() {
        let gold_subjidx = val[0];
        let parser_objidx = val[1]; // OBJ but should have been SUBJ
        let gold_objidx = val[2];
        let parser_subjidx = val[3]; // SUBJ but should have been OBJ

        // Only difference to subj_obj_ambigs: && gold_subjidx > gold_objidx {
        if gold_subjidx > 0 && gold_objidx > 0 && gold_subjidx > gold_objidx {
            // Clause has a subject and an object
            *overall_subjobjs += 1;

            if parser_objidx > 0 || parser_subjidx > 0 {
                // SUBJ and OBJ confused
                *errors += 1;

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

/// Find sentences where subject-object confusion occurs and count those cases where
/// the subject and/or object form is ambiguous between nominative and accusative case.
pub fn subj_obj_highly_ambigs(
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
                    entry[1] = i; // SUBJ mistaken for OBJ
                }
            } else if gold_deprel.starts_with("OBJ") {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                if token.form() == token.lemma().expect("No lemma") {
                    // Ambiguous object (?)
                    entry[2] = i;

                    if token_deprel == "SUBJ" {
                        entry[3] = i; // OBJ mistaken for SUBJ
                    }
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
            // Clause has a subject and an object
            *overall_subjobjs += 1;

            if parser_objidx > 0 && parser_subjidx > 0 {
                // SUBJ and OBJ confused
                *errors += 1;

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

/// Count long-distance errors
pub fn distance_errs(
    overall_dist: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
    print_sents: bool,
) {
    let min_distance = 7;

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_headidx = gold_token.head().expect("No head");

        let distance = if i < gold_headidx && (gold_headidx - i) > min_distance && gold_headidx > i
            {
                gold_headidx - i
            } else if i < gold_headidx && (i - gold_headidx) > min_distance && i > gold_headidx {
            i - gold_headidx
        } else {
            0
        };

        if distance != 0 {
            *overall_dist += 1;

            let parser_token = &parser_sent[i];
            let parser_headidx = parser_token.head().expect("No head idx");
            let parser_deprel = parser_token.head_rel().expect("No deprel");

            if gold_headidx != parser_headidx || gold_deprel != parser_deprel {
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
                }
            }
        }
    }
}

pub fn ovs_count(ovs: &mut usize, gold_sent: &[Token]) {
    let mut head_verb_args = HashMap::new();

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_head = gold_token.head().expect("No head");

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
            } else if gold_deprel.starts_with("OBJ") {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[1] = i;
            }
        }
    }

    for (verb_idx, verb_args) in head_verb_args.iter() {
        let subj_idx = verb_args[0];
        let obj_idx = verb_args[1];

        // Clause has a subject and an object, subject follows and object precedes verb
        if subj_idx > 0 && obj_idx > 0 && subj_idx > *verb_idx && obj_idx < *verb_idx {
            *ovs += 1;
        }
    }
}
// ---> Parser evaluation

/// Count PP attachments and errors.
pub fn n_pp_ambig(
    overall_pps: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let mut gold_headidx = gold_token.head().expect("No head"); //To avoid panic, use `match`
        if gold_headidx == 0 {
            //Ignore tokens with ROOT as head
            continue;
        } else {
            gold_headidx -= 1;
        }
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let parser_token = &parser_sent[i];
        let mut parser_headidx = parser_token.head().expect("No head idx");
        if parser_headidx == 0 {
            //Ignore tokens with ROOT as head
            continue;
        } else {
            parser_headidx -= 1;
        }
        let parser_deprel = parser_token.head_rel().expect("No deprel");

        if (gold_deprel == "PP" || gold_deprel == "OBJP") && gold_deprel == parser_deprel {
            *overall_pps += 1;
            if gold_headidx != parser_headidx {
                *errors += 1;
                for token in gold_sent {
                    print!("{} ", token.form());
                }
                println!();
                println!("{:?\n}", i);
            }
        }
    }
}

/// Count PPs and OBJPs and errors made in the assignment of the correct dependency label,
/// i.e. confusion between PP and OBJP.
pub fn n_pp_objps_ambig(
    overall_pps: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    nongold_sent: &[Token],
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let parser_token = &nongold_sent[i];
        let parser_deprel = parser_token.head_rel().expect("No deprel");

        if gold_deprel == "PP" || gold_deprel == "OBJP" {
            *overall_pps += 1;
            if gold_deprel == "PP" && parser_deprel == "OBJP" {
                *errors += 1;
            } else if gold_deprel == "OBJP" && parser_deprel == "PP" {
                *errors += 1;
                for token in gold_sent {
                    print!("{} ", token.form());
                }
                println!();
                println!("{:?\n}", i);
            }
        }
    }
}

/// Count fronted objects as in the example "Rosen warfen die Frauen."
/// and errors made in such cases. Also include general confusions between
/// subject and object.
/// For counting only object fronting errors, include ``&& gold_subjidx > gold_objidx``
/// in the line ``if gold_subjidx > 0 && gold_objidx > 0 {  // Clause has a subject and an object``
pub fn n_subj_obj_ambig(
    overall_subjobjs: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
) {
    let mut head_verb_args = HashMap::new();

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_head = gold_token.head().expect("No head");

        let token = &parser_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");
        //let mut token_head = token.head().expect("No head");

        if gold_deprel == "SUBJ" || gold_deprel.starts_with("OBJ") {
            let mut verb_idx;
            if gold_sent[gold_head - 1]
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
                    entry[1] = i; // SUBJ mistaken for OBJ: Save
                }
            } else if gold_deprel.starts_with("OBJ") {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[2] = i;

                if token_deprel == "SUBJ" {
                    entry[3] = i; // OBJA mistaken for SUBJ
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
            // Clause has a subject and an object
            *overall_subjobjs += 1;

            if parser_objidx > 0 || parser_subjidx > 0 {
                *errors += 1;

                if gold_sent.len() < 30 {
                    if gold_subjidx > gold_objidx {
                        println!("\nSubject-object INVERSION:");
                    } else {
                        println!("\nSubject-object CONFUSION:");
                    }
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!();
                    println!(">  Gold subj/obj {:?} {:?}", gold_subjidx, gold_objidx);
                    if parser_objidx > 0 && parser_subjidx > 0 {
                        println!(
                            ">  Parser subj/obj {:?} {:?}",
                            parser_subjidx, parser_objidx
                        );
                    } else if parser_objidx > 0 {
                        println!(">  Parser obj {:?}", parser_objidx);
                    } else if parser_subjidx > 0 {
                        println!(">  Parser subj {:?}", parser_subjidx);
                    }
                }
            }
        }
    }
}

/// Count ambiguous verb particles as in the example "Was haben Teilnehmer von Lehrgängen, ..."
/// and errors made in such cases.
pub fn n_phrasalv_prep_ambig(
    overall_phrasalv_prep: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        //let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_pos = gold_token.pos().expect("No PoS");
        let mut gold_head = gold_token.head().expect("No head");
        if gold_head == 0 {
            //Ignore tokens with ROOT as head
            continue;
        } else {
            gold_head -= 1;
        }

        let parser_token = &parser_sent[i];
        //let parser_deprel = parser_token.head_rel().expect("No deprel");
        //let parser_pos = parser_token.pos().expect("No PoS");
        let mut parser_head = parser_token.head().expect("No head");
        if parser_head == 0 {
            //Ignore tokens with ROOT as head
            continue;
        } else {
            parser_head -= 1;
        }

        if gold_pos.eq("PTKVZ") {
            // Preposition is a verbal particle
            *overall_phrasalv_prep += 1;
            if gold_head != parser_head
                && gold_sent[gold_head]
                .pos()
                .expect("No deprel")
                .starts_with("V")
                {
                    *errors += 1;
                    if gold_sent.len() < 30 {
                        println!();
                        for j in 0..gold_sent.len() {
                            if j == i {
                                print!("__{}__ ", &gold_sent[j].form());
                            } else if j == gold_head {

                            } else {
                                print!("{} ", &gold_sent[j].form());
                            }
                        }
                        println!();
                        println!(
                            "{}; gold head {}, parser head {}",
                            i, gold_head, parser_head
                        );
                    }
                }
        }

        /*  //Initial filtering of phrasal verbs: Verb-Prep and Noun-Prep combinations considered to be potentially ambiguous
        if (gold_deprel == "PP" || gold_deprel == "OBJP") && gold_deprel == parser_deprel {
            if gold_sent[gold_head].pos().expect("No deprel").starts_with("V") { // Head of PP is a verb
                *overall_phrasalv_prep += 1;
                if parser_sent[parser_head].pos().expect("No deprel").starts_with("N") {
                    *errors += 1;
                    println!("Should be phrasal verb, was NP-attached PP");
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!();
                    println!("{:?\n}", i);
                }
            } else if gold_sent[gold_head].pos().expect("No deprel").starts_with("N") {
                *overall_phrasalv_prep += 1;
                if parser_sent[parser_head].pos().expect("No deprel").starts_with("V") {
                    *errors += 1;
                    println!("Should be NP-attached PP, was phrasal verb");
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!();
                    println!("{:?\n}", i);
                }
            }
        }
        */
    }
}

/// Count cases where it is difficult to separate subject and object,
/// as in the example "... weil IBM Oracle Geld gibt.", and errors made in such cases.
pub fn n_appo_phrase_ambig(
    overall_subj_objs_separations: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
) {
    let mut gold_subjidx = 0;
    let mut gold_objidx = 0;
    let mut subjidx = 0;
    let mut objidx = 0;

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let parser_token = &parser_sent[i];
        let parser_deprel = parser_token.head_rel().expect("No deprel");

        if gold_deprel == "SUBJ" && parser_token.pos().expect("No PoS tag").starts_with("N") {
            gold_subjidx = i;
            if parser_deprel.starts_with("OBJ") {
                objidx = i;
            }
        } else if gold_deprel == "OBJA"
            || gold_deprel == "OBJD" && parser_token.pos().expect("No PoS tag").starts_with("N")
            {
                gold_objidx = i;
                if parser_deprel == "SUBJ" {
                    subjidx = i;
                }
            }

        if gold_subjidx > 0
            && gold_objidx > 0
            && (gold_objidx == (gold_subjidx + 1) || gold_subjidx == (gold_objidx + 1))
            {
                *overall_subj_objs_separations += 1;
                if (objidx > 0 && objidx != gold_objidx) || (subjidx > 0 && subjidx != gold_subjidx) {
                    *errors += 1;
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!();
                    println!("{:?\n}", gold_subjidx);
                    println!("{:?\n}", gold_objidx);
                }
                gold_subjidx = 0;
                gold_objidx = 0;
                objidx = 0;
            }
    }
}

//TODO: Count only verb-kon-verb combinations?
/// Count coordinations and errors made in such cases.
pub fn n_coord_ambig(
    overall_coords: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_pos = gold_token.pos().expect("No PoS tag");
        let mut gold_headidx = gold_token.head().expect("No head");
        if gold_headidx == 0 {
            //Ignore tokens with ROOT as head
            continue;
        } else {
            gold_headidx -= 1;
        }

        let parser_token = &parser_sent[i];
        let parser_pos = parser_token.pos().expect("No PoS tag");
        let mut parser_headidx = parser_token.head().expect("No head");
        if parser_headidx == 0 {
            //Ignore tokens with ROOT as head
            continue;
        } else {
            parser_headidx -= 1;
        }

        if gold_pos == "KON" && gold_pos == parser_pos //{
            && !gold_sent[gold_headidx].pos().expect("No PoS tag").starts_with("V")
            {
                // Head of coordination is a verb
                *overall_coords += 1;
                if gold_headidx != parser_headidx {
                    *errors += 1;
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    print!("\n->\n");
                    for token in gold_sent {
                        if token == gold_token {
                            print!(
                                "{}_{} (G: {}, P: {}) ",
                                parser_token.form(),
                                i,
                                gold_headidx,
                                parser_headidx
                            );
                        } else {
                            print!("{}_{} ", token.form(), token.head().expect("No head"));
                        }
                    }
                    //println!("{}", parser_sent[parser_headidx].pos().expect("No PoS tag"));
                    println!();
                    println!("Coord idx: {:?}\n", i);
                }
            }
    }
}

/// Count adjective ambiguities as in the example "How slow horses run."
/// and errors made by the parser in such cases.
pub fn n_adj_adv_ambig(
    overall_adjs: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
) {
    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_pos = gold_token.pos().expect("No PoS tag");
        let mut gold_headidx = gold_token.head().expect("No head");
        if gold_headidx == 0 {
            //Ignore tokens with ROOT as head
            continue;
        } else {
            gold_headidx -= 1;
        }
        let gold_head = &gold_sent[gold_headidx];
        let gold_headpos = gold_head.pos().expect("No PoS tag");

        let parser_token = &parser_sent[i];
        let parser_pos = parser_token.pos().expect("No PoS tag");

        if gold_pos == "PWAV" && gold_headpos == "ADJD" && gold_headpos.starts_with("N") {
            *overall_adjs += 1;
            if parser_pos != "PWAV" {
                *errors += 1;
                for token in gold_sent {
                    print!("{} ", token.form());
                }
                println!();
                println!("{:?\n}", i);
            }
        } else if gold_headpos == "ADJA" && gold_headpos.starts_with("N") && i > 0 {
            *overall_adjs += 1;
            if parser_pos != "ADJA" && parser_sent[i - 1].pos().expect("No PoS tag") == "PWAV" {
                *errors += 1;
                for token in gold_sent {
                    print!("{} ", token.form());
                }
                println!();
                println!("{:?\n}", i);
            }
        }
    }
}

/// Count pronoun-quantifier ambiguities as in the example "I will tell them all my opinions."
/// and errors made in such cases.
pub fn n_pron_quant_ambig(
    overall_prons: &mut usize,
    errors: &mut usize,
    gold_sent: &[Token],
    parser_sent: &[Token],
) {
    for i in 0..gold_sent.len() {
        if i + 1 < gold_sent.len() {
            let gold_token = &gold_sent[i];
            let gold_deprel = gold_token.head_rel().expect("No deprel");
            let next_gold_token = &gold_sent[i + 1];
            let next_gold_pos = next_gold_token.pos().expect("No PoS tag");
            let next_gold_deprel = next_gold_token.head_rel().expect("No deprel");

            //TODO: Check if these variables should be used somewhere
            //let parser_token = &parser_sent[i];
            //let parser_deprel = parser_token.head_rel().expect("No deprel");
            let parser_nexttoken = &parser_sent[i + 1];
            let parser_nextpos = parser_nexttoken.pos().expect("No PoS tag");
            let parser_nextdeprel = parser_nexttoken.head_rel().expect("No deprel");

            if gold_deprel == "OBJD" && (next_gold_deprel == "OBJD" || next_gold_pos == "PIDAT") {
                *overall_prons += 1;
                for token in gold_sent {
                    print!("{} ", token.form());
                }
                println!();
                println!("{:?\n}", i);
                if (next_gold_deprel == "OBJD" && next_gold_deprel != parser_nextdeprel)
                    || (next_gold_pos == "PIDAT" && next_gold_pos != parser_nextpos)
                    {
                        *errors += 1;
                    }
            }
        }
    }
}
