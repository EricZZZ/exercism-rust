use std::collections::{BTreeSet, HashMap};

fn main() {
    println!(
        "{:?}",
        find_x_sum(
            vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000],
            6,
            1
        )
    )
}

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
    fn add(
        left: &mut BTreeSet<(i32, i32)>,
        right: &mut BTreeSet<(i32, i32)>,
        map: &HashMap<i32, i32>,
        sum: &mut i64,
        val: i32,
    ) {
        let cur = *map.get(&val).unwrap_or(&0);
        if cur == 0 {
            return;
        }
        let x = (cur, val);
        if !right.is_empty() && *right.first().unwrap() < x {
            right.insert(x);
            *sum += cur as i64 * val as i64;
        } else {
            left.insert(x);
        }
    }

    fn remove(
        left: &mut BTreeSet<(i32, i32)>,
        right: &mut BTreeSet<(i32, i32)>,
        map: &HashMap<i32, i32>,
        sum: &mut i64,
        val: i32,
    ) {
        let cur = *map.get(&val).unwrap_or(&0);
        if cur == 0 {
            return;
        }
        let x = (cur, val);
        if right.contains(&x) {
            right.remove(&x);
            *sum -= cur as i64 * val as i64;
        } else {
            left.remove(&x);
        }
    }
    fn left_out(left: &mut BTreeSet<(i32, i32)>, right: &mut BTreeSet<(i32, i32)>, sum: &mut i64) {
        let x = left.pop_last().unwrap();
        *sum += x.0 as i64 * x.1 as i64;
        right.insert(x);
    }
    fn right_out(left: &mut BTreeSet<(i32, i32)>, right: &mut BTreeSet<(i32, i32)>, sum: &mut i64) {
        let x = right.pop_first().unwrap();
        *sum -= x.0 as i64 * x.1 as i64;
        left.insert(x);
    }
    let mut map = HashMap::new();
    let n = nums.len();
    let k = k as usize;
    let x = x as usize;
    let mut ans = vec![0_i64; n - k + 1];
    let mut sum = 0;
    let mut left = BTreeSet::new();
    let mut right = BTreeSet::new();
    for i in 0..n {
        remove(&mut left, &mut right, &map, &mut sum, nums[i]);
        *map.entry(nums[i]).or_insert(0) += 1;
        add(&mut left, &mut right, &map, &mut sum, nums[i]);

        // 防止整型溢出问题，attempt to subtract with overflow
        let l = match (i + 1).checked_sub(k) {
            Some(val) => val,
            None => continue,
        };

        while !left.is_empty() && right.len() < x {
            left_out(&mut left, &mut right, &mut sum);
        }
        while right.len() > x {
            right_out(&mut left, &mut right, &mut sum);
        }

        ans[l] = sum;

        remove(&mut left, &mut right, &map, &mut sum, nums[l]);
        *map.entry(nums[l]).or_insert(0) -= 1;
        add(&mut left, &mut right, &map, &mut sum, nums[l]);
    }
    ans
}
