fn main() {
    println!("{}", minimum_cost(vec![1, 2, 3, 12]))
}

pub fn minimum_cost(nums: Vec<i32>) -> i32 {
    let mut min_f = i32::MAX;
    let mut min_s = i32::MAX;
    for n in &nums[1..] {
        if *n < min_f {
            min_s = min_f;
            min_f = *n;
        } else if *n < min_s {
            min_s = *n
        }
    }
    nums[0] + min_f + min_s
}
