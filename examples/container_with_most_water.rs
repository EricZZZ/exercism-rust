use std::cmp;

fn main() {
    println!("{}", max_area_2(vec![8, 7, 2, 1]));
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut l = 0;
    let mut r = height.len() - 1;
    for step in (1..height.len()).rev() {
        ans = cmp::max(ans, cmp::min(height[l], height[r]) * (step as i32));
        println!("l:{},r:{},step:{},ans:{}", height[l], height[r], step, ans);
        if height[r] > height[l] {
            l += 1;
        } else {
            r -= 1;
        }
    }
    ans
}

pub fn max_area_2(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut l = 0;
    let mut r = height.len() - 1;
    while l < r {
        ans = cmp::max(ans, cmp::min(height[l], height[r]) * (r - l) as i32);
        if height[r] > height[l] {
            l += 1;
        } else {
            r -= 1;
        }
    }
    ans
}
