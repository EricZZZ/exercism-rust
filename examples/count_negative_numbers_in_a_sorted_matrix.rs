fn main() {}

pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for g in grid {
        let count = g.iter().filter(|&&x| x < 0).count();
        ans += count;
    }
    ans as i32
}
