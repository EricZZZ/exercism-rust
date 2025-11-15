fn main() {
    println!("{}", number_of_substrings("00011".to_string()))
}

pub fn number_of_substrings(s: String) -> i32 {
    let mut ans = 0;
    let chars: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut pre = vec![-1; n + 1];
    for i in 0..n {
        if i == 0 || chars[i - 1] == '0' {
            pre[i + 1] = i as i32;
        } else {
            pre[i + 1] = pre[i];
        }
    }

    for i in 1..=n {
        let mut zero_count = if chars[i - 1] == '0' { 1 } else { 0 };
        let mut j = i as i32;

        while j > 0 && (zero_count * zero_count) as usize <= n {
            let one_count = (i as i32 - pre[j as usize]) - zero_count;
            if zero_count * zero_count <= one_count {
                ans += std::cmp::min(j - pre[j as usize], one_count - zero_count * zero_count + 1);
            }
            j = pre[j as usize];
            zero_count += 1;
        }
    }
    ans
}
