fn main() {
    println!("{}", best_closing_time("YYNY".to_string()))
}

pub fn best_closing_time(customers: String) -> i32 {
    let mut sum = customers.as_bytes().iter().filter(|&&x| x == b'Y').count();
    let mut ans = 0;
    let mut min_penalty = sum;
    for (i, &c) in customers.as_bytes().iter().enumerate() {
        if c == b'Y' {
            sum -= 1;
        } else {
            sum += 1;
        }

        if sum < min_penalty {
            min_penalty = sum;
            ans = i as i32 + 1;
        }
    }
    ans
}
