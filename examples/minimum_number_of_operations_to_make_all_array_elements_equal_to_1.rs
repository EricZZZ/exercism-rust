fn main() {}

// 使用欧几里得算法计算最大公约数
fn gcd(a: i32, b: i32) -> i32 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let count = nums.iter().filter(|&&x| x == 1).count() as i32;
    if count > 0 {
        return nums.len() as i32 - count;
    }
    let mut m = nums.len() as i32 + 1;
    for i in 0..nums.len() {
        let mut x = 0;
        for j in i..nums.len() {
            x = gcd(x, nums[j]);
            if x == 1 {
                m = m.min((j - i + 1) as i32);
            }
        }
    }
    if m > nums.len() as i32 {
        -1
    } else {
        nums.len() as i32 - 1 + m - 1
    }
}
