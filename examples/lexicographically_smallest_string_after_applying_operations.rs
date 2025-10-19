fn main() {}
pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
    let n = s.len();
    let mut vis = vec![false; n];
    let mut res = s.clone();

    let s = s.repeat(2);
    let mut i = 0;
    while !vis[i] {
        vis[i] = true;
        for j in 0..10 {
            let k_limit = if b % 2 == 0 { 0 } else { 9 };
            for k in 0..=k_limit {
                let mut t: Vec<char> = s[i..i + n].chars().collect();
                for p in (1..n).step_by(2) {
                    let digit = t[p].to_digit(10).unwrap() as i32;
                    t[p] = std::char::from_digit(((digit + j * a) % 10) as u32, 10).unwrap();
                }
                for p in (0..n).step_by(2) {
                    let digit = t[p].to_digit(10).unwrap() as i32;
                    t[p] = std::char::from_digit(((digit + k * a) % 10) as u32, 10).unwrap();
                }
                let t_str: String = t.into_iter().collect();
                if t_str < res {
                    res = t_str;
                }
            }
        }
        i = (i + b as usize) % n;
    }
    res
}
