extern crate conllx;
use conllx::Token;

pub fn get_error_counts(gold_sent: &[Token], parser_sent: &[Token]) -> (usize, usize, usize, usize) {

    let mut correct_labels= 0;
    let mut correct_heads= 0;
    let mut label_errors= 0;
    let mut head_errors= 0;

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let mut gold_headidx = gold_token.head().expect("No head");
        if gold_headidx == 0 { //Ignore tokens with ROOT as head
            continue
        } else {
            gold_headidx -= 1;
        }
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let parser_token = &parser_sent[i];
        let mut parser_headidx = parser_token.head().expect("No head idx");
        if parser_headidx == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            parser_headidx -= 1;
        }
        let parser_deprel = parser_token.head_rel().expect("No deprel");

        if gold_deprel != parser_deprel {
            label_errors += 1;
        } else {
            correct_labels += 1;
        }

        if gold_headidx != parser_headidx {
            head_errors += 1;
        } else {
            correct_heads += 1;
        }
    }

    (correct_labels, correct_heads, label_errors, head_errors)
}

pub fn get_ambiguity_counts(gold_sent: &[Token], parser_sent: &[Token], fun: fn(&mut usize, &mut usize, &[Token], &[Token])) -> (usize, usize) {
    assert_eq!(gold_sent.len(), parser_sent.len());

    let mut overall_occurrences:usize = 0;
    let mut errors:usize = 0;
    fun(&mut overall_occurrences, &mut errors, gold_sent, parser_sent);
    (overall_occurrences, errors)
}

/// Count PP attachments and errors made in such cases.
pub fn n_pp_ambig(overall_pps: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let mut gold_headidx = gold_token.head().expect("No head");    //To avoid panic, use `match`
        if gold_headidx == 0 { //Ignore tokens with ROOT as head
            continue
        } else {
            gold_headidx -= 1;
        }
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let parser_token = &parser_sent[i];
        let mut parser_headidx = parser_token.head().expect("No head idx");
        if parser_headidx == 0 {  //Ignore tokens with ROOT as head
            continue
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
pub fn n_pp_objps_ambig(overall_pps: &mut usize, errors: &mut usize, gold_sent: &[Token], nongold_sent: &[Token]) {

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

//TODO: update counts in tex file; do we really want to count also subjects mistaken for objects?
/// Count fronted objects as in the example "Rosen warfen die Frauen."
/// and errors made in such cases.
pub fn n_subj_obj_ambig(overall_subjobjs: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    let mut gold_subjidx = 0;
    let mut gold_objidx = 0;
    let mut subjidx = 0;
    let mut objidx = 0;
    let mut is_passive = false;  // Exclude passive, relative clauses or reflexives?
    let mut is_relcl = false;

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let gold_lemma = match gold_token.lemma() { //@Daniël: There seem to be None lemmas in the data
            Some(_) => gold_token.lemma().unwrap(),
            None => ""
        };

        let token = &parser_sent[i];
        let token_deprel = token.head_rel().expect("No deprel");

        if gold_deprel.starts_with("OBJ") {
            gold_objidx = i;
            if gold_token.pos().expect("No PoS tag") == "PRELS" {   // Exclude object fronting in relative clauses
                is_relcl = true;
            }
            if token_deprel == "SUBJ" {
                subjidx = i;    // Fronted OBJ mistaken for SUBJ
            }
        } else if gold_deprel == "SUBJ" {
            gold_subjidx = i;
            if token_deprel.starts_with("OBJ") {
                objidx = i;
            }

        } else if gold_lemma == "werden%passiv" {   // Exclude passives - currently NOT used
            is_passive = true;
        }

        if gold_deprel == "-PUNCT-" || i == gold_sent.len()-1 { // For every clause
            if gold_subjidx > 0 && gold_objidx > 0  {   // The clause contains a subj AND an obj
                *overall_subjobjs += 1;

                if gold_objidx < gold_subjidx && !is_relcl && subjidx > 0 { // Fronted object
                    *errors += 1;
                    println!("Inversion error");
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!();
                    println!("{:?\n}", gold_subjidx);
                    println!("{:?\n}", gold_objidx);
                } else if objidx > 0 {
                    *errors += 1;
                    println!("Subject mistaken as object");
                    for token in gold_sent {
                        print!("{} ", token.form());
                    }
                    println!();
                    println!("{:?\n}", gold_subjidx);
                    println!("{:?\n}", gold_objidx);
                }

            }
            gold_subjidx = 0;
            gold_objidx = 0;
            subjidx = 0;
            objidx = 0;
            is_passive = false;
            is_relcl = false;
        }
    }
}

/// Count ambiguous verb particles as in the example "Was haben Teilnehmer von Lehrgängen, ..."
/// and errors made in such cases.
pub fn n_phrasalv_prep_ambig(overall_phrasalv_prep: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_deprel = gold_token.head_rel().expect("No deprel");
        let mut gold_head = gold_token.head().expect("No head");
        if gold_head == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            gold_head -= 1;
        }

        let parser_token = &parser_sent[i];
        let parser_deprel = parser_token.head_rel().expect("No deprel");
        let mut parser_head = parser_token.head().expect("No head");
        if parser_head == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            parser_head -= 1;
        }

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
    }
}

