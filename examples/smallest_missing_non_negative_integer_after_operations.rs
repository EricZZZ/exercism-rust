use std::collections::HashMap;

fn main() {
    println!("{}", find_smallest_integer(vec![0, -3], 4))
}

pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
    let mut map = HashMap::new();
    for n in nums {
        *map.entry((n % value + value) % value).or_insert(0) += 1;
    }
    for m in 0.. {
        if let Some(c) = map.get_mut(&(m % value)) {
            if *c > 0 {
                *c -= 1;
                continue;
            }
        }
        return m;
    }
    unreachable!()
}
