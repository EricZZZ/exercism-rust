fn main() {
    println!("{}", minimum_pair_removal(vec![5, 2, 3, 1]))
}

pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut nums = nums.clone();
    while nums.len() > 1 {
        let mut is_ascending = true;
        let mut min = i32::MAX;
        let mut index = -1;

        for i in 0..nums.len() - 1 {
            let sum = nums[i] + nums[i + 1];
            if nums[i] > nums[i + 1] {
                is_ascending = false
            }
            if sum < min {
                min = sum;
                index = i as i32;
            }
        }

        if is_ascending {
            break;
        }

        ans += 1;
        nums[index as usize] = min;
        nums.remove(index as usize + 1);
    }
    ans
}
