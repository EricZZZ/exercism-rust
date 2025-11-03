fn main() {
    println!("{}", min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1]))
}

pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut prev_char: Option<char> = None;
    let mut prev_time: i32 = 0;

    for (i, ch) in colors.chars().enumerate() {
        let t = needed_time[i];
        if Some(ch) == prev_char {
            ans += t.min(prev_time);
            prev_time = t.max(prev_time);
        } else {
            prev_char = Some(ch);
            prev_time = t;
        }
    }

    ans
}
