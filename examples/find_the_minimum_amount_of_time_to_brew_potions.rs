fn main() {
    println!("{}", min_time(vec![1, 5, 2, 4], vec![5, 1, 4, 2]))
}

pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
    let n = skill.len();
    let mut last_finish = vec![0_i64; n];
    for m in mana {
        let mut sum = 0;
        for i in 0..n {
            sum = sum.max(last_finish[i]) + (skill[i] * m) as i64;
        }
        last_finish[n - 1] = sum;
        for i in (0..n - 1).rev() {
            last_finish[i] = last_finish[i + 1] - (skill[i + 1] * m) as i64;
        }
    }
    last_finish[n - 1]
}
