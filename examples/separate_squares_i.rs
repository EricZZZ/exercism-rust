fn main() {
    println!("{}", separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]]))
}

pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
    let mut total_area = 0_f64;
    let mut max_y = 0_f64;
    for sq in &squares {
        let y = sq[1] as f64;
        let l = sq[2] as f64;
        total_area += l * l;
        max_y = max_y.max(y + l);
    }

    fn check(squares: &Vec<Vec<i32>>, y: f64, total_area: f64) -> bool {
        let mut area = 0.0;
        for sq in squares {
            let yi = sq[1] as f64;
            let l = sq[2] as f64;
            if yi < y {
                area += l * (y - yi).min(l);
            }
        }
        area >= total_area / 2.0
    }

    let mut left = 0.0;
    let mut right = max_y;
    for _ in 0..47 {
        let mid = (left + right) / 2.0;
        if check(&squares, mid, total_area) {
            right = mid;
        } else {
            left = mid;
        }
    }

    (left + right) / 2.0
}
