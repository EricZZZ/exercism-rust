fn main() {
    println!("{:?}", prefixes_div_by5(vec![0, 1, 1]))
}

pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut ans: Vec<bool> = Vec::new();
    let mut res = 0_i32;
    for num in nums {
        res = ((res << 1) + num) % 5;
        ans.push(res == 0);
    }
    ans
}
