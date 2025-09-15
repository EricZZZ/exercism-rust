use std::collections::HashSet;

fn main() {
    let text = "hello world".to_string();
    let br = "ad".to_string();
    println!("{:?}", text.split(' ').collect::<Vec<&str>>());
    can_be_typed_words(text, br);
}
pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let hashset: HashSet<char> = broken_letters.chars().collect();

    text.split(" ")
        .filter(|word| word.chars().all(|ch| !hashset.contains(&ch)))
        .count() as i32
}
