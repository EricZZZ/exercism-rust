fn main() {
    println!(
        "{}",
        max_side_length(
            vec![
                vec![1, 1, 3, 2, 4, 3, 2],
                vec![1, 1, 3, 2, 4, 3, 2],
                vec![1, 1, 3, 2, 4, 3, 2]
            ],
            4
        )
    )
}

// 二维前缀和
pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut sum: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + mat[i][j];
        }
    }
    let mut ans = 0;

    fn query(sum: &[Vec<i32>], r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
        sum[r2 + 1][c2 + 1] - sum[r2 + 1][c1] - sum[r1][c2 + 1] + sum[r1][c1]
    }
    for i in 0..m {
        for j in 0..n {
            while i + ans < m && j + ans < n && query(&sum, i, j, i + ans, j + ans) <= threshold {
                ans += 1;
            }
        }
    }
    ans as i32
}
