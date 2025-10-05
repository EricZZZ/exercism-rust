fn main() {
    println!(
        "{:?}",
        pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4]
        ])
    );
}

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if heights.is_empty() || heights[0].is_empty() {
        return vec![];
    }

    let n = heights.len(); // 行数
    let m = heights[0].len(); // 列数

    // P 用于记录从太平洋出发所能到达的点（即水可以流向太平洋的点）
    // A 用于记录从大西洋出发所能到达的点（即水可以流向大西洋的点）
    // 使用 bool 矩阵代替原代码中的 i32 矩阵（0/1）来表示是否可达
    let mut pacific_reachable = vec![vec![false; m]; n];
    let mut atlantic_reachable = vec![vec![false; m]; n];
    let mut ans = Vec::new();

    // 沿左右两列边界进行 DFS
    for i in 0..n {
        dfs(&heights, &mut pacific_reachable, i, 0, n, m);
        dfs(&heights, &mut atlantic_reachable, i, m - 1, n, m);
    }

    // 沿上下两行边界进行 DFS (注意边界角点在上面的循环中已经处理过)
    for j in 0..m {
        dfs(&heights, &mut pacific_reachable, 0, j, n, m);
        dfs(&heights, &mut atlantic_reachable, n - 1, j, n, m);
    }

    // 遍历所有点，找出既能流向太平洋又能流向大西洋的点
    for i in 0..n {
        for j in 0..m {
            if pacific_reachable[i][j] && atlantic_reachable[i][j] {
                ans.push(vec![i as i32, j as i32]);
            }
        }
    }

    ans
}

fn dfs(
    heights: &Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
    n: usize,
    m: usize,
) {
    // 如果一个点已经遍历过，便返回
    if visited[i][j] {
        return;
    }

    // 标记当前点已访问/可达
    visited[i][j] = true;

    // 四个方向：上、下、左、右
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (di, dj) in directions.iter() {
        // 计算下一个单元格的坐标
        let ni = i as isize + di;
        let nj = j as isize + dj;

        // 检查边界
        if ni >= 0 && ni < n as isize && nj >= 0 && nj < m as isize {
            let ni = ni as usize;
            let nj = nj as usize;

            // 检查水流条件：从 **目标点** (ni, nj) 到 **当前点** (i, j) 的高度必须非递减。
            // 这相当于水从 (i, j) 流向 (ni, nj) 时高度非递增（即水从高处流向低处或平地）。
            // 由于我们是从海洋向内陆逆向搜索，所以高度必须是 **当前点 <= 目标点**
            // 也就是 M[ni][nj] >= M[i][j]
            if heights[ni][nj] >= heights[i][j] {
                dfs(heights, visited, ni, nj, n, m);
            }
        }
    }
}
