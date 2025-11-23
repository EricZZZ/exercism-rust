fn main() {}

pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut a1 = Vec::new();
    let mut a2 = Vec::new();
    let mut sum = 0;
    for num in nums {
        sum += num;
        if num % 3 == 1 {
            a1.push(num);
        } else if num % 3 == 2 {
            a2.push(num);
        }
    }
    if sum % 3 == 0 {
        return sum;
    }
    a1.sort_unstable();
    a2.sort_unstable();
    if sum % 3 == 2 {
        let temp = a1.clone();
        a1 = a2;
        a2 = temp;
    }
    let mut ans = 0;
    if !a1.is_empty() {
        ans = sum - a1[0];
    }
    if a2.len() > 1 {
        ans = std::cmp::max(ans, sum - a2[0] - a2[1]);
    }
    ans
}
