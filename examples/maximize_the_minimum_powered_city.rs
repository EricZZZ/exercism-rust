fn main() {
    println!("{:?}", max_power(vec![4, 4, 4, 4], 0, 3))
}

// 二分，差分，前缀合
pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
    let n = stations.len();

    let mut sum: Vec<i64> = vec![0_i64; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + stations[i] as i64;
    }

    let mut power: Vec<i64> = vec![0_i64; n];
    let mut max = i64::MAX;
    for i in 0..n {
        power[i] = sum[(i + r as usize + 1).min(n)] - sum[0.max(i as i32 - r) as usize];
        max = max.min(power[i]);
    }

    fn check(mid: &i64, power: &[i64], r: i32, k: i32) -> bool {
        let n = power.len();
        let mut diff: Vec<i64> = vec![0_i64; n + 1];
        let mut sum_d = 0_i64;
        let mut built = 0_i64;
        for i in 0..n {
            sum_d += diff[i];
            let m = mid - (power[i] + sum_d);
            if m <= 0 {
                continue;
            }
            built += m;
            if built > k as i64 {
                return false;
            }
            sum_d += m;
            diff[(i + r as usize * 2 + 1).min(n)] -= m;
        }
        true
    }

    let mut left = max + k as i64 / n as i64;
    let mut right = max + k as i64 + 1;
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(&mid, &power, r, k) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}
