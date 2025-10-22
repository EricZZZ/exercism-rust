use std::collections::{HashMap, HashSet};

fn main() {}

pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
    nums.sort();
    let mut ans = 0;
    let mut num_count = HashMap::new();
    let mut modes = HashSet::new();

    let mut add_mode = |value: i32| {
        modes.insert(value);
        if value - k >= nums[0] {
            modes.insert(value - k);
        }
        if value + k <= nums[nums.len() - 1] {
            modes.insert(value + k);
        }
    };

    let mut last_num_index = 0;
    for i in 0..nums.len() {
        if nums[i] != nums[last_num_index] {
            num_count.insert(nums[last_num_index], (i - last_num_index) as i32);
            ans = ans.max((i - last_num_index) as i32);
            add_mode(nums[last_num_index]);
            last_num_index = i;
        }
    }

    num_count.insert(nums[last_num_index], (nums.len() - last_num_index) as i32);
    ans = ans.max((nums.len() - last_num_index) as i32);
    add_mode(nums[last_num_index]);

    let left_bound = |value: i32| -> usize {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] < value {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    };

    let right_bound = |value: i32| -> usize {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = (left + right).div_ceil(2);
            if nums[mid] > value {
                right = mid - 1;
            } else {
                left = mid;
            }
        }
        left
    };

    let mut sorted_modes: Vec<i32> = modes.into_iter().collect();
    sorted_modes.sort();

    for mode in sorted_modes {
        let l = left_bound(mode - k);
        let r = right_bound(mode + k);

        let temp_ans = if let Some(&count) = num_count.get(&mode) {
            (r - l + 1).min(count as usize + num_operations as usize)
        } else {
            (r - l + 1).min(num_operations as usize)
        };
        ans = ans.max(temp_ans as i32);
    }
    ans
}
