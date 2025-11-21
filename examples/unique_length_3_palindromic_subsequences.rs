use std::collections::HashSet;

fn main() {
    println!("{}", count_palindromic_subsequence("aabca".to_string()))
}

pub fn count_palindromic_subsequence(s: String) -> i32 {
    let mut ans = 0;

    for ch in 'a'..='z' {
        let mut chars = s.chars();
        let l = chars.position(|c| c == ch);
        let r = chars
            .rev()
            .position(|c| c == ch)
            .map(|pos| s.len() - 1 - pos);
        if let (Some(l), Some(r)) = (l, r) {
            if r > l + 1 {
                let uni_chars: HashSet<char> = s[l + 1..r].chars().collect();
                ans += uni_chars.len() as i32;
            }
        }
    }
    ans
}
