extern crate conllx;
use conllx::Token;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::{HashMap, HashSet};

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

/// Parser evaluation

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
                    /*
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
                    */
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
                /*
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
                */
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
                // Reattach content verb to auxiliary verb
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

/// In order to get inversions from gold data which were incorrectly parsed but also
/// inversions in parser data which are not inversions in the gold data, run inversion_ambigs
/// first on gold-parsed, then on parsed-gold as gold_sent-parser_sent.
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
                    /*
                    println!(
                        "\nSUBJ idx {}, OBJ idx {} (GOLD)",
                        gold_subjidx, gold_objidx
                    );
                    println!(
                        "SUBJ idx {}, OBJ idx {} (PARSER)",
                        parser_subjidx, parser_objidx
                    );
                    */
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
                // Reattach content verb to auxiliary verb
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

/// Note that `ovs_count' acknowledges the fact that there may be more than one
/// object in a sentence and chooses the direct object that is the closest to the main verb.
pub fn ovs_count(ovs: &mut usize, gold_sent: &[Token]) {
    let mut head_verb_args = HashMap::new();

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_head = gold_token.head().expect("No head");

        if gold_deprel == "SUBJ" || gold_deprel == "OBJA" {  //starts_with("OBJ") to match any object
            let mut verb_idx;
            if (gold_head > 0)
                && gold_sent[gold_head - 1]
                    .head_rel()
                    .expect("No deprel")
                    .eq("AUX")
            {
                // Reattach content verb to auxiliary verb
                verb_idx = gold_sent[gold_head - 1].head().expect("No head");
            } else {
                verb_idx = gold_head;
            }

            if gold_deprel == "SUBJ" {
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                entry[0] = i;
            } else if gold_deprel == "OBJA" {  //starts_with("OBJ") to match any object
                let entry = head_verb_args.entry(verb_idx).or_insert(vec![0; 4]);
                // If there is more than one object, pick the one closer to the verb
                let verb_distance_i = if verb_idx > i {
                    verb_idx - i
                } else { i - verb_idx };
                let verb_distance_entry = if verb_idx > entry[1] {
                    verb_idx - entry[1]
                } else { entry[i] - verb_idx };

                if !entry[1] > 0 || verb_distance_entry > verb_distance_i {
                    entry[1] = i;
                }
            }
        }
    }

    for (verb_idx, verb_args) in head_verb_args.iter() {
        let subj_idx = verb_args[0];
        let obj_idx = verb_args[1];

        // Clause has a subject and an object, subject follows and object precedes verb
        //if subj_idx > 0 && obj_idx > 0 && subj_idx > *verb_idx && obj_idx < *verb_idx {
        if subj_idx > 0 && obj_idx > 0 && obj_idx < subj_idx {
            *ovs += 1;
        }
    }
}

/// Collect error sentences from file.
pub fn read_err_sents(err_file: &str) -> io::Result<(HashSet<String>)> {

    let mut err_sents = HashSet::new();
    let file = File::open(err_file.to_string())?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        err_sents.insert(line?);
    }

    Ok(err_sents)
}

pub fn matching_sents(sents_a: HashSet<String>, sents_b: HashSet<String>) -> HashSet<String> {
    let mut matching_sents = HashSet::new();
    for sent in sents_a.iter() {
        if sents_b.contains(sent) {
            matching_sents.insert(sent.clone());
        }
    }
    matching_sents
}