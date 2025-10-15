fn main() {
    println!(
        "{:?}",
        max_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1])
    );
}

pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
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

    ans
}
