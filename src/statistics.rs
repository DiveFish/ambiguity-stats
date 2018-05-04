pub fn precision(correct: usize, parser_count: usize) -> usize {
    correct / parser_count
}

pub fn recall(correct: usize, gold_count: usize) -> usize {
    correct / gold_count
}

pub fn f1score(precision: usize, recall: usize) -> usize {
    (2 * precision * recall) / (precision + recall)
}