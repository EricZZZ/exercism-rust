use std::collections::HashMap;

fn main() {
    println!("{}", fraction_to_decimal(4, 333));
}

// 长除法
pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    let mut numerator = numerator as i64;
    let mut denominator = denominator as i64;

    let sign = if numerator * denominator < 0 { "-" } else { "" };
    numerator = numerator.abs();
    denominator = denominator.abs();

    let mut quotient = numerator / denominator;
    let mut remainder = numerator % denominator;
    if remainder == 0 {
        return format!("{}{}", sign, quotient);
    }

    let mut ans = format!("{}{}.", sign, quotient);
    let mut pos_map = HashMap::new();
    pos_map.insert(remainder, ans.len());

    while remainder != 0 {
        remainder *= 10;
        quotient = remainder / denominator;
        remainder %= denominator;
        ans.push((b'0' + quotient as u8) as char);
        if let Some(&pos) = pos_map.get(&remainder) {
            return format!("{}({})", &ans[..pos], &ans[pos..]);
        }
        pos_map.insert(remainder, ans.len());
    }

    ans
}
