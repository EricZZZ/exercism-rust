fn main() {}

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for num in nums {
        if num % 3 != 0 {
            ans += 1;
        }
    }
    ans
}
