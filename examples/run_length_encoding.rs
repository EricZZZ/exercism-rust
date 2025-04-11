fn main() {}

pub fn encode(source: &str) -> String {
    let mut res = String::new();
    if source.len() < 2 {
        return source.to_string();
    }
    let bytes: &[u8] = source.as_bytes();
    let mut count: u8 = 1;
    let mut per_ch = bytes[0];
    for ch in bytes.iter().take(source.len()).skip(1) {
        if per_ch == *ch {
            count += 1;
        } else {
            if count > 1 {
                res = format!("{}{}{}", res, count, per_ch as char);
            } else {
                res = format!("{}{}", res, per_ch as char);
            }
            per_ch = *ch;
            count = 1;
        }
    }
    if count > 1 {
        res = format!("{}{}{}", res, count, per_ch as char);
    } else {
        res = format!("{}{}", res, per_ch as char);
    }
    res
}

pub fn decode(source: &str) -> String {
    let mut res = String::new();
    let mut count_str = String::new();
    for ch in source.chars() {
        if ch.is_numeric() {
            println!("ch is num:{}", ch);
            count_str = format!("{}{}", count_str, ch)
        } else {
            let mut count: u32 = count_str.parse().unwrap_or(1);
            while count != 0 {
                res.push(ch);
                count -= 1;
            }
            count_str = "".to_string();
        }
    }
    res
}

/**
 * 自己实现的方法，只能按部就班
 * 答案用到了函数式，闭包，非常优雅，fold用的出神入化了啊😱
use std::cmp;

pub fn encode(input: &str) -> String {
   input
       .chars()
       .fold(
           (String::new(), ' ', 0, 1),
           |(mut acc, last, last_n, pos), c| {
               // acc = where answer is accumulated
               // last = last character read
               // last_n = accum count for last
               if c == last {
                   if pos == input.len() {
                       // end of string
                       acc += (last_n + 1).to_string().as_str();
                       acc.push(c);
                   }
                   (acc, last, last_n + 1, pos + 1)
               } else {
                   if last_n > 1 {
                       acc += last_n.to_string().as_str();
                   }
                   if last_n > 0 {
                       // ignore initial last (single whitespace)
                       acc.push(last);
                   }
                   if pos == input.len() {
                       // end of string
                       acc.push(c);
                   }
                   (acc, c, 1, pos + 1)
               }
           },
       )
       .0
}

pub fn decode(input: &str) -> String {
   input
       .chars()
       .fold((String::new(), 0), |(mut acc, last_n), c| {
           if let Some(d) = c.to_digit(10) {
               (acc, 10 * last_n + d)
           } else {
               acc += c.to_string().repeat(cmp::max(last_n, 1) as usize).as_str();
               (acc, 0)
           }
       })
       .0
}
 */

// encoding tests
#[test]
fn test_encode_empty_string() {
    assert_eq!("", encode(""));
}

#[test]
//#[ignore]
fn test_encode_single_characters() {
    assert_eq!("XYZ", encode("XYZ"));
}

#[test]
//#[ignore]
fn test_encode_string_with_no_single_characters() {
    assert_eq!("2A3B4C", encode("AABBBCCCC"));
}

#[test]
//#[ignore]
fn test_encode_single_characters_mixed_with_repeated_characters() {
    assert_eq!(
        "12WB12W3B24WB",
        encode("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB")
    );
}

#[test]
//#[ignore]
fn test_encode_multiple_whitespace_mixed_in_string() {
    assert_eq!("2 hs2q q2w2 ", encode("  hsqq qww  "));
}

#[test]
//#[ignore]
fn test_encode_lowercase_characters() {
    assert_eq!("2a3b4c", encode("aabbbcccc"));
}

// decoding tests

#[test]
//#[ignore]
fn test_decode_empty_string() {
    assert_eq!("", decode(""));
}

#[test]
//#[ignore]
fn test_decode_single_characters_only() {
    assert_eq!("XYZ", decode("XYZ"));
}

#[test]
//#[ignore]
fn test_decode_string_with_no_single_characters() {
    assert_eq!("AABBBCCCC", decode("2A3B4C"));
}

#[test]
//#[ignore]
fn test_decode_single_characters_with_repeated_characters() {
    assert_eq!(
        "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB",
        decode("12WB12W3B24WB")
    );
}

#[test]
//#[ignore]
fn test_decode_multiple_whitespace_mixed_in_string() {
    assert_eq!("  hsqq qww  ", decode("2 hs2q q2w2 "));
}

#[test]
//#[ignore]
fn test_decode_lower_case_string() {
    assert_eq!("aabbbcccc", decode("2a3b4c"));
}

// consistency test

#[test]
//#[ignore]
fn test_consistency() {
    assert_eq!("zzz ZZ  zZ", decode(encode("zzz ZZ  zZ").as_str()));
}
