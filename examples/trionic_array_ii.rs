fn main() {
    println!("{}", max_sum_trionic(vec![0, -2, -1, -3, 0, 2, -1]))
}

pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
    const NEG_INF: i64 = i64::MIN / 2;
    let mut ans = NEG_INF;
    let mut f1 = NEG_INF;
    let mut f2 = NEG_INF;
    let mut f3 = NEG_INF;
    for i in 1..nums.len() {
        let x = nums[i - 1] as i64;
        let y = nums[i] as i64;
        if x < y {
            f3 = f2.max(f3) + y;
        } else {
            f3 = NEG_INF;
        }
        if x > y {
            f2 = f2.max(f1) + y;
        } else {
            f2 = NEG_INF;
        }
        if x < y {
            f1 = f1.max(x) + y;
        } else {
            f1 = NEG_INF;
        }
        ans = ans.max(f3);
    }
    ans
}
