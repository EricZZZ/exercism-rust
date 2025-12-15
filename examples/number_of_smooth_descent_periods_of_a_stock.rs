fn main() {
    println!("{}", get_descent_periods(vec![3, 2, 1, 4]))
}

pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for i in 0..matrix[0].len() {
        let max = matrix.iter().map(|row| row[i]).max().unwrap();
        for row in matrix.iter_mut() {
            if row[i] == -1 {
                row[i] = max;
            }
        }
    }
    matrix
}

pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
    let mut ans = 0_i64;
    let mut count = 1_i64;
    for (i, &p) in prices.iter().enumerate() {
        if i > 0 && p == prices[i - 1] - 1 {
            count += 1
        } else {
            count = 1
        }
        ans += count;
    }
    ans
}
