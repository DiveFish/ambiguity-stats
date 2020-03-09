extern crate conllx;

use conllx::Token;
use conllx::Features;

pub fn precision(true_pos: f32, false_pos: f32) -> f32 {
    true_pos / (true_pos + false_pos)
}

pub fn recall(true_pos: f32, false_neg: f32) -> f32 {
    true_pos / (true_pos + false_neg)
}

pub fn f1_score(precision: f32, recall: f32) -> f32 {
    2.0 * ( (precision * recall) / (precision + recall))
}

pub fn las(output: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>) -> f32 {
    let mut head_label_errors = 0.0;
    let mut n_attachments = 0.0;

    for (output_sent, gold_sent) in output.iter().zip(gold.iter()) {
        for (output_token, gold_token) in output_sent.iter().zip(gold_sent.iter()) {
            n_attachments += 1.0;
            if output_token.head() != gold_token.head()
                || output_token.head_rel() != gold_token.head_rel()
            {
                head_label_errors += 1.0;
            }
        }
    }

    1.0 - (head_label_errors / n_attachments)
}

pub fn uas(output: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>) -> f32 {
    let mut head_errors = 0.0;
    let mut n_heads = 0.0;

    for (output_sent, gold_sent) in output.iter().zip(gold.iter()) {
        for (output_token, gold_token) in output_sent.iter().zip(gold_sent.iter()) {
            n_heads += 1.0;
            if output_token.head() != gold_token.head() {
                head_errors += 1.0;
            }
        }
    }

    1.0 - (head_errors / n_heads)
}

pub fn las_uas(output: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>) -> (f32, f32) {
    let mut head_errors = 0.0;
    let mut head_label_errors = 0.0;
    let mut n_attachments = 0.0;

    for (output_sent, gold_sent) in output.iter().zip(gold.iter()) {
        for (output_token, gold_token) in output_sent.iter().zip(gold_sent.iter()) {
            n_attachments += 1.0;
            if output_token.head() != gold_token.head() {
                head_errors += 1.0;
                head_label_errors += 1.0;
            } else if output_token.head_rel() != gold_token.head_rel() {
                head_label_errors += 1.0;
            }
        }
    }

    (
        1.0 - (head_label_errors / n_attachments),
        1.0 - (head_errors / n_attachments),
    )
}

pub fn las_uas_no_punct(output: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>) -> (f32, f32) {
    let mut head_errors = 0.0;
    let mut head_label_errors = 0.0;
    let mut n_attachments = 0.0;

    for (output_sent, gold_sent) in output.iter().zip(gold.iter()) {
        for (output_token, gold_token) in output_sent.iter().zip(gold_sent.iter()) {
            if gold_token.head_rel().unwrap() != "-PUNCT-" {
                n_attachments += 1.0;
                if output_token.head() != gold_token.head() {
                    head_errors += 1.0;
                    head_label_errors += 1.0;
                } else if output_token.head_rel() != gold_token.head_rel() {
                    head_label_errors += 1.0;
                }
            }
        }
    }

    (
        1.0 - (head_label_errors / n_attachments),
        1.0 - (head_errors / n_attachments),
    )
}

