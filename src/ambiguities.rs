extern crate conllx;
use conllx::Token;

pub fn get_ambiguity_counts(gold_sent: &[Token], parser_sent: &[Token], fun: fn(&mut usize, &mut usize, &[Token], &[Token])) -> (usize, usize) {
    assert_eq!(gold_sent.len(), parser_sent.len());

    let mut overall_occurrences:usize = 0;
    let mut errors:usize = 0;
    fun(&mut overall_occurrences, &mut errors, gold_sent, parser_sent);
    (overall_occurrences, errors)
}

//TODO: Finetuning needed to capture only ambiguous PPs, maybe only look at sentences with 2 PPs?
/// Count PP attachments and errors made in such cases.
pub fn n_pp_attachments(overall_pps: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let mut gold_headidx = gold_token.head().expect("No head");    //To avoid panic, use `match`
        if gold_headidx == 0 { //Ignore tokens with ROOT as head
            continue
        } else {
            gold_headidx -= 1;
        }
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let token = &parser_sent[i];
        let mut headidx = token.head().expect("No head idx");
        if headidx == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            headidx -= 1;
        }
        let deprel = token.head_rel().expect("No deprel");

        if (gold_deprel == "PP") && gold_deprel == deprel {
            *overall_pps += 1;
            for token in gold_sent {
                print!("{:?} ", token.form());
            }
            println!();
            println!("{:?\n}", i);
            if gold_headidx != headidx {
                *errors += 1;
            }
        }
    }
}

/// Count PPs and OBJPs and errors made in the assignment of the correct dependency label,
/// i.e. confusion between PP and OBJP.
pub fn n_pp_objps(overall_pps: &mut usize, errors: &mut usize, gold_sent: &[Token], nongold_sent: &[Token]) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let token = &nongold_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_deprel == "PP" || gold_deprel == "OBJP" {
            *overall_pps += 1;
            if gold_deprel == "PP" && token_deprel == "OBJP" {
                *errors += 1;
            } else if gold_deprel == "OBJP" && token_deprel == "PP" {
                *errors += 1;
                for token in gold_sent {
                    print!("{:?} ", token.form());
                }
                println!();
                println!("{:?\n}", i);
            }
        }
    }
}

/// Count fronted objects as in the example "Rosen warfen die Frauen."
/// and errors made in such cases.
pub fn n_obj_frontings(overall_frontedobjs: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    let mut gold_subjidx = 0;
    let mut gold_objidx = 0;
    let mut subjidx = 0;
    let mut is_passive = false;  // Exclude passive, relative clauses or reflexives?
    let mut is_relcl = false;

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_lemma = match gold_token.lemma() { //@Daniël: There seem to be None lemmas in the data
            Some(_) => gold_token.lemma().unwrap(),
            None => "",
        };

        let token = &parser_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_deprel == "OBJD" || gold_deprel == "OBJA" {
            gold_objidx = i;
            if gold_token.pos().expect("No PoS tag") == "PRELS" {   // Exclude object fronting in relative clauses
                is_relcl = true;
            }
            if token_deprel == "SUBJ" {
                subjidx = i;    // Fronted OBJ mistaken for SUBJ
            }
        } else if gold_deprel == "SUBJ" {
            gold_subjidx = i;

        } else if gold_lemma == "werden%passiv" {   // Exclude passives - currently not used
            is_passive = true;
        }

        if gold_deprel == "-PUNCT-" || i == gold_sent.len()-1 { // For every clause
            if gold_subjidx > 0 && gold_objidx > 0 && gold_objidx < gold_subjidx && !is_relcl { // Fronted object
                for token in gold_sent {
                    print!("{:?} ", token.form());
                }
                println!();
                println!("{:?\n}", gold_subjidx);
                println!("{:?\n}", gold_objidx);
                *overall_frontedobjs += 1;
                if subjidx > 0 {
                    *errors += 1;
                }
            }
            gold_subjidx = 0;
            gold_objidx = 0;
            subjidx = 0;
            is_passive = false;
            is_relcl = false;
        }
    }
}

//TODO: Also count cases where noun modifier is mistakenly labeled as a verbal argument?
/// Count ambiguous verb particles as in the example "Was haben Teilnehmer von Lehrgängen, ..."
/// and errors made in such cases.
pub fn n_verb_particles(overall_verb_particles: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let mut gold_head = gold_token.head().expect("No head");
        if gold_head == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            gold_head -= 1;
        }

        let token = &parser_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");
        let mut token_head = token.head().expect("No head");
        if token_head == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            token_head -= 1;
        }

        if (gold_deprel == "PP" || gold_deprel == "OBJP") && gold_deprel == token_deprel
            && gold_sent[gold_head].pos().expect("No deprel").starts_with("V") { // Head of PP is a verb
            *overall_verb_particles += 1;
            for token in gold_sent {
                print!("{:?} ", token.form());
            }
            println!();
            println!("{:?\n}", i);
            if parser_sent[token_head].pos().expect("No deprel").starts_with("N") {
                *errors += 1;
            }
        }
    }
}

