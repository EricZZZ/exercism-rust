use std::collections::HashMap;

fn main() {
    println!("{}", special_triplets(vec![8, 4, 2, 8, 4]))
}

pub fn special_triplets(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut freq_next: HashMap<i32, i32> = HashMap::new();

    for num in &nums {
        *freq_next.entry(*num).or_insert(0) += 1;
    }
    let mut ans = 0;
    let mut freq_prev = HashMap::new();
    for num in nums {
        *freq_next.entry(num).or_insert(0) -= 1;
        ans = (ans
            + *freq_next.get(&(num * 2)).unwrap_or(&0) as i64
                * *freq_prev.get(&(num * 2)).unwrap_or(&0) as i64)
            % MOD;
        *freq_prev.entry(num).or_insert(0) += 1;
    }
    ans as i32
}
