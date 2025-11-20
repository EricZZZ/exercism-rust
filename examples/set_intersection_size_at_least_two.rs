fn main() {}

pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by(|a, b| {
        if a[1] == b[1] {
            b[0].cmp(&a[0])
        } else {
            a[1].cmp(&b[1])
        }
    });

    let mut ans = 0;
    let mut s = -1;
    let mut e = -1;

    for v in intervals.iter() {
        let a = v[0];
        let b = v[1];

        if a <= s {
            continue;
        }

        if a <= e {
            ans += 1;
            s = e;
            e = b;
        } else {
            ans += 2;
            s = b - 1;
            e = b;
        }
    }

    ans
}
