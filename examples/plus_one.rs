fn main() {
    println!("{:?}", plus_one(vec![9, 9]))
}

pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let n = digits.len();
    for i in (0..digits.len()).rev() {
        if digits[i] < 9 {
            digits[i] += 1;
            return digits;
        }
        digits[i] = 0;
    }
    let mut ans = vec![0; n + 1];
    ans[0] = 1;
    ans
}
