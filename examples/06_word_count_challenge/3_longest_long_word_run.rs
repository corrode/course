/// Returns the greatest number of consecutive words with more than three
/// Unicode scalar values. Words are whitespace-separated, and punctuation
/// remains part of each word. A word with three or fewer scalar values breaks
/// the run; whitespace alone does not. Returns 0 when no words qualify.
fn longest_long_word_run(text: &str) -> usize {
    todo!()
}

#[test]
fn empty_and_short_words_have_no_run() {
    assert_eq!(longest_long_word_run(""), 0);
    assert_eq!(longest_long_word_run(" \t\n"), 0);
    assert_eq!(longest_long_word_run("a to cat"), 0);
}

#[test]
fn short_words_separate_runs() {
    assert_eq!(longest_long_word_run("four five a seven eight"), 2);
    assert_eq!(longest_long_word_run("four cat five"), 1);
}

#[test]
fn best_run_can_end_before_the_last_word() {
    assert_eq!(longest_long_word_run("four five seven a eight"), 3);
    assert_eq!(longest_long_word_run("four five seven a"), 3);
}

#[test]
fn best_run_can_reach_the_end() {
    assert_eq!(longest_long_word_run("a four five seven"), 3);
    assert_eq!(longest_long_word_run("four a five seven eight"), 3);
    assert_eq!(longest_long_word_run("four"), 1);
}

#[test]
fn whitespace_does_not_break_a_run() {
    assert_eq!(longest_long_word_run("  four\t\tfive\nseven  "), 3);
    assert_eq!(longest_long_word_run("four\u{2003}five"), 2);
}

#[test]
fn scalar_values_determine_whether_a_word_qualifies() {
    assert_eq!(longest_long_word_run("café été four"), 1);
    assert_eq!(longest_long_word_run("猫猫猫 猫猫猫猫 four"), 2);
    assert_eq!(longest_long_word_run("abe\u{301} four"), 2);
}

#[test]
fn punctuation_remains_part_of_a_word() {
    assert_eq!(longest_long_word_run("cat! four"), 2);
    assert_eq!(longest_long_word_run("four ! five"), 1);
}
