fn main() {
    println!(
        "{}",
        min_deletion_size(vec![
            "rrjk".to_string(),
            "furt".to_string(),
            "guzm".to_string()
        ])
    )
}

// 就是矩阵从列开始遍历
pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut ans = 0;
    let row_count = strs.len();
    let col_count = strs[0].len();

    for i in 0..col_count {
        for j in 1..row_count {
            if strs[j - 1].as_bytes()[i] > strs[j].as_bytes()[i] {
                ans += 1;
                break;
            }
        }
    }
    ans
}
