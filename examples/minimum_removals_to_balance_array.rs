fn main() {
    println!("{}", min_removal(vec![2, 1, 5], 2))
}

// 滑动窗口
pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let mut left = 0;
    nums.sort_unstable();
    for i in 0..nums.len() {
        while nums[left] as i64 * (k as i64) < nums[i] as i64 {
            left += 1;
        }
        ans = ans.max(i - left + 1);
    }
    (nums.len() - ans) as _
}