/// Count cases where it is difficult to separate subject and object,
/// as in the example "... weil IBM Oracle Geld gibt.", and errors made in such cases.
pub fn n_subj_obj_splits(overall_subj_objs_separations: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    let mut gold_subjidx = 0;
    let mut gold_objidx = 0;
    let mut objidx = 0;

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let token = &parser_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_deprel == "SUBJ" && token.pos().expect("No PoS tag").starts_with("N") {
            gold_subjidx = i;
            if token_deprel.starts_with("OBJ") {
                objidx = i;
            }
        } else if gold_deprel == "OBJA" || gold_deprel == "OBJD"
            && token.pos().expect("No PoS tag").starts_with("N") {
            gold_objidx = i;
        }

        if gold_subjidx > 0 && gold_objidx > 0 && gold_objidx == (gold_subjidx+1) {
            *overall_subj_objs_separations += 1;
            if objidx > 0 && objidx != gold_objidx {
                *errors += 1;
                for token in gold_sent {
                    print!("{:?} ", token.form());
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

//TODO: Count only coordinations in sentences with verb-kon-verb combination?
/// Count coordinations and errors made in such cases.
pub fn n_coordinations(overall_coords: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_pos = gold_token.pos().expect("No PoS tag");
        let mut gold_headidx = gold_token.head().expect("No head");
        if gold_headidx == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            gold_headidx -= 1;
        }

        let token = &parser_sent[i];
        let token_pos = token.pos().expect("No PoS tag");
        let mut token_headidx = token.head().expect("No head");
        if token_headidx == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            token_headidx -= 1;
        }

        if gold_pos == "KON" && gold_pos == token_pos
            && gold_sent[gold_headidx].pos().expect("No PoS tag").starts_with("V") { // Head of coordination is a verb
            *overall_coords += 1;
            if gold_headidx != token_headidx {
                *errors += 1;
                for token in gold_sent {
                    print!("{:?} ", token.form());
                }
                println!();
                println!("{:?\n}", i);
            }
        }
    }
}

/// Count adjective ambiguities as in the example "How slow horses run."
/// and errors made by the parser in such cases.
pub fn n_adjectives(overall_adjs: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_pos = gold_token.pos().expect("No PoS tag");
        let mut gold_headidx = gold_token.head().expect("No head");
        if gold_headidx == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            gold_headidx -= 1;
        }
        let gold_head = &gold_sent[gold_headidx];
        let gold_headpos = gold_head.pos().expect("No PoS tag");

        let mut gold_npidx = 0;
        if gold_headidx + 1 > gold_sent.len()-1 {  //Ignore tokens with ROOT as head
            continue
        } else {
            gold_npidx = gold_headidx + 1;
        }
        let gold_headheadpos = &gold_sent[gold_npidx].pos().expect("No PoS tag");

        let token = &parser_sent[i];
        let token_pos = token.pos().expect("No PoS tag");

        if gold_pos == "PWAV" && gold_headpos == "ADJD" && gold_headheadpos.starts_with("N") { //|| (gold_pos == "ADJA" && gold_headpos.starts_with("N")) {
            *overall_adjs += 1;
            for token in gold_sent {
                print!("{:?} ", token.form());
            }
            println!();
            println!("{:?\n}", i);
            if token_pos != "PWAV" {
                *errors += 1;
            }
        }
    }
}

/// Count pronoun-quantifier ambiguities as in the example "I will tell them all my opinions."
/// and errors made in such cases.
pub fn n_pronoun_quants(overall_prons: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    for i in 0..gold_sent.len() {

        if i+1 < gold_sent.len() {
            let gold_token = &gold_sent[i];
            let gold_deprel = gold_token.head_rel().expect("No deprel");
            let next_gold_token = &gold_sent[i+1];
            let next_gold_pos = next_gold_token.pos().expect("No PoS tag");
            let next_gold_deprel = next_gold_token.head_rel().expect("No deprel");

            let token = &parser_sent[i];
            let token_deprel = token.head_rel().expect("No deprel");
            let next_token = &parser_sent[i+1];
            let next_token_pos = next_token.pos().expect("No PoS tag");
            let next_token_deprel = next_token.head_rel().expect("No deprel");

            if gold_deprel == "OBJD" && (next_gold_deprel == "OBJD" || next_gold_pos == "PIDAT") {
                *overall_prons += 1;
                for token in gold_sent {
                    print!("{:?} ", token.form());
                }
                println!();
                println!("{:?\n}", i);
                if (next_gold_deprel == "OBJD" && next_gold_deprel != next_token_deprel) ||
                    (next_gold_pos == "PIDAT" && next_gold_pos != next_token_pos) {
                    *errors += 1;

                }
            }
        }
    }
}