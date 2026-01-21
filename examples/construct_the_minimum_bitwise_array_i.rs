fn main() {
    println!("{:?}", min_bitwise_array(vec![11, 13, 31]))
}

pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![0; n];

    fn f(m: i32) -> i32 {
        for i in 0..m {
            if (i | (i + 1)) == m {
                return i;
            }
        }
        -1
    }

    for i in 0..n {
        ans[i] = f(nums[i])
    }
    ans
}
