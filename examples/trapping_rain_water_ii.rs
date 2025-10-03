use std::collections::BinaryHeap;

fn main() {}

pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
    let m = height_map.len();
    let n = height_map[0].len();
    let mut heap = BinaryHeap::new();
    for (i, r) in height_map.iter_mut().enumerate() {
        for (j, h) in r.iter_mut().enumerate() {
            if i == 0 || i == m - 1 || j == n - 1 || j == 0 {
                heap.push((-*h, i, j));
                *h = -1;
            }
        }
    }
    let mut ans = 0;
    while let Some((min_h, i, j)) = heap.pop() {
        let min_h = -min_h;
        for (x, y) in [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)] {
            if x < m && y < n && height_map[x][y] >= 0 {
                ans += 0.max(min_h - height_map[x][y]);
                heap.push((-min_h.max(height_map[x][y]), x, y));
                height_map[x][y] = -1;
            }
        }
    }
    ans
}
