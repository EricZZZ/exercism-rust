use std::collections::HashMap;

fn main() {
    println!("{:?}", find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2))
}

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut ans: Vec<i32> = Vec::new();
    if k == 0 || nums.is_empty() || k as usize > nums.len() {
        return ans;
    }
    for num in nums.iter().take(k as usize) {
        *map.entry(*num).or_default() += 1;
    }
    fn x_sum(x: usize, map: &HashMap<i32, i32>, ans: &mut Vec<i32>) {
        let mut sorted_vec: Vec<(&i32, &i32)> = map.iter().collect();
        sorted_vec.sort_by(|a, b| {
            // 1. 比较值 (Value)
            b.1.cmp(a.1)
                // 2. 如果值相等 (Equal)，则比较键 (Key)
                .then_with(|| b.0.cmp(a.0))
        });
        let sum: i32 = sorted_vec.iter().take(x).map(|v| (*v.0) * (*v.1)).sum();
        ans.push(sum);
    }
    // initial window
    x_sum(x as usize, &map, &mut ans);
    for i in k as usize..nums.len() {
        let new_add = nums[i];
        *map.entry(new_add).or_default() += 1;
        // remove outgoing element
        let remove = nums[i - k as usize];
        if let Some(cnt) = map.get_mut(&remove) {
            *cnt -= 1;
            if *cnt == 0 {
                map.remove(&remove);
            }
        }
        x_sum(x as usize, &map, &mut ans);
    }
    ans
}
