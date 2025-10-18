fn main() {
    println!("{}", max_distinct_elements(vec![1, 2, 2, 3, 3, 4], 2))
}

pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut ans = 0;
    let mut pre = i32::MIN;
    for n in nums {
        let x = (n - k).max(pre + 1).min(n + k);
        if x > pre {
            ans += 1;
            pre = x;
        }
    }
    ans
}
