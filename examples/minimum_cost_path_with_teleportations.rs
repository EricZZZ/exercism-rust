use std::cmp;

fn main() {
    println!(
        "{}",
        min_cost(vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]], 2)
    )
}

pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = grid[0].len();
    let mut mx = 0;

    for row in &grid {
        for &x in row {
            mx = cmp::max(mx, x);
        }
    }

    let mx_usize = mx as usize;

    let mut suf_min_f = vec![i32::MAX; mx_usize + 2];
    let mut min_f = vec![i32::MAX; mx_usize + 1];
    let mut f = vec![i32::MAX / 2; n + 1];

    for _ in 0..=k {
        f[1] = -grid[0][0];

        for row in &grid {
            for j in 0..n {
                let x = row[j];
                f[j + 1] = cmp::min(cmp::min(f[j], f[j + 1]) + x, suf_min_f[x as usize]);
                min_f[x as usize] = cmp::min(min_f[x as usize], f[j + 1]);
            }
        }

        for i in (0..=mx_usize).rev() {
            suf_min_f[i] = cmp::min(suf_min_f[i + 1], min_f[i]);
        }
    }
    f[n]
}
