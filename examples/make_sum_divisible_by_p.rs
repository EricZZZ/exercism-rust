use std::collections::HashMap;

fn main() {
    println!("{}", min_subarray(vec![3, 1, 4, 2], 6))
}

pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let mut x = 0;
    let n = nums.len();
    for num in &nums {
        x = (x + num) % p;
    }
    if x == 0 {
        return 0;
    }
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut y = 0;
    let mut res = n;
    for (i, num) in nums.iter().enumerate() {
        map.insert(y, i);
        println!("i:{},num:{}", i, num);
        y = (y + num) % p;
        if map.contains_key(&((y - x + p) % p)) {
            res = res.min(i - *map.get(&((y - x + p) % p)).unwrap() + 1);
        }
    }
    if res == n {
        -1
    } else {
        res as i32
    }
}
