fn main() {
    println!("{}", swim_in_water(vec![vec![0, 2], vec![1, 3]]))
}

const DIRS: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

/**
 * 二分查找，深度优先搜素DFS
 */
pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    let mut left = 0;
    let mut right = grid.len() * grid.len() - 1;
    let n = grid[0].len();
    while left < right {
        let mid = (left + right) / 2;
        let mut visited = vec![vec![false; n]; n];
        if grid[0][0] <= mid as i32 && dfs(&grid, 0, 0, &mut visited, mid as i32) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as i32
}

fn dfs(grid: &Vec<Vec<i32>>, x: i32, y: i32, visited: &mut Vec<Vec<bool>>, mid: i32) -> bool {
    visited[x as usize][y as usize] = true;
    for dir in DIRS {
        let new_x = x + dir[0];
        let new_y = y + dir[1];
        if in_area(&new_x, &new_y, &grid.len())
            && !visited[new_x as usize][new_y as usize]
            && grid[new_x as usize][new_y as usize] <= mid
        {
            if new_x == (grid.len() - 1) as i32 && new_y == (grid.len() - 1) as i32 {
                return true;
            }
            if dfs(grid, new_x, new_y, visited, mid) {
                return true;
            }
        }
    }
    false
}

fn in_area(x: &i32, y: &i32, n: &usize) -> bool {
    *x >= 0 && *x < *n as i32 && *y >= 0 && *y < *n as i32
}
