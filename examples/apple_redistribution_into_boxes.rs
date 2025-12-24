fn main() {
    println!("{}", minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]))
}

pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
    let mut sum = apple.iter().sum::<i32>();
    capacity.sort_by_key(|a| -a);

    for (i, cap) in capacity.iter().enumerate() {
        if sum <= 0 {
            return i as i32;
        } else {
            sum -= cap;
        }
    }
    capacity.len() as i32
}
