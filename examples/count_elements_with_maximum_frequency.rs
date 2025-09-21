use std::{cmp, collections::HashMap};

fn main() {}

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut max = 0;

    // 一次遍历
    // let mut res = 0;
    // for num in nums {
    //     let val = map.entry(num).or_insert(0);
    //     *val += 1;
    //     if *val > max {
    //         max = *val;
    //         res = *val;
    //     } else if *val == max {
    //         res += *val;
    //     }
    // }
    // res

    // 插入map中，再遍历map value，统计数量等于max
    for num in nums {
        let val = map.entry(num).or_insert(0);
        *val += 1;
        max = cmp::max(*val, max);
    }
    map.into_values().filter(|n| *n == max).sum()
}