pub fn las_no_heads_feats(parsed: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>, parser_model: &str) -> f32 {
    let mut n_attachments = 0.0;
    let mut label_errors = 0.0;
    for (parsed_sent, gold_sent) in parsed.iter().zip(gold.iter()) {

        print!("{}\t", parser_model);

        let mut s_fit = "-1";
        let mut o_fit = "-1";
        let mut subj_gold = "UNK";
        let mut obj_gold = "UNK";
        let mut subj_parsed = "UNK";
        let mut obj_parsed = "UNK";
        let mut order = "UNK";
        let mut prop1 = "UNK";
        let mut prop2 = "UNK";

        let mut first = true;
        for (parsed_token, gold_token) in parsed_sent.iter().zip(gold_sent.iter()) {

            if first {
                let mut features = gold_token.features().map(Features::as_map).expect("No mapping");
                order = &features.get("order").expect("No order").as_ref().expect("No more order");
                let props =  &features.get("props").expect("No props").as_ref().expect("No more props").split("-").collect::<Vec<_>>();
                prop1 = props[0];
                prop2 = props[1];
                first = false;
            }

            print!("{} ", parsed_token.form());

            if let Some(gold_token_rel) = gold_token.head_rel() {
                let gold_token_rel = gold_token.head_rel().expect("No head rel");
                let parsed_token_rel = parsed_token.head_rel().expect("No head rel");

                if gold_token_rel == "nsubj" || gold_token_rel == "obj" {

                    n_attachments += 1.0;
                    if parsed_token.head_rel() != gold_token.head_rel() {
                        label_errors += 1.0;
                    }

                    if gold_token_rel == "nsubj" {
                        subj_gold = gold_token_rel;
                        subj_parsed = parsed_token_rel;
                    } else if gold_token_rel == "obj" {
                        obj_gold = gold_token_rel;
                        obj_parsed = parsed_token_rel;
                    }

                    if gold_token_rel == "nsubj" && parsed_token.head_rel() == gold_token.head_rel() {
                        s_fit = "1";
                    } else if gold_token_rel == "obj" && parsed_token.head_rel() == gold_token.head_rel() {
                        o_fit = "1";
                    } else if gold_token_rel == "nsubj" && parsed_token_rel == "obj" {
                        s_fit = "0";
                    } else if gold_token_rel == "obj" && parsed_token_rel == "nsubj" {
                        o_fit = "0";
                    } else if gold_token_rel == "nsubj" {
                        s_fit = parsed_token_rel;;
                    } else if gold_token_rel == "obj" {
                        o_fit = parsed_token_rel;
                    }
                }
            }
        }
        println!("\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", s_fit, o_fit, subj_gold, obj_gold, subj_parsed, obj_parsed, order, prop1, prop2);
    }
    1.0 - (label_errors / n_attachments)
}

pub fn per_sent_las(output: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>) -> Vec<f32> {
    let mut sent_las = Vec::with_capacity(gold.len());

    for (output_sent, gold_sent) in output.iter().zip(gold.iter()) {
        let mut head_label_errors = 0.0;
        let mut n_attachments = 0.0;

        for (output_token, gold_token) in output_sent.iter().zip(gold_sent.iter()) {
            n_attachments += 1.0;
            if output_token.head() != gold_token.head()
                || output_token.head_rel() != gold_token.head_rel()
            {
                head_label_errors += 1.0;
            }
        }
        sent_las.push(1.0 - (head_label_errors / n_attachments));
    }

    sent_las
}

pub fn per_sent_uas(output: &Vec<Vec<Token>>, gold: &Vec<Vec<Token>>) -> Vec<f32> {
    let mut sent_uas = Vec::with_capacity(gold.len());

    for (output_sent, gold_sent) in output.iter().zip(gold.iter()) {
        let mut head_errors = 0.0;
        let mut n_heads = 0.0;

        for (output_token, gold_token) in output_sent.iter().zip(gold_sent.iter()) {
            n_heads += 1.0;
            if output_token.head() != gold_token.head() {
                head_errors += 1.0;
            }
        }
        sent_uas.push(1.0 - (head_errors / n_heads));
    }

    sent_uas
}

/// Get labeled attachment score (LAS) components.
pub fn get_las_parts(gold_sent: &[Token], parser_sent: &[Token]) -> (usize, usize, usize, usize) {
    let mut attachments = 0;
    let mut combined_errors = 0;
    let mut head_errors = 0;
    let mut label_errors = 0;

    for i in 0..gold_sent.len() {
        let gold_token = &gold_sent[i];
        let gold_headidx = gold_token.head().expect("No head");
        let gold_deprel = gold_token.head_rel().expect("No deprel");

        let parser_token = &parser_sent[i];
        let parser_headidx = parser_token.head().expect("No head idx");
        let parser_deprel = parser_token.head_rel().expect("No deprel");

        attachments += 1;
        if gold_headidx != parser_headidx && gold_deprel != parser_deprel {
            combined_errors += 1;
        } else if gold_headidx != parser_headidx {
            head_errors += 1;
        } else if gold_deprel != parser_deprel {
            label_errors += 1;
        }
    }

    (attachments, combined_errors, head_errors, label_errors)
}
