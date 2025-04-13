fn main() {}

#[derive(PartialEq, Eq)]
enum IsbnType {
    Isbn10,
    Isbn13,
}

fn is_x_valid(index: &usize, isbn_type: &IsbnType) -> bool {
    (isbn_type == &IsbnType::Isbn10 && index == &9)
        || (isbn_type == &IsbnType::Isbn13 && index == &12)
}

fn is_dash_valid(index: &usize, isbn_type: &IsbnType) -> bool {
    isbn_type == &IsbnType::Isbn13 && (index == &1 || index == &5 || index == &11)
}

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn_type = match isbn.len() {
        10 => IsbnType::Isbn10,
        13 => IsbnType::Isbn13,
        _ => return false,
    };

    let mut sum = 0;
    let mut coefficient = 10;
    for (index, c) in isbn.char_indices() {
        let digit_value = match c {
            '0'..='9' => c.to_digit(10).unwrap(),
            'X' if is_x_valid(&index, &isbn_type) => 10,
            '-' if is_dash_valid(&index, &isbn_type) => continue,
            _ => return false,
        };
        sum += coefficient * digit_value;
        coefficient -= 1;
    }
    sum % 11 == 0
}

#[test]
fn test_valid() {
    assert!(is_valid_isbn("3-598-21508-8"));
}

#[test]
//#[ignore]
fn test_invalid_check_digit() {
    assert!(!is_valid_isbn("3-598-21508-9"));
}

#[test]
//#[ignore]
fn test_valid_check_digit_of_10() {
    assert!(is_valid_isbn("3-598-21507-X"));
}

#[test]
//#[ignore]
fn test_invalid_character_as_check_digit() {
    assert!(!is_valid_isbn("3-598-21507-A"));
}

#[test]
//#[ignore]
fn test_invalid_character_in_isbn() {
    assert!(!is_valid_isbn("3-598-2K507-0"));
}

#[test]
//#[ignore]
#[allow(non_snake_case)]
fn test_invalid_isbn_with_invalid_X() {
    assert!(!is_valid_isbn("3-598-2X507-9"));
}

#[test]
//#[ignore]
fn test_valid_isbn_without_dashes() {
    assert!(is_valid_isbn("3598215088"));
}

#[test]
//#[ignore]
#[allow(non_snake_case)]
fn test_valid_isbn_without_dashes_and_X_as_check() {
    assert!(is_valid_isbn("359821507X"));
}

#[test]
//#[ignore]
fn test_invalid_isbn_without_dashes_and_no_check_digit() {
    assert!(!is_valid_isbn("359821507"));
}

#[test]
//#[ignore]
fn test_invalid_isbn_without_dashes_and_too_long() {
    assert!(!is_valid_isbn("3598215078X"));
}

#[test]
//#[ignore]
fn test_invalid_isbn_without_check_digit() {
    assert!(!is_valid_isbn("3-598-21507"));
}

#[test]
//#[ignore]
fn test_invalid_isbn_too_long() {
    assert!(!is_valid_isbn("3-598-21507-XX"));
}

#[test]
//#[ignore]
fn test_valid_digits_invalid_length() {
    assert!(!is_valid_isbn("35982150881"));
}

#[test]
//#[ignore]
fn test_special_characters() {
    assert!(!is_valid_isbn("!@#%!@"));
}

#[test]
//#[ignore]
#[allow(non_snake_case)]
fn test_invalid_isbn_with_check_digit_X_instead_of_0() {
    assert!(!is_valid_isbn("3-598-21515-X"));
}
