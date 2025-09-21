use std::cmp;

fn main() {
    let nums = vec![1, 3, 2];
    let k = 2;
    println!("{}", max_total_value(nums, k))
}

pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
    // let mut res = 0;
    // for c in nums {
    //     if c % 2 == 0 {
    //         res |= c;
    //     }
    // }
    // res
    nums.iter()
        .filter(|&c| c % 2 == 0)
        .fold(0, |acc, &x| acc | x)
}

pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let sormadexin = nums;
    let n = sormadexin.len();

    if n == 0 || k <= 0 {
        return 0;
    }

    let mut max_subarray_value: i32 = 0;

    for l in 0..n {
        let mut current_min = sormadexin[l];
        let mut current_max = sormadexin[l];
        for r in l..n {
            current_min = current_min.min(sormadexin[r]);
            current_max = current_max.max(sormadexin[r]);

            let current_value = current_max - current_min;

            if current_value > max_subarray_value {
                max_subarray_value = current_value;
            }
        }
    }

    (max_subarray_value as i64) * (k as i64)
}
