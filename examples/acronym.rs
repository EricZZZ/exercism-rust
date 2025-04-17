use core::str;

fn main() {}
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || !c.is_alphanumeric())
        .flat_map(split_camel)
        .filter_map(|str| str.chars().next())
        .collect::<String>()
        .to_uppercase()
}

fn split_camel(phrase: &str) -> Vec<String> {
    let chars: Vec<char> = phrase.chars().collect();
    let mut words: Vec<String> = Vec::new();
    let mut word_start: usize = 0;
    for (i, c) in chars.iter().enumerate() {
        if i == chars.len() - 1 || c.is_lowercase() && chars[i + 1].is_uppercase() {
            words.push(chars[word_start..i + 1].iter().cloned().collect());
            word_start = i + 1;
        }
    }
    words
}

#[test]
fn empty() {
    assert_eq!(abbreviate(""), "");
}

#[test]
//#[ignore]
fn basic() {
    assert_eq!(abbreviate("Portable Network Graphics"), "PNG");
}

#[test]
//#[ignore]
fn lowercase_words() {
    assert_eq!(abbreviate("Ruby on Rails"), "ROR");
}

#[test]
//#[ignore]
fn camelcase() {
    assert_eq!(abbreviate("HyperText Markup Language"), "HTML");
}

#[test]
//#[ignore]
fn punctuation() {
    assert_eq!(abbreviate("First In, First Out"), "FIFO");
}

#[test]
//#[ignore]
fn all_caps_words() {
    assert_eq!(abbreviate("PHP: Hypertext Preprocessor"), "PHP");
}

#[test]
//#[ignore]
fn non_acronym_all_caps_word() {
    assert_eq!(abbreviate("GNU Image Manipulation Program"), "GIMP");
}

#[test]
//#[ignore]
fn hyphenated() {
    assert_eq!(
        abbreviate("Complementary metal-oxide semiconductor"),
        "CMOS"
    );
}
