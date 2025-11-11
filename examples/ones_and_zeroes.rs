fn main() {
    println!(
        "{}",
        find_max_form(
            vec![
                "10".to_string(),
                "0001".to_string(),
                "111001".to_string(),
                "1".to_string(),
                "0".to_string()
            ],
            5,
            3
        )
    )
}

// 动图规划，看视频 https://www.bilibili.com/video/BV16Y411v7Y6
pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];
    for str in strs {
        let one_num = str.chars().filter(|s| *s == '1').count();
        let zero_num = str.len() - one_num;

        for i in (zero_num..=m as usize).rev() {
            for j in (one_num..=n as usize).rev() {
                dp[i][j] = dp[i][j].max(dp[i - zero_num][j - one_num] + 1)
            }
        }
    }
    dp[m as usize][n as usize]
}
