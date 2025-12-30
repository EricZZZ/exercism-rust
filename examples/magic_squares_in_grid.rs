fn main() {
    println!(
        "{}",
        num_magic_squares_inside(vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]])
    )
}

pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let r = grid.len();
    let c = grid[0].len();
    if r < 3 || c < 3 {
        return 0;
    }
    let mut ans = 0;

    fn is_magic_square(
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        g: i32,
        h: i32,
        i: i32,
    ) -> bool {
        let vals = [a, b, c, d, e, f, g, h, i];
        let mut frequency = [0; 16];
        for &value in vals.iter() {
            if value < 1 || value > 9 {
                return false;
            }
            frequency[value as usize] += 1;
        }

        for num in 1..=9 {
            if frequency[num] != 1 {
                return false;
            }
        }

        a + b + c == 15 && // 第一行
            d + e + f == 15 && // 第二行
            g + h + i == 15 && // 第三行
            a + d + g == 15 && // 第一列
            b + e + h == 15 && // 第二列
            c + f + i == 15 && // 第三列
            a + e + i == 15 && // 主对角线
            c + e + g == 15 // 副对角线
    }

    for x in 0..r - 2 {
        for y in 0..c - 2 {
            if grid[x + 1][y + 1] != 5 {
                continue;
            }
            if is_magic_square(
                grid[x][y],
                grid[x][y + 1],
                grid[x][y + 2],
                grid[x + 1][y],
                grid[x + 1][y + 1],
                grid[x + 1][y + 2],
                grid[x + 2][y],
                grid[x + 2][y + 1],
                grid[x + 2][y + 2],
            ) {
                ans += 1;
            }
        }
    }
    ans
}
