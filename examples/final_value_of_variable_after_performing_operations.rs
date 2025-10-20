fn main() {}
pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut ans = 0;
    for s in operations {
        if s.contains('+') {
            ans += 1;
        }
        if s.contains('-') {
            ans -= 1
        }
    }
    ans
}
