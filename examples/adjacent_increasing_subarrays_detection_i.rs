use std::collections::HashSet;

fn main() {
    println!("{}", has_increasing_subarrays(vec![5, 8, -2, -1], 2))
}

pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
    let mut ans = 0;
    let mut current = 1;
    let mut pre_current = 0;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            current += 1;
        } else {
            pre_current = current;
            current = 1;
        }
        ans = ans.max(pre_current.min(current));
        ans = ans.max(current / 2);
    }

    ans >= k
}
