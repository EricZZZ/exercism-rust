fn main() {}

pub fn number_of_ways(corridor: String) -> i32 {
    let mut ans = 1;
    let mut count_s = 0;
    let mut last_s = 0;

    for (i, ch) in corridor.bytes().enumerate() {
        if ch == b'S' {
            count_s += 1;
            if count_s >= 3 && count_s % 2 > 0 {
                ans = ans * ((i - last_s) as i64) % 1_000_000_007;
            }
            last_s = i;
        }
    }
    if count_s == 0 || count_s % 2 > 0 {
        return 0;
    }
    ans as i32
}
