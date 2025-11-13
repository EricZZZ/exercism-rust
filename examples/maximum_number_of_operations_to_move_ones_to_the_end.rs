fn main() {}

pub fn max_operations(s: String) -> i32 {
    let mut ans = 0;
    let mut count = 0;
    let chars: Vec<char> = s.chars().collect();
    for (i, ch) in chars.iter().enumerate() {
        if *ch == '1' {
            count += 1
        } else if i > 0 && chars[i - 1] == '1' {
            ans += count;
        }
    }
    ans
}
