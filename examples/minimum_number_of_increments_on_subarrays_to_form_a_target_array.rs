fn main() {
    println!("{}", min_number_operations(vec![1, 2, 3, 2, 1]))
}

pub fn min_number_operations(target: Vec<i32>) -> i32 {
    let mut ans = target[0];
    for i in 1..target.len() {
        ans += std::cmp::max(target[i] - target[i - 1], 0);
    }
    ans
}
