fn main() {
    println!("{}", count_valid_selections(vec![2, 3, 4, 0, 4, 1, 0]))
}

pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
    fn is_valid(mut index: i32, nums: &Vec<i32>, mut direction: i32, mut nozero: i32) -> bool {
        let mut nums = nums.clone();
        while nozero > 0 && index >= 0 && index < nums.len() as i32 {
            if nums[index as usize] > 0 {
                nums[index as usize] -= 1;
                direction *= -1;
                if nums[index as usize] == 0 {
                    nozero -= 1;
                }
            }
            index += direction;
        }
        nozero == 0
    }

    let mut ans = 0;
    let nozero = nums.iter().filter(|&&x| x > 0).count() as i32;
    for i in 0..nums.len() {
        if nums[i] == 0 {
            if is_valid(i as i32, &nums, 1, nozero) {
                ans += 1;
            }
            if is_valid(i as i32, &nums, -1, nozero) {
                ans += 1;
            }
        }
    }
    ans
}
