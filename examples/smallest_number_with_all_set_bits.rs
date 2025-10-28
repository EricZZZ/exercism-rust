fn main() {
    println!("{}", smallest_number(10))
}

pub fn smallest_number(n: i32) -> i32 {
    2_i32.pow(format!("{:b}", n).len() as u32) - 1
}
