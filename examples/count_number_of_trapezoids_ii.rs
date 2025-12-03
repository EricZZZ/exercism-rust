use core::f64;
use std::collections::{BTreeMap, HashMap};

fn main() {
    println!(
        "{}",
        count_trapezoids(vec![
            vec![34, 88],
            vec![-62, -38],
            vec![26, 88],
            vec![91, 88],
            vec![47, -38]
        ])
    )
}

pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    let to_key = |val: f64| -> u64 {
        if val == 0.0 {
            0.0f64.to_bits() // 统一将 -0.0 和 0.0 转为正 0.0 的 bits
        } else {
            val.to_bits()
        }
    };

    let mut group: HashMap<u64, Vec<f64>> = HashMap::new();
    let mut group2: HashMap<i32, Vec<f64>> = HashMap::new();

    for i in 0..points.len() {
        let x = points[i][0];
        let y = points[i][1];
        for j in 0..i {
            let x2 = points[j][0];
            let y2 = points[j][1];
            let dy = y - y2;
            let dx = x - x2;
            let (k, b) = if dx != 0 {
                let k = dy as f64 / dx as f64;
                let b = (y * dx - dy * x) as f64 / dx as f64;
                (k, b)
            } else {
                (f64::MAX, x as f64)
            };
            group.entry(to_key(k)).or_default().push(b);
            let mid = ((x + x2 + 2000) << 16) | (y + y2 + 2000);
            group2.entry(mid).or_default().push(k);
        }
    }
    let mut ans = 0;

    for g in group.values() {
        if g.len() == 1 {
            continue;
        }

        let mut cnt: HashMap<u64, i32> = HashMap::new();
        for &b in g {
            *cnt.entry(to_key(b)).or_default() += 1;
        }

        let mut s = 0;
        for &c in cnt.values() {
            ans += s * c;
            s += c;
        }
    }

    for g in group2.values() {
        if g.len() == 1 {
            continue;
        }

        let mut cnt: HashMap<u64, i32> = HashMap::new();
        for &k in g {
            *cnt.entry(to_key(k)).or_default() += 1;
        }

        let mut s = 0;
        for &c in cnt.values() {
            ans -= s * c; // 减去平行四边形多统计的一次
            s += c;
        }
    }

    ans
}
