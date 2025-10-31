use std::collections::HashSet;

fn main() {}

pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(2);
    let mut set = HashSet::with_capacity(nums.len());
    for num in nums {
        if set.contains(&num) {
            ans.push(num);
        } else {
            set.insert(num);
        }
    }
    ans
}
