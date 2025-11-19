use std::collections::{HashMap, HashSet};

fn main() {}

pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    let mut ans = original;
    let set: HashSet<i32> = nums.into_iter().collect();
    while set.contains(&ans) {
        ans *= 2;
    }
    ans
}
