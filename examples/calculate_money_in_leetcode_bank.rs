fn main() {
    println!("{}", total_money(20))
}

pub fn total_money(n: i32) -> i32 {
    let mut ans = 0;
    let mut temp = 0;
    for i in 1..=n {
        if i % 7 == 0 {
            ans += 7 + temp;
            temp += 1;
        } else {
            ans += (i / 7) + (i % 7);
        }
    }
    ans
}
