fn main() {
    println!("{}", num_sub("0110111".to_string()))
}

pub fn num_sub(s: String) -> i32 {
    let mut ans = 0_i64;

    let chars: Vec<char> = s.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        if chars[index] == '1' {
            let mut one_count = 1;
            while index + 1 < chars.len() && chars[index + 1] == '1' {
                one_count += 1;
                index += 1;
            }
            ans += (one_count + 1) * one_count / 2;
        }
        index += 1;
    }

    (ans % (1000000000 + 7)) as i32
}
