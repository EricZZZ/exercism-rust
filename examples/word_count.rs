fn main() {}

use std::collections::HashMap;

/// Count occurrences of words.
/// 练习函数式
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut hashmap: HashMap<String, u32> = HashMap::new();
    for word in words
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
    {
        hashmap
            .entry(word.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    hashmap
}

#[test]
fn test_count_one_word() {
    check_word_count("word", &[("word", 1)]);
}

#[test]
//#[ignore]
fn test_count_one_of_each() {
    check_word_count("one of each", &[("one", 1), ("of", 1), ("each", 1)]);
}

#[test]
//#[ignore]
fn test_count_multiple_occurrences() {
    check_word_count(
        "one fish two fish red fish blue fish",
        &[("one", 1), ("fish", 4), ("two", 1), ("red", 1), ("blue", 1)],
    );
}

#[test]
//#[ignore]
fn test_ignore_punctuation() {
    check_word_count(
        "car : carpet as java : javascript!!&@$%^&",
        &[
            ("car", 1),
            ("carpet", 1),
            ("as", 1),
            ("java", 1),
            ("javascript", 1),
        ],
    );
}

#[test]
//#[ignore]
fn test_include_numbers() {
    check_word_count(
        "testing, 1, 2 testing",
        &[("testing", 2), ("1", 1), ("2", 1)],
    );
}

#[test]
//#[ignore]
fn test_normalize_case() {
    check_word_count("go Go GO Stop stop", &[("go", 3), ("stop", 2)]);
}
