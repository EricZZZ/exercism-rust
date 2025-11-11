fn main() {}

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut stack: Vec<i32> = Vec::new();
    for &x in nums.iter() {
        while let Some(&last) = stack.last() {
            if last > x {
                ans += 1;
                stack.pop();
            } else {
                break;
            }
        }
        if x != 0 && (stack.is_empty() || *stack.last().unwrap() != x) {
            stack.push(x);
        }
    }
    ans += stack.len() as i32;
    ans
}
