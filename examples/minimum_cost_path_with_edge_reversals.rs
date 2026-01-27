use std::collections::BinaryHeap;

fn main() {
    println!(
        "{}",
        min_cost(
            4,
            vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]]
        )
    )
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    cost: i32,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
// Dijkstra算法
pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for e in edges {
        let x = e[0] as usize;
        let y = e[1] as usize;
        let wt = e[2];
        g[x].push((y, wt));
        g[y].push((x, wt * 2));
    }
    let mut dis = vec![i32::MAX; n];
    let mut pq = BinaryHeap::new();

    dis[0] = 0;
    pq.push(State { cost: 0, node: 0 });

    while let Some(State { cost, node }) = pq.pop() {
        if cost > dis[node] {
            continue;
        }

        if node == n - 1 {
            return cost;
        }

        for &(next_node, weight) in &g[node] {
            let next_cost = cost + weight;
            if next_cost < dis[next_node] {
                dis[next_node] = next_cost;
                pq.push(State {
                    cost: next_cost,
                    node: next_node,
                });
            }
        }
    }
    -1
}
