fn main() {
    println!("{}", k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2))
}

pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let mut pre: i32 = -1;
    for (i, num) in nums.iter().enumerate() {
        if pre == -1 && *num == 1 {
            pre = i as i32;
        } else if pre != -1 && *num == 1 {
            if i as i32 - pre - 1 < k {
                return false;
            } else {
                pre = i as i32;
            }
        }
    }
    true
}
