fn main() {
    println!("{}", is_one_bit_character(vec![1, 1, 0]))
}
pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let mut last = 0;
    let mut i = 0;
    while i < bits.len() {
        if bits[i] == 1 && i + 1 < bits.len() {
            last = 1;
            i += 2;
        } else {
            last = 0;
            i += 1;
        }
    }
    last == 0
}
