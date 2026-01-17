use std::collections::HashSet;

fn main() {
    println!("{}", maximize_square_area(4, 3, vec![2, 3], vec![2]))
}

pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
    fn f(mut a: Vec<i32>, mx: i32) -> HashSet<i32> {
        a.push(1);
        a.push(mx);
        a.sort_unstable();
        let n = a.len();
        let mut set = HashSet::new();
        for i in 0..n {
            for j in i + 1..n {
                set.insert(a[j] - a[i]);
            }
        }
        set
    }

    let h_set = f(h_fences, m);
    let v_set = f(v_fences, n);

    let mut ans = -1;
    for x in h_set {
        if v_set.contains(&x) {
            ans = ans.max(x);
        }
    }
    if ans == -1 {
        return -1;
    }
    (ans as i64 * ans as i64 % 1_000_000_007) as i32
}
