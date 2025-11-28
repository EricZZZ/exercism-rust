fn main() {}

pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![]; n];

    for e in edges {
        let x = e[0] as usize;
        let y = e[1] as usize;
        g[x].push(y);
        g[y].push(x);
    }

    fn dfs(x: usize, fa: usize, g: &[Vec<usize>], values: &[i32], k: i64, ans: &mut i32) -> i64 {
        let mut sum = values[x] as i64;
        for &y in &g[x] {
            if y != fa {
                sum += dfs(y, x, g, values, k, ans);
            }
        }
        if sum % k == 0 {
            *ans += 1;
        }
        sum
    }
    let mut ans = 0;
    dfs(0, 0, &g, &values, k as i64, &mut ans);
    ans
}
