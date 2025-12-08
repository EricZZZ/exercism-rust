fn main() {
    println!("{}", count_triples(18))
}

pub fn count_triples(n: i32) -> i32 {
    let mut ans = 0;
    for a in 1..=n {
        for b in a + 1..=n {
            for c in b + 1..=n {
                if (a * a + b * b) == c * c {
                    ans += 1;
                }
            }
        }
    }
    ans * 2
}

pub fn count_triples_i(n: i32) -> i32 {
    let mut ans = 0;
    for a in 1..=n {
        for b in 1..=n {
            let c = ((a * a + b * b) as f64).sqrt().floor() as i32;
            if c <= n && c * c == a * a + b * b {
                ans += 1;
            }
        }
    }
    ans
}
