fn main() {
    println!("{}", maximum_energy(vec![-2, -3, -1], 2))
}

pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
    let mut max = i32::MIN;
    let n = energy.len();
    for i in n - k as usize..n {
        let mut temp = 0;
        for j in (0..=i).rev().step_by(k as usize) {
            println!("i:{},j{}", i, j);
            temp += energy[j];
            max = max.max(temp);
        }
    }

    max
}
