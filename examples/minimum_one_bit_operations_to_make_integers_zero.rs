fn main() {}

pub fn minimum_one_bit_operations(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let x = 32 - n.leading_zeros();
    (1 << x) - 1 - minimum_one_bit_operations(n - (1 << (x - 1)))
}
