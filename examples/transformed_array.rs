fn main() {
    println!("{:?}", construct_transformed_array(vec![-10]))
}

pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![0; nums.len()];
    for i in 0..n {
        let step = (nums[i] + i as i32) % n as i32;
        let mut index = 0;
        if step > n as i32 {
            index = (step - n as i32) % n as i32;
        } else if step < 0 {
            index = (n as i32 + step) % n as i32;
        } else {
            index = step;
        }
        println!("index:{},step:{}", index, step);
        ans[i] = nums[index as usize];
    }

    ans
}
