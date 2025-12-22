fn main() {
    println!(
        "{}",
        min_deletion_size(vec!["babca".to_string(), "bbazb".to_string()])
    )
}

// 最长递增自序列
pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let n = strs[0].len();
    let mut f = vec![0; n];
    for i in 0..n {
        for j in 0..i {
            if f[j] > f[i] && strs.iter().all(|s| s.as_bytes()[j] <= s.as_bytes()[i]) {
                f[i] = f[j];
            }
        }
        f[i] += 1;
    }
    n as i32 - *f.iter().max().unwrap()
}
