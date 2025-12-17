fn main() {
    println!(
        "{}",
        maximum_profit(vec![12, 16, 19, 19, 8, 1, 19, 13, 9], 3)
    )
}

// 动态规划，太优雅了
pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
    let mut f = vec![vec![0_i64; 3]; k as usize + 2];
    for f in f.iter_mut().skip(1).take(k as usize + 1) {
        f[1] = i64::MIN / 2;
    }
    f[0][0] = i64::MIN / 2;
    for p in prices {
        for j in (1..=k as usize + 1).rev() {
            f[j][0] = f[j][0].max((f[j][1] + p as i64).max(f[j][2] - p as i64));
            f[j][1] = f[j][1].max(f[j - 1][0] - p as i64);
            f[j][2] = f[j][2].max(f[j - 1][0] + p as i64);
        }
    }
    f[k as usize + 1][0]
}
