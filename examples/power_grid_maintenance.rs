use std::collections::BTreeSet;

fn main() {
    let connections = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
    let queries = vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]];
    println!("{:?}", process_queries(5, connections, queries))
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let pa = self.find(a);
        let pb = self.find(b);

        if pa == pb {
            return false;
        }

        if self.size[pa] > self.size[pb] {
            self.parent[pb] = pa;
            self.size[pa] += self.size[pb];
        } else {
            self.parent[pa] = pb;
            self.size[pb] += self.size[pa];
        }
        true
    }
}

pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = (c + 1) as usize;
    let mut uf = UnionFind::new(n);

    for e in connections {
        if e.len() == 2 {
            uf.union(e[0] as usize, e[1] as usize);
        }
    }

    let mut st: Vec<BTreeSet<i32>> = vec![BTreeSet::new(); n];

    for i in 1..=c as usize {
        let root = uf.find(i);
        st[root].insert(i as i32);
    }

    let mut ans = Vec::new();
    let mut uf_mut = uf;

    for q in queries {
        if q.len() != 2 {
            continue;
        }
        let a = q[0];
        let x = q[1];

        let root = uf_mut.find(x as usize);

        if a == 1 {
            let set = &st[root];

            if set.contains(&x) {
                ans.push(x);
            } else if let Some(first) = set.iter().next() {
                ans.push(*first);
            } else {
                ans.push(-1);
            }
        } else if a == 2 {
            st[root].remove(&x);
        }
    }

    ans
}
