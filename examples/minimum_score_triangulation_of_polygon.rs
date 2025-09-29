use std::{cmp, i32};

fn main() {
    println!("{}", min_score_triangulation(vec![3, 7, 4, 5]));
}

// DP 问题总是很难理解
pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
    let n = values.len();
    let mut ans = vec![vec![0; n]; n];

    for i in (0..values.len() - 2).rev() {
        for j in i + 2..n {
            ans[i][j] = i32::MAX;
            for k in i + 1..j {
                ans[i][j] =
                    ans[i][j].min(ans[i][k] + ans[k][j] + values[i] * values[j] * values[k]);
            }
        }
    }

    ans[0][n - 1]
}
