fn main() {
    println!("{}", max_profit(vec![4, 2, 8], vec![-1, 0, 1], 2))
}

// 前缀和
pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
    let n = prices.len();
    let mut sum = vec![0_i64; n + 1];
    let mut sum_sell = vec![0_i64; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + prices[i] as i64 * strategy[i] as i64;
        sum_sell[i + 1] = sum_sell[i] + prices[i] as i64;
    }
    let mut ans = sum[n];
    for i in k as usize..=n {
        ans = ans
            .max(sum[i - k as usize] + sum[n] - sum[i] + sum_sell[i] - sum_sell[i - k as usize / 2])
    }
    ans
}
