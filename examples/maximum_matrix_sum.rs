fn main() {
    println!("{}", max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]))
}

pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut total = 0;
    let mut cnt = 0;
    let mut mn = i32::MAX;
    for row in matrix {
        for mut x in row {
            if x < 0 {
                cnt += 1;
                x = -x;
            }
            mn = mn.min(x);
            total += x as i64;
        }
    }

    if cnt % 2 > 0 {
        total -= (mn * 2) as i64;
    }
    total
}
