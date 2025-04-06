fn main() {}

pub fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true;
            } else {
                return false;
            }
        } else {
            return true;
        }
    } else {
        return false;
    }

    // 官方答案，写法非常优雅，使用了闭包
    // let has_factor = |n| year % n == 0;
    // has_factor(4) && (!has_factor(100) || has_factor(400))
}

#[test]
fn test_vanilla_leap_year() {
    assert_eq!(is_leap_year(1996), true);
}

#[test]
//#[ignore]
fn test_any_old_year() {
    assert_eq!(is_leap_year(1997), false);
}

#[test]
//#[ignore]
fn test_century() {
    assert_eq!(is_leap_year(1700), false);
    assert_eq!(is_leap_year(1800), false);
    assert_eq!(is_leap_year(1900), false);
}

#[test]
//#[ignore]
fn test_exceptional_centuries() {
    assert_eq!(is_leap_year(1600), true);
    assert_eq!(is_leap_year(2000), true);
    assert_eq!(is_leap_year(2400), true);
}

#[test]
//#[ignore]
fn test_years_1600_to_1699() {
    let incorrect_years = (1600..1700)
        .filter(|&year| is_leap_year(year) != (year % 4 == 0))
        .collect::<Vec<_>>();

    if !incorrect_years.is_empty() {
        panic!("incorrect result for years: {:?}", incorrect_years);
    }
}
