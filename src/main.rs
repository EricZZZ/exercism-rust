use std::collections::VecDeque;

fn main() {
    let s1 = "abc".to_string();
    let s2 = "c".to_string();
    println!("{}", min_operations(vec![4, 1, 3], 4));
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

pub fn is_valid(s: String) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }
    let chars: Vec<char> = s.chars().collect();
    let mut quenu = VecDeque::<char>::new();
    for ch in chars {
        if ch == '(' || ch == '{' || ch == '[' {
            quenu.push_back(ch);
        }
        if ch == ')' && quenu.pop_back() != Some('(') {
            return false;
        }
        if '}' == ch && quenu.pop_back() != Some('{') {
            return false;
        }
        if ']' == ch && quenu.pop_back() != Some('[') {
            return false;
        }
    }
    quenu.is_empty()
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack == needle {
        return 0;
    }
    if !haystack.contains(&needle) {
        return -1;
    }
    for i in 0..haystack.len() {
        println!("{}", haystack.get(i..i + needle.len()).unwrap());
        if haystack.get(i..i + needle.len()) == Some(&needle) {
            return i as i32;
        }
    }
    -1
}

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let sum: i32 = nums.iter().sum();
    sum % k
}
