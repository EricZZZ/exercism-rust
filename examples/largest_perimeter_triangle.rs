fn main() {
    println!("{}", largest_perimeter(vec![2, 1, 2]));
}

pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    for (i, _) in nums.iter().rev().take(n - 2).enumerate() {
        if nums[n - 1 - i] < (nums[n - i - 2] + nums[n - i - 3]) {
            return nums[n - 1 - i] + nums[n - i - 2] + nums[n - i - 3];
        }
    }
    0
}
