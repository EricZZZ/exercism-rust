fn main() {}

pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut ans = 0;
    // 有效三角形 a+b>c;b+c>a;a+c>b;
    for c_index in 2..nums.len() {
        let c = nums[c_index];
        let mut a_index = 0;
        let mut b_index = c_index - 1;
        while a_index < b_index {
            if nums[a_index] + nums[b_index] > c {
                ans += b_index - a_index;
                b_index -= 1;
            } else {
                a_index += 1;
            }
        }
    }
    ans as i32
}
