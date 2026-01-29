fn main() {}

pub fn minimum_cost(
    source: String,
    target: String,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>,
) -> i64 {
    const INF: i32 = i32::MAX / 2;
    // 初始化 26x26 的距离矩阵
    let mut dis = [[INF; 26]; 26];

    for i in 0..26 {
        dis[i][i] = 0;
    }

    // 填充初始边（注意：可能存在重复边，取最小值）
    for i in 0..original.len() {
        let x = (original[i] as u8 - b'a') as usize;
        let y = (changed[i] as u8 - b'a') as usize;
        dis[x][y] = dis[x][y].min(cost[i]);
    }

    // Floyd-Warshall 算法计算所有字符对之间的最短路
    for k in 0..26 {
        for i in 0..26 {
            if dis[i][k] == INF {
                continue;
            }
            for j in 0..26 {
                dis[i][j] = dis[i][j].min(dis[i][k] + dis[k][j]);
            }
        }
    }

    // 计算总开销
    let mut ans: i64 = 0;
    let source_bytes = source.as_bytes();
    let target_bytes = target.as_bytes();

    for i in 0..source_bytes.len() {
        if source_bytes[i] != target_bytes[i] {
            let x = (source_bytes[i] - b'a') as usize;
            let y = (target_bytes[i] - b'a') as usize;
            let d = dis[x][y];
            if d == INF {
                return -1;
            }
            ans += d as i64;
        }
    }

    ans
}
