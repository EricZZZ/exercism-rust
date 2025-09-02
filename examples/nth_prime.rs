fn main() {}

pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    if n == 0 {
        2
    } else {
        let mut count = 0;
        let mut candidate = 1;
        while count < n {
            candidate += 2;
            if is_prime(candidate) {
                count += 1;
            }
        }
        candidate
    }
}
// 判断一个数是否是素数，试除法
fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u32) {
        if num % i == 0 {
            return false;
        }
    }
    true
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
