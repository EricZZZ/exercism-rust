fn main() {
    println!("{}", climb_stairs(4))
}

pub fn climb_stairs(n: i32) -> i32 {
    let (mut a, mut b) = (1, 1);
    let mut sum;
    for _ in 0..n as usize - 1 {
        sum = a + b;
        a = b;
        b = sum;
    }
    b
}
