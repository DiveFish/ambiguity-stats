pub fn precision(correct: usize, parser_count: usize) -> usize {
    correct / parser_count
}

pub fn recall(correct: usize, gold_count: usize) -> usize {
    correct / gold_count
}

pub fn f1score(precision: usize, recall: usize) -> usize {
    2 ((precision * recall) / (precision + recall))
}

pub fn most_freq_toks(input: String, n_freq: usize) -> Vec<(String, usize)> {
    let mut freq_map: HashMap<String, usize> = HashMap::new();
    let files = get_all_files(input);
    for file in files {
        let sents = read_data(&file);
        for sent in sents {
            for token in sent {
                let entry = freq_map.entry(token.form().to_string()).or_insert(0);
                *entry += 1;
            }
        }
    }

    let mut freq_vec: Vec<_> = freq_map.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let mut most_freq_vec = Vecc::with_capacity(n_freq);
    for i in 0..n_freq {
        most_freq_vec.push(freq_vec[i]);
    }

    most_freq_vec
}