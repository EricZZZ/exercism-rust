fn main() {}

pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let mut sum: u32 = 0;
    let mut temp: u32 = num;
    let digit: u32 = num.to_string().len() as u32;
    while temp > 0 {
        sum += (temp % 10).pow(digit);
        temp /= 10;
    }
    sum == num
}

#[test]
fn test_single_digit_numbers_are_armstrong_numbers() {
    assert!(is_armstrong_number(5))
}

#[test]
//#[ignore]
fn test_there_are_no_2_digit_armstrong_numbers() {
    assert!(!is_armstrong_number(10))
}

#[test]
//#[ignore]
fn test_three_digit_armstrong_number() {
    assert!(is_armstrong_number(153))
}

#[test]
//#[ignore]
fn test_three_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(100))
}

#[test]
//#[ignore]
fn test_four_digit_armstrong_number() {
    assert!(is_armstrong_number(9474))
}

#[test]
//#[ignore]
fn test_four_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(9475))
}

#[test]
//#[ignore]
fn test_seven_digit_armstrong_number() {
    assert!(is_armstrong_number(9926315))
}

#[test]
//#[ignore]
fn test_seven_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(9926316))
}
