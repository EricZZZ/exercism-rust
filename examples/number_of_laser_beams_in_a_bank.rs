fn main() {
    println!(
        "{}",
        number_of_beams(vec![
            "011001".to_string(),
            "000000".to_string(),
            "010100".to_string(),
            "001000".to_string()
        ])
    )
}

pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut ans: i32 = 0;
    let mut prev_count: i32 = 0;

    for row in bank {
        let cnt = row.chars().filter(|ch| *ch == '1').count() as i32;
        if cnt > 0 {
            if prev_count > 0 {
                ans += prev_count * cnt;
            }
            prev_count = cnt;
        }
    }

    ans
}
