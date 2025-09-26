use std::fmt::format;

fn main() {
    let s = "hello".to_string();
    let mut ans = 0;
    let bytes = s.as_bytes();
    for i in 1..bytes.len() {
        ans += (bytes[i - 1] as i32 - bytes[i] as i32).abs();
    }
    println!("{}", ans);
}

pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    let mut ans = Vec::new();
    for (index, words) in words.iter().enumerate() {
        for ch in words.chars() {
            if ch == x {
                ans.push(index as i32);
                break;
            }
        }
    }
    ans
}

pub fn convert_date_to_binary(date: String) -> String {
    date.split('-')
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|n| format!("{:b}", n))
        .collect::<Vec<_>>()
        .join("-")
}
