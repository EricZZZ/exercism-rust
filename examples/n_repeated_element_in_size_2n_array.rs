use std::collections::HashSet;

fn main() {
    println!("{}", repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]))
}

pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    for n in nums {
        if set.contains(&n) {
            return n;
        }
        set.insert(n);
    }
    0
}
