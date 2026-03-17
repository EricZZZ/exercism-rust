fn main() {
    println!(
        "{}",
        map_word_weights(
            vec!["abcd".to_string(), "def".to_string(), "xyz".to_string()],
            vec![5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2]
        )
    )
}

pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
    let mut ans = String::new();
    for word in words {
        let mut sum = 0;
        for &byte in word.as_bytes() {
            sum += weights[(byte - b'a') as usize];
        }
        ans.push((b'z' - (sum % 26) as u8) as char);
    }
    ans
}
