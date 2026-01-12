use std::cmp;

fn main() {
    println!(
        "{}",
        min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]])
    )
}

pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut pre_x = points[0][0];
    let mut pre_y = points[0][1];
    for point in points.iter().skip(1) {
        let cur_x = point[0];
        let cur_y = point[1];

        ans += cmp::max((pre_x - cur_x).abs(), (pre_y - cur_y).abs());

        pre_x = cur_x;
        pre_y = cur_y;
    }
    ans
}
