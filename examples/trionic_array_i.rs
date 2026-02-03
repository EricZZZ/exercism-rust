fn main() {
    println!("{}", is_trionic(vec![8, 9, 4, 6, 1]))
}

pub fn is_trionic(nums: Vec<i32>) -> bool {
    let mut p = -1;
    let mut q = -1;
    for i in 0..nums.len() - 1 {
        if p == -1 {
            if nums[i] > nums[i + 1] {
                p = i as i32;
            } else if nums[i] == nums[i + 1] {
                return false;
            }
        } else if q == -1 {
            if nums[i] < nums[i + 1] {
                q = i as i32;
            } else if nums[i] == nums[i + 1] {
                return false;
            }
        } else if nums[i] >= nums[i + 1] {
            return false;
        }
    }
    println!("p:{},q:{}", p, q);
    0 < p && p < q && q < (nums.len() - 1) as i32
}
