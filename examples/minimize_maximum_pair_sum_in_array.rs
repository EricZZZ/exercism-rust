fn main() {
    println!("{}", min_pair_sum(vec![3, 5, 2, 3]))
}

pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut ans = 0;
    for i in 0..n / 2 {
        let sum = nums[i] + nums[n - 1 - i];
        ans = ans.max(sum);
    }
    ans
}
