fn main() {
    println!(
        "{}",
        count_unguarded(
            4,
            6,
            vec![vec![0, 0], vec![1, 1], vec![2, 3]],
            vec![vec![0, 1], vec![2, 2], vec![1, 4]]
        )
    )
}

pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let mut gird = vec![vec![0_i32; n as usize]; m as usize];
    for g in &guards {
        gird[g[0] as usize][g[1] as usize] = -1;
    }
    for w in walls {
        gird[w[0] as usize][w[1] as usize] = -1;
    }
    fn check(x: i32, y: i32, dir_x: i32, dir_y: i32, gird: &mut Vec<Vec<i32>>) {
        let mut x = (x + dir_x) as usize;
        let mut y = (y + dir_y) as usize;
        while x < gird.len() && y < gird[0].len() && gird[x][y] != -1 {
            gird[x][y] = 1;
            x = (x as i32 + dir_x) as usize;
            y = (y as i32 + dir_y) as usize;
        }
    }
    for g in guards {
        let i = g[0];
        let j = g[1];

        // 遇到守卫，横，竖
        // 左，右，下，上
        check(i, j, -1, 0, &mut gird);
        check(i, j, 1, 0, &mut gird);
        check(i, j, 0, -1, &mut gird);
        check(i, j, 0, 1, &mut gird);
    }
    gird.into_iter().flatten().filter(|&x| x == 0).count() as i32
}
