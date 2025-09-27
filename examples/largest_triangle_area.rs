fn main() {
    println!(
        "{}",
        largest_triangle_area(vec![vec![4, 6], vec![6, 5], vec![3, 1],])
    )
}

pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let mut ans: f64 = 0.0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for k in j + 1..points.len() {
                ans = f64::max(
                    ans,
                    shoelace_formula(
                        points[i][0],
                        points[i][1],
                        points[j][0],
                        points[j][1],
                        points[k][0],
                        points[k][1],
                    ),
                )
            }
        }
    }
    ans
}

fn shoelace_formula(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> f64 {
    0.5 * (x1 * y2 + x2 * y3 + x3 * y1 - x1 * y3 - x2 * y1 - x3 * y2).abs() as f64
}
