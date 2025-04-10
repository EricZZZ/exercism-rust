fn main() {}

pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec!["".to_string(); digits.len() + 1],
        _ => digits
            .chars()
            .collect::<Vec<char>>()
            .windows(len)
            .map(|window| window.iter().collect::<String>())
            .collect::<Vec<String>>(),
    }
}

#[test]
fn test_with_zero_length() {
    let expected = vec!["".to_string(); 6];
    assert_eq!(series("92017", 0), expected);
}

#[test]
//#[ignore]
fn test_with_length_2() {
    let expected = vec![
        "92".to_string(),
        "20".to_string(),
        "01".to_string(),
        "17".to_string(),
    ];
    assert_eq!(series("92017", 2), expected);
}

#[test]
//#[ignore]
fn test_with_numbers_length() {
    let expected = vec!["92017".to_string()];
    assert_eq!(series("92017", 5), expected);
}

#[test]
//#[ignore]
fn test_too_long() {
    let expected: Vec<String> = vec![];
    assert_eq!(series("92017", 6), expected);
}
