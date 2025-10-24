fn main() {
    println!("{}", next_beautiful_number(2));
}

pub fn next_beautiful_number(mut n: i32) -> i32 {
    loop {
        n += 1;
        let mut cnt = [0; 10];
        let mut temp_n = n;
        while temp_n > 0 {
            let digit = (temp_n % 10) as usize;
            cnt[digit] += 1;
            temp_n /= 10;
        }
        let mut is_beautiful = true;

        let mut temp_n = n;
        while temp_n > 0 {
            let digit = (temp_n % 10) as usize;
            let count = cnt[digit];

            if count as usize != digit {
                is_beautiful = false;
                break;
            }
            temp_n /= 10;
        }
        if is_beautiful {
            return n;
        }
    }
}
