fn main() {
    println!("{}", has_same_digits("3902".to_string()))
}

pub fn has_same_digits(s: String) -> bool {
    let mut chars = s.chars().collect::<Vec<char>>();
    while chars.len() > 2 {
        let mut temp_str = Vec::new();
        for i in 1..chars.len() {
            let x = chars[i - 1].to_digit(10).unwrap();
            let y = chars[i].to_digit(10).unwrap();
            let digit = (x + y) % 10;
            // convert the numeric digit (0..9) to the corresponding char
            temp_str.push(std::char::from_digit(digit, 10).unwrap());
        }
        chars = temp_str;
    }
    chars[0] == chars[1]
}
