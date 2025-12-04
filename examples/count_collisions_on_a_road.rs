fn main() {
    println!("{}", count_collisions("LLRR".to_string()))
}

pub fn count_collisions(directions: String) -> i32 {
    let n = directions.len();
    let chars = directions.as_bytes();

    let mut l = 0;
    while l < n && chars[l] == b'L' {
        l += 1;
    }

    let mut r = n;
    while r > l && chars[r - 1] == b'R' {
        r -= 1;
    }
    chars[l..r].iter().filter(|&&c| c != b'S').count() as i32
}