/// Count cases where it is difficult to separate subject and object,
/// as in the example "... weil IBM Oracle Geld gibt.", and errors made in such cases.
pub fn n_appo_phrase_ambig(overall_subj_objs_separations: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

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
        } else if gold_deprel == "OBJA" || gold_deprel == "OBJD"
            && parser_token.pos().expect("No PoS tag").starts_with("N") {
            gold_objidx = i;
            if parser_deprel == "SUBJ" {
                subjidx = i;
            }
        }

        if gold_subjidx > 0 && gold_objidx > 0 && (gold_objidx == (gold_subjidx+1) || gold_subjidx == (gold_objidx+1)) {
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
pub fn n_coord_ambig(overall_coords: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_pos = gold_token.pos().expect("No PoS tag");
        let mut gold_headidx = gold_token.head().expect("No head");
        if gold_headidx == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            gold_headidx -= 1;
        }

        let parser_token = &parser_sent[i];
        let parser_pos = parser_token.pos().expect("No PoS tag");
        let mut parser_headidx = parser_token.head().expect("No head");
        if parser_headidx == 0 {  //Ignore tokens with ROOT as head
            continue
        } else {
            parser_headidx -= 1;
        }

        if gold_pos == "KON" && gold_pos == parser_pos //{
            && !gold_sent[gold_headidx].pos().expect("No PoS tag").starts_with("V") { // Head of coordination is a verb
            *overall_coords += 1;
            if gold_headidx != parser_headidx {
                *errors += 1;
                for token in gold_sent {
                    print!("{} ", token.form());
                }
                print!("\n->\n");
                for token in gold_sent {
                    if token == gold_token {
                        print!("{}_{} (G: {}, P: {}) ", parser_token.form(), i, gold_headidx, parser_headidx);
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
pub fn n_adj_adv_ambig(overall_adjs: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

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
            if parser_pos != "ADJA" && parser_sent[i-1].pos().expect("No PoS tag") == "PWAV"{
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
pub fn n_pron_quant_ambig(overall_prons: &mut usize, errors: &mut usize, gold_sent: &[Token], parser_sent: &[Token]) {

    for i in 0..gold_sent.len() {

        if i+1 < gold_sent.len() {
            let gold_token = &gold_sent[i];
            let gold_deprel = gold_token.head_rel().expect("No deprel");
            let next_gold_token = &gold_sent[i+1];
            let next_gold_pos = next_gold_token.pos().expect("No PoS tag");
            let next_gold_deprel = next_gold_token.head_rel().expect("No deprel");

            let parser_token = &parser_sent[i];
            //TODO: Check if this variable should be used somewhere
            let parser_deprel = parser_token.head_rel().expect("No deprel");
            let parser_nexttoken = &parser_sent[i+1];
            let parser_nextpos = parser_nexttoken.pos().expect("No PoS tag");
            let parser_nextdeprel = parser_nexttoken.head_rel().expect("No deprel");

            if gold_deprel == "OBJD" && (next_gold_deprel == "OBJD" || next_gold_pos == "PIDAT") {
                *overall_prons += 1;
                for token in gold_sent {
                    print!("{} ", token.form());
                }
                println!();
                println!("{:?\n}", i);
                if (next_gold_deprel == "OBJD" && next_gold_deprel != parser_nextdeprel) ||
                    (next_gold_pos == "PIDAT" && next_gold_pos != parser_nextpos) {
                    *errors += 1;

                }
            }
        }
    }
}