extern crate conllx;

use conllx::Token;

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
