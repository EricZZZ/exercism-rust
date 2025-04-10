fn main() {}

// 这种也能答对，但是写法丑陋
// pub fn collatz(n: u64) -> Option<u64> {
//     let mut count = 0;
//     let mut temp = n;
//     match n {
//         0 => None,
//         1 => Some(0),
//         _ => {
//             while temp != 1 {
//                 if temp % 2 == 0 {
//                     temp /= 2;
//                 } else {
//                     temp *= 3;
//                     temp += 1;
//                 }

//                 count += 1;
//             }
//             Some(count)
//         }
//     }
// }

pub fn collatz_positive(n: u64) -> u64 {
    if n == 1 {
        0
    } else {
        1 + match n % 2 {
            0 => collatz_positive(n / 2),
            _ => collatz_positive(n * 3 + 1),
        }
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        None
    } else {
        Some(collatz_positive(n))
    }
}

#[test]
fn test_1() {
    assert_eq!(Some(0), collatz(1));
}

#[test]
//#[ignore]
fn test_16() {
    assert_eq!(Some(4), collatz(16));
}

#[test]
//#[ignore]
fn test_12() {
    assert_eq!(Some(9), collatz(12));
}

#[test]
//#[ignore]
fn test_1000000() {
    assert_eq!(Some(152), collatz(1000000));
}

#[test]
//#[ignore]
fn test_0() {
    assert_eq!(None, collatz(0));
}
