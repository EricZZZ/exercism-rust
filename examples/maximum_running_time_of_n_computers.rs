fn main() {}

pub fn max_run_time(mut n: i32, mut batteries: Vec<i32>) -> i64 {
    batteries.sort_unstable();
    let mut sum = batteries.iter().map(|&x| x as i64).sum::<i64>();
    for i in (0..batteries.len()).rev() {
        if batteries[i] as i64 <= sum / n as i64 {
            return sum / n as i64;
        }
        sum -= batteries[i] as i64;
        n -= 1;
    }
    0
}
