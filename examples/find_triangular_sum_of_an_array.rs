fn main() {
    println!("{}", triangular_sum(vec![1, 2, 3, 4, 5]));
}

pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
    for i in (0..nums.len()).rev() {
        for j in 0..i {
            nums[j] = (nums[j] + nums[j + 1]) % 10;
        }
    }
    nums[0]
}
