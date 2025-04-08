fn main() {}

pub fn reverse(input: &str) -> String {
    // input.chars().rev().collect()
    let mut res = String::with_capacity(input.len());
    res.extend(input.chars().rev());
    res
}

#[test]
/// empty string
fn test_empty_string() {
    process_reverse_case("", "");
}

#[test]
//#[ignore]
/// a word
fn test_a_word() {
    process_reverse_case("robot", "tobor");
}

#[test]
//#[ignore]
/// a capitalized word
fn test_a_capitalized_word() {
    process_reverse_case("Ramen", "nemaR");
}

#[test]
//#[ignore]
/// a sentence with punctuation
fn test_a_sentence_with_punctuation() {
    process_reverse_case("I'm hungry!", "!yrgnuh m'I");
}

#[test]
//#[ignore]
/// a palindrome
fn test_a_palindrome() {
    process_reverse_case("racecar", "racecar");
}

#[test]
//#[ignore]
/// wide characters
fn test_wide_characters() {
    process_reverse_case("子猫", "猫子");
}
