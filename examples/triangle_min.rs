use std::cmp;

fn main() {
    let ve = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    // let ve = vec![vec![-10]];
    // let ve = vec![vec![-1], vec![2, 3], vec![1, -1, -3]];
    println!("{}", minimum_total(ve))
}

pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    for i in 1..triangle.len() {
        triangle[i][0] += triangle[i - 1][0];
        for j in 1..i {
            let left = triangle[i - 1][j - 1] + triangle[i][j];
            let right = triangle[i - 1][j] + triangle[i][j];
            triangle[i][j] = cmp::min(left, right);
        }
        triangle[i][i] += triangle[i - 1][i - 1];
    }
    *triangle[triangle.len() - 1].iter().min().unwrap()
}
