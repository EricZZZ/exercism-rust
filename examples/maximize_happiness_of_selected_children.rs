fn main() {}

pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
    happiness.sort_by_key(|x| -x);
    let mut ans = 0;
    for (i, &x) in happiness[..k as usize].iter().enumerate() {
        if x <= i as i32 {
            break;
        }
        ans += (x - i as i32) as i64;
    }
    ans
}
