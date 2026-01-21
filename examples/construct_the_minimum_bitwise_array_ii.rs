fn main() {
    println!("{:?}", min_bitwise_array(vec![2, 3, 5, 7]))
}

// 位运算太优雅了
pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![0; n];

    for i in 0..n {
        let x = nums[i];
        if x % 2 == 0 {
            ans[i] = -1;
        } else {
            ans[i] = x ^ ((x + 1) & !x) >> 1;
        }
    }
    ans
}
