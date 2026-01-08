use std::cmp;

fn main() {
    println!("{}", max_dot_product(vec![3, -2], vec![2, -6, 7]))
}

pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums1.len();
    let m = nums2.len();

    let mut dp = vec![vec![i32::MIN; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            let current_product = nums1[i - 1] * nums2[j - 1];
            let combined = if dp[i - 1][j - 1] > 0 {
                dp[i - 1][j - 1] + current_product
            } else {
                current_product
            };
            dp[i][j] = cmp::max(combined, cmp::max(dp[i - 1][j], dp[i][j - 1]));
        }
    }
    dp[n][m]
}
