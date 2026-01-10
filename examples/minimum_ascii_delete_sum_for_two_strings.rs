use std::cmp;

fn main() {
    println!(
        "{}",
        minimum_delete_sum("sea".to_string(), "eat".to_string())
    )
}

pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let total = s1.as_bytes().iter().map(|&b| b as i32).sum::<i32>()
        + s2.as_bytes().iter().map(|&b| b as i32).sum::<i32>();

    let s = s1.as_bytes();
    let t = s2.as_bytes();
    let n = s.len();
    let m = t.len();

    let mut f = vec![vec![0_i32; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            if s[i] == t[j] {
                f[i + 1][j + 1] = f[i][j] + s[i] as i32;
            } else {
                f[i + 1][j + 1] = cmp::max(f[i][j + 1], f[i + 1][j]);
            }
        }
    }
    total - f[n][m] * 2
}
