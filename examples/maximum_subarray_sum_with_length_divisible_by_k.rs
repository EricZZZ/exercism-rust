fn main() {}

pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut sum = vec![0_i64; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + nums[i] as i64;
    }
    let mut mins = vec![i64::MAX / 2; k as usize];
    let mut ans = i64::MAX;
    for j in 0..n + 1 {
        let i = j % k as usize;
        ans = ans.max(sum[j] - mins[i]);
        mins[i] = mins[i].min(sum[j])
    }
    ans
}
