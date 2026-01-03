fn main() {
    println!("{}", num_of_ways(5000))
}

pub fn num_of_ways(n: i32) -> i32 {
    let n = n as usize;
    let mut f = vec![0_i64; n + 1];
    f[0] = 3;
    f[1] = 12;
    for i in 2..=n {
        f[i] = (f[i - 1] * 5 - f[i - 2] * 2) % 1_000_000_007;
    }
    ((f[n] + 1_000_000_007) % 1_000_000_007) as i32
}
