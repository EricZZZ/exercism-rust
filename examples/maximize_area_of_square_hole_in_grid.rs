use std::cmp;

fn main() {
    println!("{}", maximize_square_hole_area(2, 1, vec![2, 3], vec![2]))
}

pub fn maximize_square_hole_area(
    n: i32,
    m: i32,
    mut h_bars: Vec<i32>,
    mut v_bars: Vec<i32>,
) -> i32 {
    fn f(a: &mut [i32]) -> i32 {
        a.sort_unstable();
        let mut mx = 1;
        let mut cnt = 1;
        for i in 1..a.len() {
            if a[i] == a[i - 1] + 1 {
                cnt += 1;
                mx = mx.max(cnt);
            } else {
                cnt = 1;
            }
        }
        mx
    }

    let side = cmp::min(f(&mut h_bars), f(&mut v_bars)) + 1;
    side * side
}
