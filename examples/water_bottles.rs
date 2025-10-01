fn main() {
    println!("{}", num_water_bottles(15, 4));
}

pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
    let mut ans = num_bottles;
    while num_bottles >= num_exchange {
        ans -= num_bottles % num_exchange;
        num_bottles = num_bottles / num_exchange + num_bottles % num_exchange;
        ans += num_bottles;
    }
    ans
}
