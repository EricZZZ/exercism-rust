use std::collections::HashMap;

fn main() {}

pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(points.len());
    for p in points {
        map.entry(p[1]).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut ans = 0_i64;
    let mut s = 0_i64;
    for v in map.values() {
        let t = *v as i64 * (*v as i64 - 1) / 2;
        ans = (ans + s * t) % 1_000_000_007;
        s += t;
    }
    ans as i32
}
