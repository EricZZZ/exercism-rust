fn main() {
    println!(
        "{:?}",
        minimum_abs_difference(vec![
            -17, 46, 63, 81, -101, -91, 121, -2, 112, -15, -65, -96, 6, -139
        ])
    )
}

pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    arr.sort_unstable();
    let min = arr.windows(2).map(|w| w[1] - w[0]).min().unwrap();
    for i in 0..arr.len() - 1 {
        if min == arr[i + 1] - arr[i] {
            ans.push(vec![arr[i], arr[i + 1]]);
        }
    }
    ans
}
