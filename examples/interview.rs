fn main() {
    println!("{}", sovle("3[a]2[bc]".to_string()));
    println!("{}", sovle("3[a2[c]]".to_string()));
    println!("{}", sovle("2[abc]3[cd]ef".to_string()));
}

fn sovle(s: String) -> String {
    let mut stack: Vec<(u32, String)> = Vec::new();
    let mut ans = String::new();
    let mut count = 0;
    for ch in s.chars() {
        match ch {
            '[' => {
                stack.push((count, ans));
                count = 0;
                ans = String::new();
            }
            ']' => {
                if let Some((count, mut last_str)) = stack.pop() {
                    last_str.push_str(&ans.repeat(count as usize));
                    ans = last_str;
                }
            }
            '0'..='9' => {
                count = count * 10 + ch.to_digit(10).unwrap();
            }
            _ => {
                ans.push(ch);
            }
        }
    }

    ans
}
