fn main() {
    println!("{}", smallest_repunit_div_by_k(3))
}

pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    if k % 2 == 0 || k % 5 == 0 {
        return -1;
    }
    let mut ans = 1;
    let mut r = 1 % k;
    while r != 0 {
        r = (r * 10 + 1) % k;
        println!("n:{}", r);
        ans += 1;
    }
    ans
}
