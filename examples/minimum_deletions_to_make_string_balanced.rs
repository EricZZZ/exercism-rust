fn main() {
    println!("{}", minimum_deletions("aababbab".to_string()))
}

pub fn minimum_deletions(s: String) -> i32 {
    let chars = s.as_bytes();
    let mut del = 0;
    for c in chars {
        del += (b'b' - c) as i32;
    }
    let mut ans = del;
    for c in chars {
        del += (c - b'a') as i32 * 2 - 1;
        ans = ans.min(del);
    }
    ans
}
