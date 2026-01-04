fn main() {}

pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for &num in &nums {
        let mut count = 0;
        let mut sum = 0;
        let mut i = 1;
        while i * i <= num {
            if num % i == 0 {
                count += 1;
                sum += i;
                if i * i != num {
                    count += 1;
                    sum += num / i;
                }
            }
            i += 1;
        }
        if count == 4 {
            ans += sum;
        }
    }
    ans
}
