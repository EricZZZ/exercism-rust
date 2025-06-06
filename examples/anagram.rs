fn main() {}

use std::collections::HashSet;

fn sort(word: &str) -> String {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort();
    sorted.into_iter().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let sorted = sort(&lower_word);
    possible_anagrams
        .iter()
        .filter(|input| {
            let input_lower = input.to_lowercase();
            lower_word != input_lower && sorted == sort(&input_lower)
        })
        .cloned()
        .collect()
}

// use std::collections::HashSet;
use std::iter::FromIterator;

fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
    let result = anagrams_for(word, inputs);

    let expected: HashSet<&str> = HashSet::from_iter(expected.iter().cloned());

    assert_eq!(result, expected);
}

#[test]
fn test_no_matches() {
    let word = "diaper";

    let inputs = ["hello", "world", "zombies", "pants"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_detect_simple_anagram() {
    let word = "ant";

    let inputs = ["tan", "stand", "at"];

    let outputs = vec!["tan"];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_does_not_confuse_different_duplicates() {
    let word = "galea";

    let inputs = ["eagle"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_eliminate_anagram_subsets() {
    let word = "good";

    let inputs = ["dog", "goody"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_detect_anagram() {
    let word = "listen";

    let inputs = ["enlists", "google", "inlets", "banana"];

    let outputs = vec!["inlets"];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_multiple_anagrams() {
    let word = "allergy";

    let inputs = [
        "gallery",
        "ballerina",
        "regally",
        "clergy",
        "largely",
        "leading",
    ];

    let outputs = vec!["gallery", "regally", "largely"];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_case_insensitive_anagrams() {
    let word = "Orchestra";

    let inputs = ["cashregister", "Carthorse", "radishes"];

    let outputs = vec!["Carthorse"];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_unicode_anagrams() {
    let word = "ΑΒΓ";

    // These words don't make sense, they're just greek letters cobbled together.
    let inputs = ["ΒΓΑ", "ΒΓΔ", "γβα"];

    let outputs = vec!["ΒΓΑ", "γβα"];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_misleading_unicode_anagrams() {
    // Despite what a human might think these words different letters, the input uses Greek A and B
    // while the list of potential anagrams uses Latin A and B.
    let word = "ΑΒΓ";

    let inputs = ["ABΓ"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_does_not_detect_a_word_as_its_own_anagram() {
    let word = "banana";

    let inputs = ["banana"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_does_not_detect_a_differently_cased_word_as_its_own_anagram() {
    let word = "banana";

    let inputs = ["bAnana"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_does_not_detect_a_differently_cased_unicode_word_as_its_own_anagram() {
    let word = "ΑΒΓ";

    let inputs = ["ΑΒγ"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
//#[ignore]
fn test_same_bytes_different_chars() {
    let word = "a⬂"; // 61 E2 AC 82

    let inputs = ["€a"]; // E2 82 AC 61

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}
