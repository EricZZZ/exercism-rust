fn main() {
    println!(
        "{}",
        largest_magic_square(vec![
            vec![7, 1, 4, 5, 6],
            vec![2, 5, 1, 6, 4],
            vec![1, 5, 4, 3, 2],
            vec![1, 2, 7, 3, 4]
        ])
    )
}

pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    // 初始化前缀和矩阵
    let mut row_sum = vec![vec![0; n + 1]; m];
    let mut col_sum = vec![vec![0; n]; m + 1];
    let mut diag_sum = vec![vec![0; n + 1]; m + 1];
    let mut anti_sum = vec![vec![0; n + 1]; m + 1];

    for i in 0..m {
        for j in 0..n {
            let x = grid[i][j];
            row_sum[i][j + 1] = row_sum[i][j] + x;
            col_sum[i + 1][j] = col_sum[i][j] + x;
            diag_sum[i + 1][j + 1] = diag_sum[i][j] + x;
            anti_sum[i + 1][j] = anti_sum[i][j + 1] + x;
        }
    }

    // 从最大可能的边长 k 开始向下尝试
    for k in (2..=m.min(n)).rev() {
        for i in k..=m {
            'next_j: for j in k..=n {
                // 子矩阵主对角线的和 (i,j) 是右下角坐标
                let target_sum = diag_sum[i][j] - diag_sum[i - k][j - k];

                // 检查反对角线
                if anti_sum[i][j - k] - anti_sum[i - k][j] != target_sum {
                    continue;
                }

                // 检查每一行
                for r in i - k..i {
                    if row_sum[r][j] - row_sum[r][j - k] != target_sum {
                        continue 'next_j;
                    }
                }

                // 检查每一列
                for c in j - k..j {
                    if col_sum[i][c] - col_sum[i - k][c] != target_sum {
                        continue 'next_j;
                    }
                }

                return k as i32;
            }
        }
    }

    1
}
