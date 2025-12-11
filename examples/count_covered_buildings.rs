use std::collections::HashMap;

fn main() {
    println!(
        "{}",
        count_covered_buildings(
            4,
            vec![
                vec![2, 4],
                vec![1, 2],
                vec![3, 1],
                vec![1, 4],
                vec![2, 3],
                vec![3, 3],
                vec![2, 2],
                vec![1, 3]
            ]
        )
    )
}

pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
    // 使用更紧凑的数据结构，避免Vec<Vec<i32>>
    // 将建筑物的坐标转换为元组数组
    let building_coords: Vec<(i32, i32)> = buildings.into_iter().map(|b| (b[0], b[1])).collect();

    // 使用一次遍历同时构建两个map和计算结果
    let mut x_map: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut y_map: HashMap<i32, (i32, i32)> = HashMap::new();

    // 第一遍：构建map
    for &(x, y) in &building_coords {
        if x <= n {
            let entry = x_map.entry(x).or_insert((y, y));
            entry.0 = entry.0.min(y);
            entry.1 = entry.1.max(y);
        }

        if y <= n {
            let entry = y_map.entry(y).or_insert((x, x));
            entry.0 = entry.0.min(x);
            entry.1 = entry.1.max(x);
        }
    }

    // 第二遍：计算结果
    let count = building_coords
        .iter()
        .filter(|&&(x, y)| {
            if let (Some(x_range), Some(y_range)) = (x_map.get(&x), y_map.get(&y)) {
                x_range.0 < y && y < x_range.1 && y_range.0 < x && x < y_range.1
            } else {
                false
            }
        })
        .count();

    count as i32
}
