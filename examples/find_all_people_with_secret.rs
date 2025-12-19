use std::collections::{HashMap, HashSet};

fn main() {
    println!(
        "{:?}",
        find_all_people(6, vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]], 1)
    )
}

// 图论，利用HashMap来构建邻接表（Adjacency List），DFS 深度优先算法
pub fn find_all_people(_n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    let mut set = HashSet::new();
    set.insert(0);
    set.insert(first_person);
    meetings.sort_by_key(|v| v[2]);

    let m = meetings.len();
    let mut i = 0;

    while i < m {
        let time = meetings[i][2];
        let mut g: HashMap<i32, Vec<i32>> = HashMap::new();

        // 找到当前时间点的所有会议并建图
        let mut j = i;
        while j < m && meetings[j][2] == time {
            let x = meetings[j][0];
            let y = meetings[j][1];
            g.entry(x).or_default().push(y);
            g.entry(y).or_default().push(x);
            j += 1;
        }

        // 4. 每个连通块只要有一个人知道秘密，DFS 扩散
        let mut vis = HashSet::new();
        // 注意：这里需要先收集 keys，或者直接遍历 g
        for &x in g.keys() {
            if set.contains(&x) && !vis.contains(&x) {
                dfs(x, &g, &mut vis, &mut set);
            }
        }

        // 移动到下一个时间点的会议
        i = j;
    }

    set.iter().cloned().collect()
}

fn dfs(x: i32, g: &HashMap<i32, Vec<i32>>, vis: &mut HashSet<i32>, have_secret: &mut HashSet<i32>) {
    vis.insert(x);
    have_secret.insert(x);
    if let Some(neighbors) = g.get(&x) {
        for &y in neighbors {
            if !vis.contains(&y) {
                dfs(y, g, vis, have_secret);
            }
        }
    }
}
