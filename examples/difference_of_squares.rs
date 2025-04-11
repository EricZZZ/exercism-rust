fn main() {}

pub fn square_of_sum(n: u32) -> u32 {
    let res = n * (n + 1) / 2;
    res * res
}

pub fn sum_of_squares(n: u32) -> u32 {
    let square = |n| n * n;
    (0..n + 1).map(square).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

#[test]
fn test_square_of_sum_1() {
    assert_eq!(1, square_of_sum(1));
}

#[test]
//#[ignore]
fn test_square_of_sum_5() {
    assert_eq!(225, square_of_sum(5));
}

#[test]
//#[ignore]
fn test_square_of_sum_100() {
    assert_eq!(25502500, square_of_sum(100));
}

#[test]
//#[ignore]
fn test_sum_of_squares_1() {
    assert_eq!(1, sum_of_squares(1));
}

#[test]
//#[ignore]
fn test_sum_of_squares_5() {
    assert_eq!(55, sum_of_squares(5));
}

#[test]
//#[ignore]
fn test_sum_of_squares_100() {
    assert_eq!(338350, sum_of_squares(100));
}

#[test]
//#[ignore]
fn test_difference_1() {
    assert_eq!(0, difference(1));
}

#[test]
//#[ignore]
fn test_difference_5() {
    assert_eq!(170, difference(5));
}

#[test]
//#[ignore]
fn test_difference_100() {
    assert_eq!(25164150, difference(100));
}
