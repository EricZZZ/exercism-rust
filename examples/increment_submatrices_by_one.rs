fn main() {
    println!("{:?}", range_add_queries(2, vec![vec![0, 0, 1, 1]]))
}

pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans = vec![vec![0; n as usize]; n as usize];
    for q in queries {
        for i in &mut ans[q[0] as usize..=q[2] as usize] {
            for j in &mut i[q[1] as usize..=q[3] as usize] {
                *j += 1;
            }
        }
    }
    ans
}

pub fn range_add_queries_right(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut diff = vec![vec![0; n + 2]; n + 2];
    for q in queries {
        let (r1, c1, r2, c2) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
        diff[r1 + 1][c1 + 1] += 1;
        diff[r1 + 1][c2 + 2] -= 1;
        diff[r2 + 2][c1 + 1] -= 1;
        diff[r2 + 2][c2 + 2] += 1;
    }

    let mut ans = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            diff[i + 1][j + 1] += diff[i + 1][j] + diff[i][j + 1] - diff[i][j];
            ans[i][j] = diff[i + 1][j + 1];
        }
    }

    ans
}
