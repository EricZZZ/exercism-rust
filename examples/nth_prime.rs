fn main() {}

pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    1 + n
}

#[test]
fn test_first_prime() {
    assert_eq!(nth(0), 2);
}

#[test]
//#[ignore]
fn test_second_prime() {
    assert_eq!(nth(1), 3);
}

#[test]
//#[ignore]
fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}

#[test]
//#[ignore]
fn test_big_prime() {
    assert_eq!(nth(10000), 104743);
}
