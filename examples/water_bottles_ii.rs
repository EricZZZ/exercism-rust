fn main() {
    println!("{}", max_bottles_drunk(10, 3));
}

pub fn max_bottles_drunk(num_bottles: i32, mut num_exchange: i32) -> i32 {
    let mut bottles_drunk = num_bottles;
    let mut empty_bottles = num_bottles;
    while empty_bottles >= num_exchange {
        empty_bottles -= num_exchange;
        empty_bottles += 1;
        num_exchange += 1;
        bottles_drunk += 1;
    }
    bottles_drunk
}
