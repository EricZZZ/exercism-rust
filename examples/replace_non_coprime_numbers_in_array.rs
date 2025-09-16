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

// 这是计算最小公倍数 (LCM) 的函数。
fn lcm(a: i32, b: i32) -> i32 {
    // 检查是否为零，避免除以零的错误。
    if a == 0 || b == 0 {
        return 0;
    }
    // 使用公式：LCM(a, b) = (|a * b|) / GCD(a, b)
    (a / gcd(a, b)) * b
}

pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for mut num in nums {
        while let Some(&last) = res.last() {
            let g = gcd(last, num);
            if g > 1 {
                num = lcm(last, num);
                res.pop();
            } else {
                break;
            }
        }
        res.push(num);
    }
    res
}
